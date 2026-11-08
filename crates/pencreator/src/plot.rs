//! From the shapes, the layers and the device to the moves of the pen, and from the moves to G-code.
//!
//! The moves are in the coordinates of the work area: X to the right, Y away from you (the drawing is turned over, as
//! the scene counts Y downwards).
use crate::device::{Device, Lift, Origin};
use crate::layers::{PenLayer, PictureMethod};
use crate::pens::PenLibrary;
use crate::warn::Warn;
use base_core::geom::{Polyline, Pt};
use base_core::image::ImageData;
use base_core::raster;
use base_core::scene::{Kind, Scene, Shape};

pub type P2 = [f64; 2];

/// The most fill lines one layer may make: a guard against a gap that is far too small for the shape.
const MAX_HATCH_LINES: usize = 200_000;
/// Up to this many strokes the order of the drawing is improved to shorten the moves with the pen up.
const MAX_ORDERED: usize = 6000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MoveKind {
    /// A move with the pen up.
    Travel,
    /// A move with the pen on the paper.
    Draw,
}

#[derive(Clone, Debug)]
pub struct Move {
    pub kind: MoveKind,
    pub from: P2,
    pub to: P2,
    pub layer: usize,
}

impl Move {
    pub fn length(&self) -> f64 {
        ((self.to[0] - self.from[0]).powi(2) + (self.to[1] - self.from[1]).powi(2)).sqrt()
    }
}

/// A comment of the G-code, written before the move `at` (or after the last move when `at` is their number).
#[derive(Clone, Debug, PartialEq)]
pub struct Note {
    pub at: usize,
    pub text: String,
    /// Written before the comment and the commands of a new layer that begins at the same move (an end of something, or
    /// the start of an element), not after them.
    pub before_layer: bool,
}

#[derive(Clone, Debug, Default)]
pub struct Program {
    pub moves: Vec<Move>,
    /// The comments of the G-code: where every element, layer, fill and line starts and ends.
    pub notes: Vec<Note>,
    /// What could not be done, or looks wrong, in words for the user.
    pub warnings: Vec<Warn>,
    /// With a drag knife: the path its tip cuts, for each cut (the moves are the path of the axis), by layer.
    pub knife_tips: Vec<(usize, Vec<P2>)>,
}

impl Program {
    pub fn is_empty(&self) -> bool {
        !self.moves.iter().any(|m| m.kind == MoveKind::Draw)
    }

    /// The box of the lines (min x, min y, max x, max y), if anything is drawn.
    pub fn bounds(&self) -> Option<[f64; 4]> {
        let mut b: Option<[f64; 4]> = None;
        for m in self.moves.iter().filter(|m| m.kind == MoveKind::Draw) {
            for p in [m.from, m.to] {
                let r = b.get_or_insert([p[0], p[1], p[0], p[1]]);
                *r = [r[0].min(p[0]), r[1].min(p[1]), r[2].max(p[0]), r[3].max(p[1])];
            }
        }
        b
    }

    /// The length of the line the pen draws, in mm.
    pub fn draw_length(&self) -> f64 {
        self.moves.iter().filter(|m| m.kind == MoveKind::Draw).map(|m| m.length()).sum()
    }

    pub fn travel_length(&self) -> f64 {
        self.moves.iter().filter(|m| m.kind == MoveKind::Travel).map(|m| m.length()).sum()
    }

    /// How many times the pen goes up between lines.
    pub fn lifts(&self) -> usize {
        self.moves.windows(2).filter(|w| w[0].kind == MoveKind::Draw && w[1].kind == MoveKind::Travel).count()
    }

    /// The time the plotter needs, in seconds: the lines at the speed of their layers, the moves at the speed of the
    /// device and the wait after every move of the pen.
    pub fn estimate_seconds(&self, dev: &Device, layers: &[PenLayer]) -> f64 {
        let moves: f64 = self
            .moves
            .iter()
            .map(|m| {
                let speed = if m.kind == MoveKind::Travel { dev.rapid } else { layers.get(m.layer).map_or(1000.0, |l| l.speed) };
                if speed > 0.0 {
                    m.length() / speed * 60.0
                } else {
                    0.0
                }
            })
            .sum();
        moves + (self.lifts() as f64 * 2.0 + 1.0) * dev.pen.delay_ms / 1000.0
    }
}

/// One line the pen draws without lifting.
struct Stroke {
    pts: Vec<P2>,
    closed: bool,
}

/// The outlines of the shapes of a layer in the coordinates of the work area. Pictures are not outlines: they are counted
/// (`count_pictures`: a picture layer draws them itself, so it does not ask for them to be counted as lost).
fn layer_outlines(scene: &Scene, layer: usize, area_y: f64, images: &mut usize, count_pictures: bool) -> Vec<Polyline> {
    let mut out = vec![];
    for s in scene.shapes.iter().filter(|s| s.layer == layer) {
        if matches!(s.kind, Kind::Image(_)) {
            if count_pictures {
                *images += 1;
            }
            continue;
        }
        for p in s.polys() {
            if p.pts.len() < 2 {
                continue;
            }
            let pts = p.pts.iter().map(|q| Pt::new(q.x, area_y - q.y)).collect();
            out.push(Polyline::new(pts, p.closed));
        }
    }
    out
}

/// Parallel lines `gap` apart at `angle` degrees, inside the closed rings (a point is inside when it is within an odd
/// number of rings, so holes stay empty). Every second line is turned round so that the pen zigzags.
fn hatch(rings: &[Polyline], angle: f64, gap: f64, budget: &mut usize) -> Vec<Stroke> {
    let rings: Vec<&Polyline> = rings.iter().filter(|r| r.closed && r.pts.len() >= 3).collect();
    if rings.is_empty() || !(gap > 0.0) {
        return vec![];
    }
    let (s, c) = angle.to_radians().sin_cos();
    // Turn the picture so that the lines are horizontal.
    let fwd = |p: &Pt| (p.x * c + p.y * s, -p.x * s + p.y * c);
    let back = |x: f64, y: f64| [x * c - y * s, x * s + y * c];
    let turned: Vec<Vec<(f64, f64)>> = rings.iter().map(|r| r.pts.iter().map(fwd).collect()).collect();
    let (mut lo, mut hi) = (f64::INFINITY, f64::NEG_INFINITY);
    for r in &turned {
        for &(_, y) in r {
            lo = lo.min(y);
            hi = hi.max(y);
        }
    }
    let mut out = vec![];
    let mut k = 0usize;
    let mut y = lo + gap / 2.0;
    while y < hi {
        if *budget == 0 {
            break;
        }
        let mut xs = vec![];
        for r in &turned {
            for i in 0..r.len() {
                let (a, b) = (r[i], r[(i + 1) % r.len()]);
                if (a.1 <= y && y < b.1) || (b.1 <= y && y < a.1) {
                    xs.push(a.0 + (y - a.1) / (b.1 - a.1) * (b.0 - a.0));
                }
            }
        }
        xs.sort_by(|a, b| a.total_cmp(b));
        let mut row: Vec<Stroke> = xs.chunks_exact(2).filter(|p| p[1] - p[0] > 1e-6).map(|p| Stroke { pts: vec![back(p[0], y), back(p[1], y)], closed: false }).collect();
        if k % 2 == 1 {
            row.reverse();
            for st in &mut row {
                st.pts.reverse();
            }
        }
        *budget = budget.saturating_sub(row.len().max(1));
        out.extend(row);
        k += 1;
        y += gap;
    }
    out
}

/// The pictures on a layer.
fn layer_pictures(scene: &Scene, layer: usize) -> Vec<(&Shape, &ImageData)> {
    scene.shapes.iter().filter(|s| s.layer == layer).filter_map(|s| if let Kind::Image(im) = &s.kind { Some((s, im)) } else { None }).collect()
}

/// A picture drawn with lines, by the method of the layer: the strokes in the coordinates of the work area.
fn picture_strokes(shape: &Shape, im: &ImageData, l: &PenLayer, area_y: f64, budget: &mut usize) -> Vec<Stroke> {
    let to_area = |p: Pt| -> P2 { [p.x, area_y - p.y] };
    let mut out: Vec<Stroke> = vec![];
    match l.picture {
        PictureMethod::Rows | PictureMethod::Dots => {
            let Some(g) = raster::sample(shape, im, l.hatch_gap, l.hatch_angle, l.dither) else { return out };
            let dark = |c: usize, r: usize| g.at(c, r) >= 0.5;
            let mut k = 0usize;
            for r in 0..g.rows {
                let mut row: Vec<Stroke> = vec![];
                let mut c = 0;
                while c < g.cols {
                    if !dark(c, r) {
                        c += 1;
                        continue;
                    }
                    let start = c;
                    while c < g.cols && dark(c, r) {
                        c += 1;
                    }
                    if l.picture == PictureMethod::Rows {
                        let y = g.min.y + (r as f64 + 0.5) * g.step;
                        let (a, b) = (Pt::new(g.min.x + start as f64 * g.step, y), Pt::new(g.min.x + c as f64 * g.step, y));
                        row.push(Stroke { pts: vec![to_area(g.rot.apply(a)), to_area(g.rot.apply(b))], closed: false });
                    } else {
                        for cc in start..c {
                            let p = g.centre(cc, r);
                            // A dot is a line so short that it is only a mark.
                            row.push(Stroke { pts: vec![to_area(g.rot.apply(p)), to_area(g.rot.apply(Pt::new(p.x + 0.02, p.y)))], closed: false });
                        }
                    }
                }
                if row.is_empty() {
                    continue;
                }
                // The pen zigzags: every second row is drawn the other way round.
                if k % 2 == 1 {
                    row.reverse();
                    for st in &mut row {
                        st.pts.reverse();
                    }
                }
                k += 1;
                if *budget < row.len() {
                    *budget = 0;
                    break;
                }
                *budget -= row.len();
                out.extend(row);
            }
        }
        PictureMethod::Tones => {
            let Some(inv) = shape.xf.inverse() else { return out };
            let Some(b) = shape.bounds() else { return out };
            let gap = l.hatch_gap.max(0.05);
            let step = (gap / 2.0).max(0.05);
            // Four sets of lines, each starting at a darker tone: the first at the angle of the layer, the next across it, then
            // the two diagonals.
            let sets = [(0.0, 0.18), (90.0, 0.38), (45.0, 0.58), (135.0, 0.78)];
            let corners = [b.min, Pt::new(b.max.x, b.min.y), b.max, Pt::new(b.min.x, b.max.y)];
            for (turn, tone) in sets {
                let (sn, cs) = (l.hatch_angle + turn).to_radians().sin_cos();
                let fwd = |p: Pt| (p.x * cs + p.y * sn, -p.x * sn + p.y * cs);
                let back = |x: f64, y: f64| Pt::new(x * cs - y * sn, x * sn + y * cs);
                let (mut x0, mut x1, mut y0, mut y1) = (f64::INFINITY, f64::NEG_INFINITY, f64::INFINITY, f64::NEG_INFINITY);
                for c in corners {
                    let (x, y) = fwd(c);
                    x0 = x0.min(x);
                    x1 = x1.max(x);
                    y0 = y0.min(y);
                    y1 = y1.max(y);
                }
                let mut y = y0 + gap / 2.0;
                let mut k = 0usize;
                while y < y1 {
                    let mut row: Vec<Stroke> = vec![];
                    let mut run: Option<(f64, f64)> = None;
                    let mut x = x0;
                    while x <= x1 + step {
                        let at_sample = x.min(x1);
                        let p = back(at_sample, y);
                        let local = inv.apply(p);
                        let inside = p.x >= b.min.x - 1e-9 && p.x <= b.max.x + 1e-9 && p.y >= b.min.y - 1e-9 && p.y <= b.max.y + 1e-9;
                        let on = inside && im.darkness_at(local.x, local.y) >= tone;
                        match (on, run) {
                            (true, None) => run = Some((at_sample, at_sample)),
                            (true, Some((s, _))) => run = Some((s, at_sample)),
                            (false, Some((s, e))) => {
                                if e - s > 1e-6 {
                                    row.push(Stroke { pts: vec![to_area(back(s, y)), to_area(back(e, y))], closed: false });
                                }
                                run = None;
                            }
                            _ => {}
                        }
                        x += step;
                    }
                    if let Some((s, e)) = run {
                        if e - s > 1e-6 {
                            row.push(Stroke { pts: vec![to_area(back(s, y)), to_area(back(e, y))], closed: false });
                        }
                    }
                    if !row.is_empty() {
                        if k % 2 == 1 {
                            row.reverse();
                            for st in &mut row {
                                st.pts.reverse();
                            }
                        }
                        k += 1;
                        if *budget < row.len() {
                            *budget = 0;
                            return out;
                        }
                        *budget -= row.len();
                        out.extend(row);
                    }
                    y += gap;
                }
            }
        }
    }
    out
}

/// A better order for the strokes: always the one that starts nearest to where the pen is (open lines may be drawn
/// from either end).
/// The order of the cuts of a drag knife: the deepest outlines first (the holes before the parts round them, the parts before what
/// is round them), each depth in the order of the shortest moves.
fn inside_first(strokes: Vec<Stroke>, start: P2) -> Vec<Stroke> {
    let rings: Vec<(Vec<Pt>, bool)> = strokes.iter().map(|s| (s.pts.iter().map(|p| Pt::new(p[0], p[1])).collect(), s.closed)).collect();
    let depth = base_core::drag_knife::nesting(&rings);
    let deepest = depth.iter().copied().max().unwrap_or(0);
    let mut by_depth: Vec<Vec<Stroke>> = (0..=deepest).map(|_| vec![]).collect();
    for (s, d) in strokes.into_iter().zip(depth) {
        by_depth[d].push(s);
    }
    let mut out = vec![];
    let mut at = start;
    for group in by_depth.into_iter().rev() {
        let ordered = order(group, at);
        if let Some(last) = ordered.last().and_then(|s| s.pts.last()) {
            at = *last;
        }
        out.extend(ordered);
    }
    out
}

fn order(strokes: Vec<Stroke>, start: P2) -> Vec<Stroke> {
    if strokes.len() > MAX_ORDERED || strokes.len() < 2 {
        return strokes;
    }
    let d2 = |a: P2, b: P2| (a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2);
    let mut left: Vec<Option<Stroke>> = strokes.into_iter().map(Some).collect();
    let mut out = Vec::with_capacity(left.len());
    let mut at = start;
    for _ in 0..left.len() {
        let mut best: Option<(usize, bool, f64)> = None;
        for (i, s) in left.iter().enumerate() {
            let Some(s) = s else { continue };
            let first = d2(at, s.pts[0]);
            if best.is_none_or(|b| first < b.2) {
                best = Some((i, false, first));
            }
            if !s.closed {
                let last = d2(at, *s.pts.last().expect("a stroke has points"));
                if best.is_none_or(|b| last < b.2) {
                    best = Some((i, true, last));
                }
            }
        }
        let Some((i, rev, _)) = best else { break };
        let mut s = left[i].take().expect("chosen above");
        if rev {
            s.pts.reverse();
        }
        at = if s.closed { s.pts[0] } else { *s.pts.last().expect("a stroke has points") };
        out.push(s);
    }
    out
}

/// What the program asks of a job beyond the project (an option of the Control tab, never saved).
#[derive(Clone, Copy, Debug, Default)]
pub struct JobOptions {
    /// Draw the elements one after the other: all the layers (and their passes) of an element, a group or a single shape that
    /// is in none, before the next one, the nearest element first. Off: a layer after a layer. The layers of an element are
    /// meant to use the same pen; with different pens the pen is changed at every switch.
    pub one_by_one: bool,
}

/// The pen of a layer: what tells that the pen has to be changed.
type PenKey = ([u8; 3], u64, Option<String>);

pub fn pen_key(l: &PenLayer) -> PenKey {
    (l.color, l.pen_width.to_bits(), l.pen.clone())
}

/// What the moves are made with: the program so far, where the pen is, what the layers made of it and the pen it is drawing with.
struct Gen {
    prog: Program,
    at: P2,
    images: usize,
    outside: bool,
    too_many: bool,
    budget: usize,
    pen: Option<PenKey>,
    last_layer: usize,
    /// The way the drag knife points after the last cut.
    heading: P2,
    /// Fills or pictures left out because the tool is a knife.
    knife_no_fill: bool,
    /// Closed outlines too small for the offset of the knife.
    knife_small: usize,
}

impl Gen {
    /// A comment before the next move: the start of something.
    fn note(&mut self, text: String) {
        self.prog.notes.push(Note { at: self.prog.moves.len(), text, before_layer: false });
    }
    /// A comment after the last move: the end of something (before the comment of the next layer).
    fn note_end(&mut self, text: String) {
        self.prog.notes.push(Note { at: self.prog.moves.len(), text, before_layer: true });
    }
}

/// The length of a line the pen draws, in mm.
fn stroke_length(s: &Stroke) -> f64 {
    s.pts.windows(2).map(|w| ((w[1][0] - w[0][0]).powi(2) + (w[1][1] - w[0][1]).powi(2)).sqrt()).sum()
}

/// The moves of the pen for the project.
#[cfg(test)]
pub fn generate(scene: &Scene, layers: &[PenLayer], dev: &Device) -> Program {
    generate_with(scene, layers, dev, JobOptions::default())
}

/// The elements of the scene in the order they are drawn one by one: groups (even over several layers) and single shapes, each
/// next one the nearest to where the pen was left (it starts at the corner of the work area where the moves start). Each element
/// is a scene of its own shapes.
fn elements_by_distance(scene: &Scene, dev: &Device) -> Vec<Scene> {
    let mut elements: Vec<(Option<u64>, Vec<Shape>)> = vec![];
    for s in &scene.shapes {
        match s.group.and_then(|g| elements.iter().position(|(k, _)| *k == Some(g))) {
            Some(i) => elements[i].1.push(s.clone()),
            None => elements.push((s.group, vec![s.clone()])),
        }
    }
    let (next_id, next_group) = scene.counters();
    let center = |e: &[Shape]| -> Pt {
        match e.iter().filter_map(|s| s.bounds()).reduce(base_core::geom::Rect::union) {
            Some(b) => Pt::new((b.min.x + b.max.x) / 2.0, (b.min.y + b.max.y) / 2.0),
            None => Pt::new(0.0, 0.0),
        }
    };
    let mut left: Vec<(Pt, Vec<Shape>)> = elements.into_iter().map(|(_, e)| (center(&e), e)).collect();
    // The moves start at the bottom left of the work area; the scene counts from the top.
    let (mut at, mut out) = (Pt::new(0.0, dev.area.y), vec![]);
    while !left.is_empty() {
        let next = (0..left.len()).min_by(|&a, &b| {
            let d = |i: usize| (left[i].0.x - at.x).powi(2) + (left[i].0.y - at.y).powi(2);
            d(a).total_cmp(&d(b))
        });
        let (c, e) = left.remove(next.unwrap_or(0));
        at = c;
        out.push(Scene::from_parts(e, next_id, next_group));
    }
    out
}

/// The moves as `generate` makes them, with what the program asks of the job (`JobOptions`).
pub fn generate_with(scene: &Scene, layers: &[PenLayer], dev: &Device, opts: JobOptions) -> Program {
    let mut g = Gen { prog: Program::default(), at: [0.0, 0.0], images: 0, outside: false, too_many: false, budget: MAX_HATCH_LINES, pen: None, last_layer: 0, heading: [1.0, 0.0], knife_no_fill: false, knife_small: 0 };
    if opts.one_by_one {
        let elements = elements_by_distance(scene, dev);
        let total = elements.len();
        for (i, element) in elements.iter().enumerate() {
            g.prog.notes.push(Note { at: g.prog.moves.len(), text: format!("Element {} of {total} start: {} objects", i + 1, element.shapes.len()), before_layer: true });
            run_layers(&mut g, element, layers, dev);
            g.note_end(format!("Element {} end", i + 1));
        }
        // Every element repeats the warnings of its layers: say each one once.
        let mut seen = std::collections::HashSet::new();
        g.prog.warnings.retain(|w| seen.insert(w.to_string()));
    } else {
        run_layers(&mut g, scene, layers, dev);
    }
    let Gen { mut prog, images, outside, too_many, knife_no_fill, knife_small, .. } = g;
    if knife_no_fill {
        prog.warnings.push(Warn::new("a drag knife only cuts outlines: the fills and the pictures of the layers were left out", &[]));
    }
    if knife_small > 0 {
        prog.warnings.push(Warn::new("{} outline(s) are too small for the offset of the knife and come out rounded", &[&knife_small]));
    }
    if images > 0 {
        prog.warnings.push(Warn::new("{} picture(s) on the layers cannot be drawn: trace them to lines first", &[&images]));
    }
    if outside {
        prog.warnings.push(Warn::new("part of the drawing lies outside the work area", &[]));
    }
    if too_many {
        prog.warnings.push(Warn::new("the fill has too many lines: the gap between them is too small, so part of the fill was left out", &[]));
    }
    prog
}


/// The layers of `scene` one after the other, in the order of their rank, into `g`.
fn run_layers(g: &mut Gen, scene: &Scene, layers: &[PenLayer], dev: &Device) {
    for li in crate::layers::layer_order(layers) {
        let l = &layers[li];
        if !l.enabled {
            continue;
        }
        let pictures = if l.draw.is_picture() { layer_pictures(scene, li) } else { vec![] };
        let outlines = layer_outlines(scene, li, dev.area.y, &mut g.images, !l.draw.is_picture());
        if outlines.is_empty() && pictures.is_empty() {
            continue;
        }
        let mut fill: Vec<Stroke> = vec![];
        let mut line: Vec<Stroke> = vec![];
        for (shape, im) in pictures {
            fill.extend(picture_strokes(shape, im, l, dev.area.y, &mut g.budget));
            g.too_many |= g.budget == 0;
        }
        if l.draw.fills() {
            fill.extend(hatch(&outlines, l.hatch_angle, l.hatch_gap, &mut g.budget));
            if l.cross_hatch {
                fill.extend(hatch(&outlines, l.hatch_angle + 90.0, l.hatch_gap, &mut g.budget));
            }
            g.too_many |= g.budget == 0;
        }
        // On a picture layer the shapes that are not pictures are drawn as outlines.
        if l.draw.outlines() || l.draw.is_picture() {
            for o in &outlines {
                let mut pts: Vec<P2> = o.pts.iter().map(|p| [p.x, p.y]).collect();
                if o.closed {
                    pts.push(pts[0]);
                }
                line.push(Stroke { pts, closed: o.closed });
            }
        }
        // A knife cuts outlines only, the holes before the parts round them.
        if let Some(knife) = dev.knife {
            if !fill.is_empty() {
                g.knife_no_fill = true;
                fill.clear();
            }
            let small = 4.0 * knife.offset;
            g.knife_small += line.iter().filter(|s| s.closed && {
                let (mut x0, mut y0, mut x1, mut y1) = (f64::MAX, f64::MAX, f64::MIN, f64::MIN);
                for p in &s.pts {
                    (x0, y0, x1, y1) = (x0.min(p[0]), y0.min(p[1]), x1.max(p[0]), y1.max(p[1]));
                }
                (x1 - x0).max(y1 - y0) < small
            }).count();
        }
        if fill.is_empty() && line.is_empty() {
            continue;
        }
        // Another pen than the last layer's: at a place of the machine, if the device has one, the plotter drives there (the
        // stop for the change comes with the first move of this layer, see `to_gcode`).
        let key = pen_key(l);
        if g.pen.as_ref().is_some_and(|p| *p != key) && dev.pen_change.enabled && !g.prog.moves.is_empty() {
            let at = [dev.pen_change.x, dev.pen_change.y];
            if at != g.at {
                g.prog.moves.push(Move { kind: MoveKind::Travel, from: g.at, to: at, layer: g.last_layer });
                g.at = at;
            }
        }
        g.pen = Some(key);
        g.last_layer = li;
        // The fill is drawn before the outline, each in its own best order.
        for (filling, group) in [(true, fill), (false, line)] {
            if group.is_empty() {
                continue;
            }
            let strokes = if dev.knife.is_some() && !filling { inside_first(group, g.at) } else { order(group, g.at) };
            let n = strokes.len();
            if filling {
                g.note(format!("Fill start: {n} lines"));
            }
            for (k, s) in strokes.into_iter().enumerate() {
                if !filling {
                    g.note(format!("Line {} of {n} start: {}, {:.1} mm", k + 1, if s.closed { "closed" } else { "open" }, stroke_length(&s)));
                }
                if s.pts.iter().any(|p| p[0] < -0.01 || p[1] < -0.01 || p[0] > dev.area.x + 0.01 || p[1] > dev.area.y + 0.01) {
                    g.outside = true;
                }
                let first = s.pts[0];
                if first != g.at {
                    g.prog.moves.push(Move { kind: MoveKind::Travel, from: g.at, to: first, layer: li });
                    g.at = first;
                }
                for pass in 0..l.passes.max(1) {
                    if l.passes > 1 && !filling {
                        g.note(format!("Pass {} of {}", pass + 1, l.passes));
                    }
                    let mut pts: Vec<P2> = if pass % 2 == 1 && !s.closed { s.pts.iter().rev().copied().collect() } else { s.pts.clone() };
                    // A drag knife: the axis goes ahead of the tip, round the corners, and on past the start of a closed outline.
                    if let Some(k) = dev.knife.and_then(|k| k.compensated()) {
                        let outline: Vec<Pt> = pts.iter().map(|p| Pt::new(p[0], p[1])).collect();
                        let heading = Pt::new(g.heading[0], g.heading[1]);
                        let (axis, end) = base_core::drag_knife::compensate(&outline, s.closed, heading, &k);
                        if axis.len() >= 2 {
                            let tip = base_core::drag_knife::simulate(&axis, outline[0], k.offset);
                            g.prog.knife_tips.push((li, tip.iter().map(|p| [p.x, p.y]).collect()));
                            pts = axis.iter().map(|p| [p.x, p.y]).collect();
                            g.heading = [end.x, end.y];
                        }
                    }
                    if pts[0] != g.at {
                        // Back to the other end with the pen down would draw a line: lift it instead.
                        g.prog.moves.push(Move { kind: MoveKind::Travel, from: g.at, to: pts[0], layer: li });
                        g.at = pts[0];
                    }
                    for q in pts.iter().skip(1) {
                        if *q != g.at {
                            g.prog.moves.push(Move { kind: MoveKind::Draw, from: g.at, to: *q, layer: li });
                            g.at = *q;
                        }
                    }
                }
                if !filling {
                    g.note_end(format!("Line {} end", k + 1));
                }
            }
            if filling {
                g.note_end("Fill end".into());
            }
        }
        g.note_end(format!("End of layer {}", l.name));
    }
}

/// The machine zero of the device: the coordinates of a point of the work area as the machine counts them.
pub fn machine_xy(dev: &Device, x: f64, y: f64) -> (f64, f64) {
    match dev.origin {
        Origin::FrontLeft => (x, y),
        Origin::FrontRight => (x - dev.area.x, y),
        Origin::BackLeft => (x, y - dev.area.y),
        Origin::BackRight => (x - dev.area.x, y - dev.area.y),
    }
}

fn num(v: f64) -> String {
    let s = format!("{v:.3}");
    let s = s.trim_end_matches('0').trim_end_matches('.');
    if s == "-0" || s.is_empty() {
        "0".to_string()
    } else {
        s.to_string()
    }
}

/// The line (or the lines of one's own) that take the pen up, or down onto the paper (`down`), without the wait.
fn pen_move(g: &mut String, dev: &Device, down: bool) {
    let at = if down { dev.pen.down } else { dev.pen.up };
    match dev.pen.lift {
        Lift::Z => g.push_str(&format!("G0 Z{}\n", num(at))),
        Lift::Servo => g.push_str(&format!("M3 S{}\n", num(at))),
        Lift::Custom => {
            let own = if down { &dev.pen_gcode.down } else { &dev.pen_gcode.up };
            for l in own.lines().map(str::trim).filter(|l| !l.is_empty()) {
                g.push_str(&base_core::gcode_out::ascii(l));
                g.push('\n');
            }
        }
    }
}

/// G-code that lifts the pen (`down` false) or lowers it onto the paper, where it stands, for the buttons of the Control tab.
/// A servo keeps its position as long as the spindle output is on, so there is no `M5` here.
pub fn pen_gcode(dev: &Device, down: bool) -> String {
    let mut g = String::from("G21 G90\n");
    pen_move(&mut g, dev, down);
    if dev.pen.delay_ms > 0.0 {
        g.push_str(&format!("G4 P{}\n", num(dev.pen.delay_ms / 1000.0)));
    }
    g
}

/// G-code that takes the pen once round the box `b` (min x, min y, max x, max y, in the coordinates of the work area),
/// lifted, so that the sheet can be checked before the job.
pub fn frame_gcode(b: [f64; 4], dev: &Device) -> String {
    let feed = dev.rapid.clamp(100.0, 6000.0);
    let (x0, y0) = machine_xy(dev, b[0], b[1]);
    let (x1, y1) = machine_xy(dev, b[2], b[3]);
    let mut g = String::from("G21 G90\n");
    pen_move(&mut g, dev, false);
    g.push_str(&format!("G0 X{} Y{}\n", num(x0), num(y0)));
    for (x, y) in [(x1, y0), (x1, y1), (x0, y1), (x0, y0)] {
        g.push_str(&format!("G1 X{} Y{} F{}\n", num(x), num(y), num(feed)));
    }
    if dev.pen.lift == Lift::Servo {
        g.push_str("M5\n");
    }
    g
}

/// The G-code (GRBL) for the program. The pen goes up and down with the Z axis or with a servo on the spindle output;
/// when the next layer needs another pen the program stops with M0 so that it can be changed.
#[cfg(test)]
pub fn to_gcode(prog: &Program, dev: &Device, layers: &[PenLayer], pens: &PenLibrary, program_name: &str) -> String {
    let info = base_core::gcode_out::Info { project_name: program_name.to_string(), ..base_core::gcode_out::Info::now() };
    to_gcode_info(prog, dev, layers, pens, &info)
}

/// The place in the program that the variables of the own G-code tell about.
struct Here {
    layer_name: String,
    layer_index: usize,
    total_layers: usize,
}

/// The value of a variable of the own G-code.
fn value_of(info: &base_core::gcode_out::Info, here: &Here, name: &str) -> Option<String> {
    match name {
        "layer_name" => Some(here.layer_name.clone()),
        "layer_index" => Some(here.layer_index.to_string()),
        "total_layers" => Some(here.total_layers.to_string()),
        _ => info.value(name),
    }
}

/// The program as G-code text. `info` tells the variables of the own G-code (the name and the path of the project, the moment);
/// the box of the drawing, the machine and the layer are filled in here. The very start and end of the file and the flavour of
/// the device are applied at the end.
pub fn to_gcode_info(prog: &Program, dev: &Device, layers: &[PenLayer], pens: &PenLibrary, info: &base_core::gcode_out::Info) -> String {
    let program_name = info.project_name.as_str();
    let mut info = info.clone();
    info.machine_name = dev.name.clone();
    // The box of what is drawn, in the coordinates of the machine (the pen is at Z 0 down: the depth is the lift).
    {
        let (mut lo, mut hi) = ([f64::INFINITY; 2], [f64::NEG_INFINITY; 2]);
        for m in prog.moves.iter().filter(|m| m.kind == MoveKind::Draw) {
            let (x, y) = machine_xy(dev, m.to[0], m.to[1]);
            for (i, v) in [x, y].into_iter().enumerate() {
                lo[i] = lo[i].min(v);
                hi[i] = hi[i].max(v);
            }
        }
        info.bounds = lo[0].is_finite().then_some(([lo[0], lo[1], dev.pen.down.min(dev.pen.up)], [hi[0], hi[1], dev.pen.down.max(dev.pen.up)]));
    }
    let mut here = Here { layer_name: String::new(), layer_index: 0, total_layers: layers.len() };
    let mut g = String::new();
    // ASCII only: GRBL takes every byte above 127 for a command of its own, even inside a comment.
    let clean = |s: &str| base_core::gcode_out::ascii(s).chars().filter(|c| c.is_ascii_alphanumeric() || *c == ' ' || *c == '-' || *c == '_' || *c == '.').collect::<String>();
    g.push_str(&format!("(PenCreator: {})\n(Device: {})\n", clean(program_name), clean(&dev.name)));
    for l in base_core::gcode_out::note_lines(&info.notes) {
        g.push_str(&format!("(Note: {l})\n"));
    }
    g.push_str("G21 G90 G17\n");
    let wait = if dev.pen.delay_ms > 0.0 { format!("G4 P{}\n", num(dev.pen.delay_ms / 1000.0)) } else { String::new() };
    let up = |g: &mut String| {
        pen_move(g, dev, false);
        g.push_str(&wait);
    };
    let down = |g: &mut String| {
        pen_move(g, dev, true);
        g.push_str(&wait);
    };
    let used: std::collections::BTreeSet<usize> = prog.moves.iter().map(|m| m.layer).collect();
    let order: Vec<usize> = crate::layers::layer_order(layers).into_iter().filter(|l| used.contains(l)).collect();
    g.push_str(&format!("(Job: {} layers, {} lines drawn, {} mm)\n", order.len(), prog.lifts() + 1, num(prog.draw_length().round())));
    // At most 70 characters: a sender that counts the characters in the buffer of GRBL (128) never sends a longer line.
    let note_text = |t: &str| base_core::gcode_out::comment_text(t, 70);
    let mut next_note = 0usize;
    up(&mut g);
    let mut pen_down = false;
    let mut layer = usize::MAX;
    let mut feed = f64::NAN;
    let mut current: Option<([u8; 3], u64, Option<String>)> = None;
    for (i, m) in prog.moves.iter().enumerate() {
        // The comments at this move in their order: the ends of what came before (and the start of an element) go before a
        // new layer, from the first one that starts something on everything comes after the commands of the layer.
        let here_notes: Vec<&Note> = prog.notes[next_note..].iter().take_while(|n| n.at == i).collect();
        next_note += here_notes.len();
        let lead = here_notes.iter().take_while(|n| n.before_layer).count();
        for n in &here_notes[..lead] {
            g.push_str(&format!("({})\n", note_text(&n.text)));
        }
        if m.layer != layer {
            layer = m.layer;
            if let Some(l) = layers.get(layer) {
                if pen_down {
                    up(&mut g);
                    pen_down = false;
                }
                g.push_str(&format!("(Layer: {})\n", clean(&l.name)));
                let part = order.iter().position(|&x| x == layer).map_or(0, |p| p + 1);
                g.push_str(&format!("({})\n", note_text(&format!("Part {part} of {} of the job: {}", order.len(), l.draw.label()))));
                g.push_str(&format!("({})\n", note_text(&format!("Speed {} mm/min, {} passes", num(l.speed), l.passes.max(1)))));
                here = Here { layer_name: l.name.clone(), layer_index: layer, total_layers: layers.len() };
                let pen = pen_key(l);
                if current.as_ref() != Some(&pen) {
                    // A pen of the library is named; a layer without one by its name and width.
                    let what = match l.pen.as_deref().and_then(|id| pens.get(id)) {
                        Some(p) => format!("{}, {}", clean(&p.name), clean(&l.name)),
                        None => format!("{}, {} mm", clean(&l.name), num(l.pen_width)),
                    };
                    if current.is_some() {
                        // At the place of the change (the plotter has driven there) the G-code of the mechanism, then the stop.
                        let change = &dev.pen_change;
                        if change.enabled {
                            for line in change.lines() {
                                g.push_str(&base_core::gcode_out::substitute(line, &|n| value_of(&info, &here, n)));
                                g.push('\n');
                            }
                        }
                        if !change.enabled || change.pause {
                            g.push_str(&format!("M0 (Change the pen to: {what})\n"));
                        } else {
                            g.push_str(&format!("(Change the pen to: {what})\n"));
                        }
                    } else {
                        g.push_str(&format!("(Pen: {what})\n"));
                    }
                    current = Some(pen);
                }
                feed = f64::NAN;
            }
        }
        for n in &here_notes[lead..] {
            g.push_str(&format!("({})\n", note_text(&n.text)));
        }
        let (x, y) = machine_xy(dev, m.to[0], m.to[1]);
        match m.kind {
            MoveKind::Travel => {
                if pen_down {
                    up(&mut g);
                    pen_down = false;
                }
                g.push_str(&format!("G0 X{} Y{}\n", num(x), num(y)));
            }
            MoveKind::Draw => {
                if !pen_down {
                    down(&mut g);
                    pen_down = true;
                }
                let speed = layers.get(m.layer).map_or(1000.0, |l| l.speed);
                let f = if feed == speed { String::new() } else { format!(" F{}", num(speed)) };
                feed = speed;
                g.push_str(&format!("G1 X{} Y{}{f}\n", num(x), num(y)));
            }
        }
    }
    up(&mut g);
    for n in &prog.notes[next_note..] {
        g.push_str(&format!("({})\n", note_text(&n.text)));
    }
    g.push_str("(End of the job)\n");
    let (hx, hy) = machine_xy(dev, 0.0, 0.0);
    g.push_str(&format!("G0 X{} Y{}\n", num(hx), num(hy)));
    if dev.pen.lift == Lift::Servo {
        g.push_str("M5\n");
    }
    g.push_str("M30\n");
    let lines = |text: &str| -> Vec<String> { text.lines().map(str::trim).filter(|l| !l.is_empty()).map(|l| base_core::gcode_out::substitute(l, &|n| value_of(&info, &here, n))).collect() };
    base_core::gcode_out::finish(&g, &lines(&dev.frame.very_start), &lines(&dev.frame.very_end), &dev.flavour)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layers::{default_layers, Draw};
    use base_core::geom::Xf;

    fn square(scene: &mut Scene, layer: usize, x: f64, y: f64, s: f64) {
        scene.add(layer, Kind::Rect { w: s, h: s }, Xf::translate(x, y));
    }

    fn drawn(prog: &Program) -> f64 {
        prog.draw_length()
    }

    /// A drag knife: the moves go along the axis, ahead of the tip; the tip cuts the square with sharp corners; the hole is cut
    /// before the part round it; a fill is left out with a warning; a cutter that makes good the offset itself gets the outline.
    #[test]
    fn a_drag_knife_cuts_the_outline_with_its_offset_made_good_and_the_hole_first() {
        let mut dev = Device::dummy();
        dev.knife = Some(crate::device::DragKnife { offset: 0.5, ..Default::default() });
        let mut layers = default_layers();
        layers[0].draw = Draw::Outline;
        let mut scene = Scene::default();
        square(&mut scene, 0, 20.0, 20.0, 60.0);
        square(&mut scene, 0, 40.0, 40.0, 10.0);
        let p = generate(&scene, &layers, &dev);
        assert_eq!(p.knife_tips.len(), 2, "two cuts");
        // The hole (10 × 10) first: its tip trace is the smaller one.
        let size = |t: &Vec<P2>| t.iter().map(|q| q[0]).fold(f64::MIN, f64::max) - t.iter().map(|q| q[0]).fold(f64::MAX, f64::min);
        assert!(size(&p.knife_tips[0].1) < 11.0 && size(&p.knife_tips[1].1) > 59.0, "the hole before the part");
        // The axis runs ahead of the tip: the cut moves are not the outline itself, but the tip lands on it.
        let first_cut = p.moves.iter().find(|m| m.kind == MoveKind::Draw).unwrap();
        let tip0 = p.knife_tips[0].1[0];
        let ahead = ((first_cut.from[0] - tip0[0]).powi(2) + (first_cut.from[1] - tip0[1]).powi(2)).sqrt();
        assert!((ahead - 0.5).abs() < 1e-6, "the blade goes down 0.5 mm from where it cuts: {ahead}");
        let ring = |x: f64, y: f64, s: f64, q: P2| {
            let (dx, dy) = ((q[0] - x).min(x + s - q[0]).abs(), (q[1] - y).min(y + s - q[1]).abs());
            let inside = q[0] >= x - 0.05 && q[0] <= x + s + 0.05 && q[1] >= y - 0.05 && q[1] <= y + s + 0.05;
            inside && dx.min(dy) < 0.05
        };
        // The big square in the coordinates of the machine (Y up from the bottom of the sheet).
        let y0 = dev.area.y - 80.0;
        assert!(p.knife_tips[1].1.iter().all(|q| ring(20.0, y0, 60.0, *q)), "the tip cuts the outline");
        // A fill is not cut, and says so.
        layers[0].draw = Draw::OutlineAndFill;
        let p = generate(&scene, &layers, &dev);
        assert!(p.warnings.iter().any(|w| w.to_string().contains("drag knife")), "{:?}", p.warnings);
        // A cutter that makes good the offset itself gets the outline as it is.
        dev.knife = Some(crate::device::DragKnife { by_controller: true, ..Default::default() });
        layers[0].draw = Draw::Outline;
        let p = generate(&scene, &layers, &dev);
        assert!(p.knife_tips.is_empty());
        let d = p.moves.iter().find(|m| m.kind == MoveKind::Draw).unwrap();
        assert!(d.from[0].fract().abs() < 1e-9 && d.from[1].fract().abs() < 1e-9, "on the corner of the square: {:?}", d.from);
        // The settings are checked.
        dev.knife = Some(crate::device::DragKnife { offset: 9.0, ..Default::default() });
        assert!(dev.check().is_err());
    }

    #[test]
    fn the_file_has_a_very_start_and_end_variables_and_a_flavour_and_the_default_changes_nothing() {
        use base_core::gcode_out::{DwellWord, Flavour};
        let mut scene = Scene::default();
        square(&mut scene, 0, 20.0, 20.0, 30.0);
        let layers = default_layers();
        let mut dev = Device::dummy();
        let prog = generate(&scene, &layers, &dev);
        let plain = to_gcode(&prog, &dev, &layers, &PenLibrary::new(), "job");
        assert!(plain.starts_with("(PenCreator: job)\n(Device: ") && plain.trim_end().ends_with("M30") && !plain.contains('%'));
        dev.frame.very_start = "%\nO{project_name}".into();
        dev.frame.very_end = "(done {machine_name} {min_x} {width})".into();
        let info = base_core::gcode_out::Info { project_name: "part".into(), project_path: "/w/part.cre".into(), date: "2026-11-01".into(), time: "12:00".into(), ..Default::default() };
        let text = to_gcode_info(&prog, &dev, &layers, &PenLibrary::new(), &info);
        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(&lines[..3], &["%", "Opart", "(PenCreator: part)"]);
        // The notes of the project come after the name as comments, with nothing in them that could end a comment.
        let noted = base_core::gcode_out::Info { notes: "Birch (3 mm); first\n\nglue".into(), ..info.clone() };
        let with_notes = to_gcode_info(&prog, &dev, &layers, &PenLibrary::new(), &noted);
        let lines: Vec<&str> = with_notes.lines().collect();
        assert_eq!(&lines[2..5], &["(PenCreator: part)", "(Device: {})".replace("{}", &dev.name).as_str(), "(Note: Birch <3 mm>, first)"][..], "{with_notes}");
        assert_eq!(lines[5], "(Note: glue)");
        assert!(!text.contains("(Note"), "no notes, no note lines");
        assert_eq!(lines[lines.len() - 2], "M30");
        assert!(lines.last().unwrap().starts_with("(done ") && !lines.last().unwrap().contains('{'), "{text}");
        assert!(dev.check_variables().is_ok());
        dev.frame.very_end = "{nope}".into();
        assert!(dev.check_variables().unwrap_err().contains("Very end of the file"));
        dev.frame = Default::default();
        dev.pen_change.gcode = "(into {layer_name} {layer_index}/{total_layers})".into();
        assert!(dev.check_variables().is_ok());
        dev.pen.delay_ms = 2000.0;
        dev.flavour = Flavour { decimal_point: true, percent: true, dwell: DwellWord::XSeconds, line_numbers: true, number_step: 5, remove_comments: true };
        let g = to_gcode(&prog, &dev, &layers, &PenLibrary::new(), "job");
        let l: Vec<&str> = g.lines().collect();
        assert_eq!((l[0], l[l.len() - 1]), ("%", "%"));
        assert!(l[1].starts_with("N5 G21 G90 G17"), "{}", l[1]);
        assert!(l.iter().any(|x| x.contains("G4 X2.")), "{g}");
        assert!(!g.contains("(PenCreator"));
        // The old device files read as they were.
        let old: Device = serde_json::from_str(&serde_json::to_string(&Device::dummy()).unwrap().replace(",\"frame\"", ",\"zzz\"")).unwrap();
        assert!(old.frame == Default::default() && old.flavour.is_default());
    }

    #[test]
    fn layers_are_drawn_in_the_order_they_were_moved_to() {
        let mut scene = Scene::default();
        square(&mut scene, 0, 20.0, 20.0, 30.0);
        square(&mut scene, 2, 100.0, 20.0, 30.0);
        let mut layers = default_layers();
        let first_drawn = |prog: &Program| prog.moves.iter().find(|m| m.kind != MoveKind::Travel).map(|m| m.layer);
        assert_eq!(first_drawn(&generate(&scene, &layers, &Device::dummy())), Some(0));
        crate::layers::move_layer(&mut layers, 2, true);
        crate::layers::move_layer(&mut layers, 2, true);
        assert_eq!(first_drawn(&generate(&scene, &layers, &Device::dummy())), Some(2), "the blue pen goes first now");
    }

    #[test]
    fn an_outline_draws_the_perimeter_once_and_starts_where_the_pen_is() {
        let mut scene = Scene::default();
        square(&mut scene, 0, 20.0, 20.0, 30.0);
        let p = generate(&scene, &default_layers(), &Device::dummy());
        assert!((drawn(&p) - 120.0).abs() < 1e-6, "{}", drawn(&p));
        assert_eq!(p.moves[0].kind, MoveKind::Travel);
        assert_eq!(p.lifts(), 0, "one closed line: the pen is not lifted in between");
        assert!(p.warnings.is_empty());
    }

    #[test]
    fn the_drawing_is_turned_over_into_the_work_area() {
        let mut scene = Scene::default();
        square(&mut scene, 0, 10.0, 10.0, 20.0);
        let dev = Device::dummy();
        let p = generate(&scene, &default_layers(), &dev);
        let ys: Vec<f64> = p.moves.iter().filter(|m| m.kind == MoveKind::Draw).flat_map(|m| [m.from[1], m.to[1]]).collect();
        let (lo, hi) = ys.iter().fold((f64::MAX, f64::MIN), |(a, b), v| (a.min(*v), b.max(*v)));
        assert!((lo - (dev.area.y - 30.0)).abs() < 1e-6 && (hi - (dev.area.y - 10.0)).abs() < 1e-6, "{lo} {hi}");
    }

    #[test]
    fn a_fill_covers_the_shape_with_lines_a_gap_apart_and_leaves_holes_empty() {
        let mut layers = default_layers();
        layers[0].draw = Draw::Fill;
        layers[0].hatch_angle = 0.0;
        layers[0].hatch_gap = 1.0;
        let mut scene = Scene::default();
        square(&mut scene, 0, 20.0, 20.0, 20.0);
        let p = generate(&scene, &layers, &Device::dummy());
        // 20 lines of 20 mm.
        assert!((drawn(&p) - 400.0).abs() < 1e-6, "{}", drawn(&p));
        // A square hole of 10 mm in the middle takes 100 mm of the lines away.
        square(&mut scene, 0, 25.0, 25.0, 10.0);
        let p = generate(&scene, &layers, &Device::dummy());
        assert!((drawn(&p) - 300.0).abs() < 1e-6, "{}", drawn(&p));
        // Crossed lines double it.
        layers[0].cross_hatch = true;
        let p = generate(&scene, &layers, &Device::dummy());
        assert!((drawn(&p) - 600.0).abs() < 1.0, "{}", drawn(&p));
    }

    #[test]
    fn fill_and_outline_draws_the_edge_after_the_fill() {
        let mut layers = default_layers();
        layers[0].draw = Draw::OutlineAndFill;
        layers[0].hatch_gap = 2.0;
        layers[0].hatch_angle = 90.0;
        let mut scene = Scene::default();
        square(&mut scene, 0, 20.0, 20.0, 20.0);
        let p = generate(&scene, &layers, &Device::dummy());
        assert!((drawn(&p) - (80.0 + 200.0)).abs() < 1e-6, "{}", drawn(&p));
        // The last drawn lines are the edge: the longest run of moves at the end is the four sides.
        let last: Vec<&Move> = p.moves.iter().rev().take(4).collect();
        assert!(last.iter().all(|m| m.kind == MoveKind::Draw && (m.length() - 20.0).abs() < 1e-6));
    }

    #[test]
    fn the_order_shortens_the_moves_with_the_pen_up() {
        let mut scene = Scene::default();
        // Added in a bad order: far, near, far, near.
        for (x, y) in [(200.0, 150.0), (10.0, 10.0), (210.0, 160.0), (30.0, 10.0)] {
            square(&mut scene, 0, x, y, 5.0);
        }
        let p = generate(&scene, &default_layers(), &Device::dummy());
        assert!(p.travel_length() < 500.0, "{}", p.travel_length());
        assert_eq!(p.lifts(), 3);
    }

    #[test]
    fn passes_draw_the_line_again_and_a_picture_or_an_outside_shape_is_reported() {
        let mut layers = default_layers();
        layers[0].passes = 2;
        let mut scene = Scene::default();
        square(&mut scene, 0, 20.0, 20.0, 30.0);
        let p = generate(&scene, &layers, &Device::dummy());
        assert!((drawn(&p) - 240.0).abs() < 1e-6);
        square(&mut scene, 1, 290.0, 20.0, 30.0);
        let p = generate(&scene, &layers, &Device::dummy());
        assert!(p.warnings.iter().any(|w| w.to_string().contains("outside")));
        layers[1].enabled = false;
        let p = generate(&scene, &layers, &Device::dummy());
        assert!(p.warnings.is_empty(), "a switched off layer is not drawn");
    }

    /// The drawn moves as a list of (square, layer): the squares are 30 wide and 200 apart (x = 0, 200 ...), one entry for each stretch
    /// of drawing on the same square and layer.
    fn draw_order(prog: &Program) -> Vec<(usize, usize)> {
        let mut out: Vec<(usize, usize)> = vec![];
        for m in prog.moves.iter().filter(|m| m.kind == MoveKind::Draw) {
            let at = ((m.from[0] + m.to[0]) / 2.0 / 200.0).round() as usize * 2;
            if out.last() != Some(&(at, m.layer)) {
                out.push((at, m.layer));
            }
        }
        out
    }

    #[test]
    fn one_by_one_draws_all_layers_of_an_element_before_the_next_and_the_nearest_element_first() {
        // Two groups, each a square on layer 0 and a smaller one on layer 1 (the same pen colour would be the usual case; here
        // the layers differ, which only changes the pen between the layers); the far group is drawn first.
        let mut scene = Scene::default();
        for x in [200.0, 0.0] {
            let outer = scene.add(0, Kind::Rect { w: 30.0, h: 30.0 }, Xf::translate(x, 20.0));
            let inner = scene.add(1, Kind::Rect { w: 10.0, h: 10.0 }, Xf::translate(x + 10.0, 30.0));
            let g = scene.new_group_id();
            for sh in scene.shapes.iter_mut().filter(|sh| sh.id == outer || sh.id == inner) {
                sh.group = Some(g);
            }
        }
        let layers = default_layers();
        let dev = Device::dummy();
        let by_layer = draw_order(&generate(&scene, &layers, &dev));
        assert_eq!(by_layer.iter().map(|c| c.1).collect::<Vec<_>>(), vec![0, 0, 1, 1], "{by_layer:?}");
        let one = generate_with(&scene, &layers, &dev, JobOptions { one_by_one: true });
        assert_eq!(draw_order(&one), vec![(0, 0), (0, 1), (2, 0), (2, 1)]);
        assert_eq!(one.moves.iter().filter(|m| m.kind == MoveKind::Draw).count(), generate(&scene, &layers, &dev).moves.iter().filter(|m| m.kind == MoveKind::Draw).count());
    }

    #[test]
    fn single_shapes_that_are_in_no_group_are_elements_of_their_own() {
        let mut scene = Scene::default();
        for x in [400.0, 0.0, 200.0] {
            square(&mut scene, 0, x, 20.0, 30.0);
        }
        let one = generate_with(&scene, &default_layers(), &Device::dummy(), JobOptions { one_by_one: true });
        assert_eq!(draw_order(&one).iter().map(|c| c.0).collect::<Vec<_>>(), vec![0, 2, 4], "nearest first, not as they were drawn");
    }

    fn two_pens() -> (Scene, Vec<PenLayer>) {
        let mut scene = Scene::default();
        square(&mut scene, 0, 20.0, 20.0, 10.0);
        square(&mut scene, 1, 60.0, 20.0, 10.0);
        (scene, default_layers())
    }

    #[test]
    fn the_pen_is_changed_at_the_place_of_the_device_when_it_has_one() {
        let (scene, layers) = two_pens();
        let mut dev = Device::dummy();
        // Without a place the job stops where it is, as before.
        let g = to_gcode(&generate(&scene, &layers, &dev), &dev, &layers, &PenLibrary::new(), "t");
        assert!(g.contains("M0 (Change the pen to: Red pen, 0.4 mm)") && !g.contains("X-30"), "{g}");
        // With one: up, to the place, the G-code of the mechanism, the stop, then on to the drawing.
        dev.pen_change = crate::device::PenChange { enabled: true, x: -30.0, y: 100.0, pause: true, gcode: "M8\n\n G4 P1 ".into() };
        let prog = generate(&scene, &layers, &dev);
        assert!(prog.moves.iter().any(|m| m.kind == MoveKind::Travel && m.to == [-30.0, 100.0]), "the preview shows the way to the place");
        let g = to_gcode(&prog, &dev, &layers, &PenLibrary::new(), "t");
        let at = g.find("G0 X-30 Y100").expect(&g);
        let m0 = g.find("M0 (Change the pen to: Red pen, 0.4 mm)").expect(&g);
        let (m8, g4) = (g.find("M8\n").expect(&g), g.rfind("G4 P1\n").expect(&g));
        assert!(at < m8 && m8 < g4 && g4 < m0, "{g}");
        assert!(g[..at].rfind("G0 Z3").is_some(), "the pen is up before it drives there: {g}");
        assert!(g.find("G0 X60 Y").unwrap() > m0, "the drawing goes on after the stop");
        // The pen of the first layer is not changed.
        assert_eq!(g.matches("G0 X-30 Y100").count(), 1);
        // A mechanism that needs no stop: the lines are sent and the job goes on.
        dev.pen_change.pause = false;
        let g = to_gcode(&generate(&scene, &layers, &dev), &dev, &layers, &PenLibrary::new(), "t");
        assert!(!g.contains("M0") && g.contains("M8\n") && g.contains("(Change the pen to: Red pen"), "{g}");
        // Two layers with the same pen need no change at all.
        let mut same = layers.clone();
        same[1].color = same[0].color;
        same[1].pen_width = same[0].pen_width;
        same[1].pen = same[0].pen.clone();
        let g = to_gcode(&generate(&scene, &same, &dev), &dev, &same, &PenLibrary::new(), "t");
        assert!(!g.contains("G0 X-30") && !g.contains("M8"), "{g}");
    }

    #[test]
    fn the_gcode_lifts_the_pen_between_lines_and_stops_for_another_pen() {
        let mut scene = Scene::default();
        square(&mut scene, 0, 20.0, 20.0, 10.0);
        square(&mut scene, 1, 60.0, 20.0, 10.0);
        let dev = Device::dummy();
        let layers = default_layers();
        let g = to_gcode(&generate(&scene, &layers, &dev), &dev, &layers, &PenLibrary::new(), "test");
        assert!(g.starts_with("(PenCreator: test)"));
        assert!(g.contains("M0 (Change the pen to: Red pen, 0.4 mm)"), "{g}");
        assert!(!g.contains("M0 (Change the pen to: Black"), "no stop before the first pen");
        assert_eq!(g.matches("G0 Z0\n").count(), 2, "down once per pen: {g}");
        assert!(g.trim_end().ends_with("M30"));
        // The pen is up before every travel: no G0 X with the pen down.
        let mut down = false;
        for line in g.lines() {
            if line == "G0 Z0" {
                down = true;
            } else if line == "G0 Z3" {
                down = false;
            } else if line.starts_with("G0 X") {
                assert!(!down, "travel with the pen down: {line}");
            }
        }
    }

    #[test]
    fn the_program_becomes_commands_of_an_axidraw_and_of_an_hpgl_plotter() {
        use base_core::machine::GrblVersion;
        let mut scene = Scene::default();
        square(&mut scene, 0, 20.0, 20.0, 10.0);
        square(&mut scene, 1, 60.0, 20.0, 10.0);
        let layers = default_layers();
        for (version, lift) in [(GrblVersion::Ebb, Lift::Z), (GrblVersion::Ebb, Lift::Servo), (GrblVersion::Hpgl, Lift::Z)] {
            let mut dev = Device::dummy();
            dev.connection.grbl_version = version;
            if lift == Lift::Servo {
                dev.pen = crate::device::PenLift { lift, up: 800.0, down: 300.0, delay_ms: 150.0 };
            }
            let g = to_gcode(&generate(&scene, &layers, &dev), &dev, &layers, &PenLibrary::new(), "t");
            let setup = crate::machine_ui::plotter_setup(&dev).expect("a plotter");
            let mut t = base_core::plotter::Translator::new(setup);
            let mut cmds = t.start();
            for l in base_ui::link::clean_gcode(&g) {
                cmds.extend(t.translate(&l));
            }
            let (down, up) = if version == GrblVersion::Ebb { ("SP,0", "SP,1") } else { ("PD;", "PU;") };
            // Two squares, one pen down each, and the pen up at the end.
            assert_eq!(cmds.iter().filter(|c| *c == down).count(), 2, "{version:?} {lift:?}: {cmds:?}");
            assert!(!t.pen_is_down(), "{cmds:?}");
            assert!(cmds.iter().filter(|c| *c == up).count() >= 2);
            assert!(cmds.iter().all(|c| !c.starts_with('G') && !c.starts_with('M') && !c.starts_with('$')), "only the plotter's own commands: {cmds:?}");
            // The last square ends where it began: 60 mm right, 20 mm up from the corner.
            let w = t.work_position();
            assert!((w[0] - 60.0).abs() < 0.01 && (w[1] - 20.0).abs() < 0.01 || w == [0.0, 0.0], "{w:?}");
        }
        // A GRBL device has no plotter setup.
        assert!(crate::machine_ui::plotter_setup(&Device::dummy()).is_none());
    }

    #[test]
    fn a_cutter_of_its_own_hpgl_gets_its_commands_from_the_start_to_the_end_of_a_job() {
        use base_core::machine::GrblVersion;
        let mut scene = Scene::default();
        square(&mut scene, 0, 20.0, 20.0, 30.0);
        square(&mut scene, 0, 30.0, 30.0, 10.0);
        let layers = default_layers();
        let mut dev = Device::dummy();
        dev.connection.grbl_version = GrblVersion::Hpgl;
        dev.knife = Some(crate::device::DragKnife { by_controller: true, ..Default::default() });
        dev.connection.plotter.hpgl = base_core::plotter::HpglCommands {
            start: "IN;SP1;".into(),
            end: "PU0,0;SP0;".into(),
            pen_down: "PD;".into(),
            pen_up: "PU;".into(),
            speed: "VS{speed};".into(),
            max_speed: 10,
            force: "FS{force};".into(),
            force_value: 80,
        };
        assert!(dev.check().is_ok(), "{:?}", dev.check());
        let g = to_gcode(&generate(&scene, &layers, &dev), &dev, &layers, &PenLibrary::new(), "t");
        let mut t = base_core::plotter::Translator::new(crate::machine_ui::plotter_setup(&dev).expect("a plotter"));
        let mut cmds = t.start();
        for l in base_ui::link::clean_gcode(&g) {
            cmds.extend(t.translate(&l));
        }
        assert_eq!(&cmds[..3], ["IN;", "SP1;", "FS80;"], "{cmds:?}");
        assert_eq!(&cmds[cmds.len() - 2..], ["PU0,0;", "SP0;"], "the end of the device comes last: {cmds:?}");
        assert_eq!(cmds.iter().filter(|c| *c == "PD;").count(), 2, "{cmds:?}");
        let speeds: Vec<u32> = cmds.iter().filter_map(|c| c.strip_prefix("VS")?.strip_suffix(';')?.parse().ok()).collect();
        assert!(!speeds.is_empty() && speeds.iter().all(|v| *v <= 10), "the speed is kept under the top speed: {cmds:?}");
        // The cutter makes good the offset: the outlines are cut as drawn (the hole first), 40 units to the millimetre.
        assert_eq!(&cmds[3..7], ["PA1200,7200;", "PD;", "VS5;", "PA1600,7200;"], "{cmds:?}");
    }

    #[test]
    fn a_servo_uses_m3_and_the_machine_zero_moves_the_numbers() {
        let mut scene = Scene::default();
        square(&mut scene, 0, 20.0, 20.0, 10.0);
        let mut dev = Device::dummy();
        dev.pen.lift = Lift::Servo;
        dev.pen.up = 800.0;
        dev.pen.down = 300.0;
        dev.origin = Origin::BackLeft;
        let layers = default_layers();
        let g = to_gcode(&generate(&scene, &layers, &dev), &dev, &layers, &PenLibrary::new(), "t");
        assert!(g.contains("M3 S800") && g.contains("M3 S300") && g.contains("M5"));
        // Back left: Y counts from the back, so the points are negative.
        assert!(g.lines().any(|l| l.starts_with("G1 X") && l.contains(" Y-")), "{g}");
    }

    #[test]
    fn own_g_code_lifts_the_pen_everywhere_and_a_plotter_of_its_own_language_refuses_it() {
        let mut scene = Scene::default();
        square(&mut scene, 0, 20.0, 20.0, 10.0);
        let mut dev = Device::dummy();
        dev.pen.lift = Lift::Custom;
        dev.pen.delay_ms = 0.0;
        dev.pen_gcode = crate::device::PenGcode { up: "M280 P0 S90\n".into(), down: "  M280 P0 S10\nG4 P0.05".into() };
        assert!(dev.check().is_ok(), "{:?}", dev.check());
        let layers = default_layers();
        let g = to_gcode(&generate(&scene, &layers, &dev), &dev, &layers, &PenLibrary::new(), "t");
        assert!(!g.lines().any(|l| l.starts_with("G0 Z") || l.starts_with("M3 S") || l == "M5"), "no Z and no servo of the spindle output: {g}");
        let (ups, downs) = (g.lines().filter(|l| *l == "M280 P0 S90").count(), g.lines().filter(|l| *l == "M280 P0 S10").count());
        assert!(downs >= 1 && ups == downs + 1, "up before and after each line, down for it: {ups} {downs}");
        assert!(g.contains("M280 P0 S10\nG4 P0.05\n"), "every line of its own, trimmed");
        // Before the job ends the pen is up.
        let end = g.rfind("M280 P0 S90").unwrap();
        assert!(end < g.find("(End of the job)").unwrap());
        assert_eq!(pen_gcode(&dev, false), "G21 G90\nM280 P0 S90\n");
        assert!(frame_gcode([0.0, 0.0, 10.0, 10.0], &dev).starts_with("G21 G90\nM280 P0 S90\nG0 X"));
        // Empty, the same for up and down, or not plain ASCII: refused.
        let mut bad = dev.clone();
        bad.pen_gcode.down = " ".into();
        assert!(bad.check().is_err());
        bad.pen_gcode.down = "M280 P0 S90".into();
        assert!(bad.check().is_err());
        bad.pen_gcode.down = "M280 P0 S10 ; ü".into();
        assert!(bad.check().is_err());
        // AxiDraw and HPGL read no G-code of one's own.
        let mut ebb = dev.clone();
        ebb.connection.grbl_version = base_core::machine::GrblVersion::Ebb;
        assert!(ebb.check().is_err());
        assert!(crate::machine_ui::plotter_setup(&ebb).is_none());
        // Not written while unset: a device of an older version is read and written as before.
        let plain = serde_json::to_string(&Device::dummy()).unwrap();
        assert!(!plain.contains("pen_gcode") && !plain.contains("linuxcnc"));
        let back: Device = serde_json::from_str(&serde_json::to_string(&dev).unwrap()).unwrap();
        assert_eq!(back.pen_gcode, dev.pen_gcode);
    }

    #[test]
    fn the_pen_buttons_send_the_up_and_down_positions_of_the_device() {
        let mut dev = Device::default();
        dev.pen = crate::device::PenLift { lift: Lift::Z, up: 3.0, down: -0.5, delay_ms: 200.0 };
        assert_eq!(pen_gcode(&dev, false), "G21 G90\nG0 Z3\nG4 P0.2\n");
        assert_eq!(pen_gcode(&dev, true), "G21 G90\nG0 Z-0.5\nG4 P0.2\n");
        dev.pen = crate::device::PenLift { lift: Lift::Servo, up: 600.0, down: 300.0, delay_ms: 0.0 };
        assert_eq!(pen_gcode(&dev, false), "G21 G90\nM3 S600\n");
        assert_eq!(pen_gcode(&dev, true), "G21 G90\nM3 S300\n");
    }

    #[test]
    fn the_frame_goes_round_the_drawing_with_the_pen_up() {
        let mut scene = Scene::default();
        square(&mut scene, 0, 20.0, 20.0, 30.0);
        let dev = Device::dummy();
        let p = generate(&scene, &default_layers(), &dev);
        let b = p.bounds().unwrap();
        assert_eq!(b[2] - b[0], 30.0);
        let g = frame_gcode(b, &dev);
        assert!(g.contains("G0 Z3") && !g.contains("Z0") && g.matches("G1 X").count() == 4, "{g}");
        assert!(Program::default().bounds().is_none());
    }

    #[test]
    fn the_time_includes_the_waits() {
        let mut scene = Scene::default();
        square(&mut scene, 0, 20.0, 20.0, 30.0);
        let dev = Device::dummy();
        let layers = default_layers();
        let p = generate(&scene, &layers, &dev);
        let t = p.estimate_seconds(&dev, &layers);
        let lines = 120.0 / layers[0].speed * 60.0;
        assert!(t > lines && t < lines + 30.0, "{t}");
    }

    /// A picture of `w` x `h` mm at (x, y): black everywhere, or a gradient from black (left) to white (right).
    fn picture(scene: &mut Scene, layer: usize, x: f64, y: f64, w: f64, h: f64, gradient: bool) {
        let gray: Vec<u8> = (0..64).map(|i| if gradient { ((i % 8) * 255 / 7) as u8 } else { 0 }).collect();
        let im = base_core::image::ImageData { name: "t".into(), px_w: 8, px_h: 8, gray, w, h, invert: false };
        scene.add(layer, Kind::Image(im), Xf::translate(x, y));
    }

    fn picture_layers(method: PictureMethod) -> Vec<PenLayer> {
        let mut layers = default_layers();
        layers[0].draw = Draw::Picture;
        layers[0].picture = method;
        layers[0].hatch_gap = 1.0;
        layers[0].hatch_angle = 0.0;
        layers[0].dither = base_core::raster::Dither::Threshold;
        layers
    }

    #[test]
    fn the_negative_of_a_white_picture_is_drawn_and_the_white_picture_is_not() {
        let lines = |invert: bool| {
            let mut scene = Scene::default();
            let im = base_core::image::ImageData { name: "t".into(), px_w: 8, px_h: 8, gray: vec![255; 64], w: 20.0, h: 20.0, invert };
            scene.add(0, Kind::Image(im), Xf::translate(10.0, 10.0));
            generate(&scene, &picture_layers(PictureMethod::Rows), &Device::dummy()).moves.len()
        };
        assert!(lines(true) > lines(false));
    }

    #[test]
    fn a_picture_is_drawn_with_rows_of_lines_dots_or_hatching_by_tone() {
        let mut scene = Scene::default();
        picture(&mut scene, 0, 20.0, 20.0, 20.0, 10.0, false);
        // Rows: a black picture of 20 x 10 mm with a gap of 1 mm is ten rows, each one line 20 mm long.
        let rows = generate(&scene, &picture_layers(PictureMethod::Rows), &Device::dummy());
        assert!(rows.warnings.is_empty(), "{:?}", rows.warnings);
        assert!((drawn(&rows) - 200.0).abs() < 1e-6, "{}", drawn(&rows));
        let b = rows.bounds().unwrap();
        assert!((b[2] - b[0] - 20.0).abs() < 1e-6, "{b:?}");
        // The work area counts Y upwards: the picture 20..30 mm from the top of a 210 mm area is 180..190 (the rows are 0.5 in).
        assert!((b[1] - 180.5).abs() < 1e-6 && (b[3] - 189.5).abs() < 1e-6, "{b:?}");
        // Dots: a dot (a mark of 0.02 mm) for each of the 200 cells.
        let dots = generate(&scene, &picture_layers(PictureMethod::Dots), &Device::dummy());
        assert_eq!(dots.moves.iter().filter(|m| m.kind == MoveKind::Draw).count(), 200);
        assert!(drawn(&dots) < 6.0);
        // Hatching by tone: black has all four sets of lines.
        let tones = generate(&scene, &picture_layers(PictureMethod::Tones), &Device::dummy());
        assert!(drawn(&tones) > 3.0 * drawn(&rows) * 0.5, "{} against {}", drawn(&tones), drawn(&rows));
        // A white picture draws nothing, a gradient draws less than black, and darker means more.
        let mut white = Scene::default();
        picture(&mut white, 0, 20.0, 20.0, 20.0, 10.0, true);
        let half = generate(&white, &picture_layers(PictureMethod::Rows), &Device::dummy());
        assert!(drawn(&half) > 0.0 && drawn(&half) < drawn(&rows), "{} / {}", drawn(&half), drawn(&rows));
        let tone_half = generate(&white, &picture_layers(PictureMethod::Tones), &Device::dummy());
        assert!(drawn(&tone_half) > 0.0 && drawn(&tone_half) < drawn(&tones));
    }

    #[test]
    fn a_picture_on_another_kind_of_layer_is_reported_and_the_dithering_changes_the_result() {
        let mut scene = Scene::default();
        picture(&mut scene, 0, 20.0, 20.0, 20.0, 10.0, true);
        let plain = generate(&scene, &default_layers(), &Device::dummy());
        assert!(plain.is_empty() && plain.warnings.iter().any(|w| w.to_string().contains("picture")), "{:?}", plain.warnings);
        let mut a = picture_layers(PictureMethod::Dots);
        let threshold = generate(&scene, &a, &Device::dummy()).moves.len();
        a[0].dither = base_core::raster::Dither::FloydSteinberg;
        let diffused = generate(&scene, &a, &Device::dummy()).moves.len();
        assert_ne!(threshold, diffused, "the method of dithering is used");
        // A picture and an outline on one layer: both are drawn.
        square(&mut scene, 0, 100.0, 20.0, 10.0);
        let both = generate(&scene, &picture_layers(PictureMethod::Rows), &Device::dummy());
        assert!(both.bounds().unwrap()[2] > 100.0, "the square is drawn too");
    }

    #[test]
    fn the_gcode_says_where_every_layer_fill_and_line_starts_and_ends_in_short_lines() {
        let mut layers = default_layers();
        layers[0].draw = Draw::Fill;
        layers[1].passes = 2;
        // GRBL takes every byte above 127 for a command (ń ends in 0x84: the safety door): the file is ASCII.
        layers[1].name = "Pióro czerwień".into();
        let mut scene = Scene::default();
        square(&mut scene, 0, 10.0, 10.0, 20.0);
        square(&mut scene, 1, 50.0, 10.0, 20.0);
        let dev = Device::dummy();
        let prog = generate(&scene, &layers, &dev);
        let g = to_gcode(&prog, &dev, &layers, &PenLibrary::default(), "comments");
        assert!(g.is_ascii() && g.contains("(Layer: Pioro czerwien)"), "{g}");
        for want in ["(Job: 2 layers", "(Part 1 of 2 of the job:", "(Fill start:", "(Fill end)", "(Line 1 of 1 start: closed", "(Pass 2 of 2)", "(Line 1 end)", "(End of layer", "(End of the job)"] {
            assert!(g.contains(want), "{want} missing in:\n{g}");
        }
        for l in g.lines() {
            assert!(l.len() <= 80, "too long: {l}");
            if l.starts_with('(') {
                assert!(l.ends_with(')') && l[1..l.len() - 1].chars().all(|c| c != '(' && c != ')' && c != ';'), "{l}");
            }
        }
        assert_eq!(g.lines().filter(|l| l.starts_with("(Layer:")).count(), 2);
        // The commands are the same as without the comments.
        let mut bare = prog.clone();
        bare.notes.clear();
        let strip = |t: &str| t.lines().filter(|l| !l.starts_with('(')).map(String::from).collect::<Vec<_>>();
        assert_eq!(strip(&g), strip(&to_gcode(&bare, &dev, &layers, &PenLibrary::default(), "comments")));
    }
}

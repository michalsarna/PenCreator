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

#[derive(Clone, Debug, Default)]
pub struct Program {
    pub moves: Vec<Move>,
    /// What could not be done, or looks wrong, in words for the user.
    pub warnings: Vec<Warn>,
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
    let mut g = Gen { prog: Program::default(), at: [0.0, 0.0], images: 0, outside: false, too_many: false, budget: MAX_HATCH_LINES, pen: None, last_layer: 0 };
    if opts.one_by_one {
        for element in elements_by_distance(scene, dev) {
            run_layers(&mut g, &element, layers, dev);
        }
        // Every element repeats the warnings of its layers: say each one once.
        let mut seen = std::collections::HashSet::new();
        g.prog.warnings.retain(|w| seen.insert(w.to_string()));
    } else {
        run_layers(&mut g, scene, layers, dev);
    }
    let Gen { mut prog, images, outside, too_many, .. } = g;
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
        for group in [fill, line] {
            for s in order(group, g.at) {
                if s.pts.iter().any(|p| p[0] < -0.01 || p[1] < -0.01 || p[0] > dev.area.x + 0.01 || p[1] > dev.area.y + 0.01) {
                    g.outside = true;
                }
                let first = s.pts[0];
                if first != g.at {
                    g.prog.moves.push(Move { kind: MoveKind::Travel, from: g.at, to: first, layer: li });
                    g.at = first;
                }
                for pass in 0..l.passes.max(1) {
                    let pts: Vec<P2> = if pass % 2 == 1 && !s.closed { s.pts.iter().rev().copied().collect() } else { s.pts.clone() };
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
            }
        }
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

/// G-code that lifts the pen (`down` false) or lowers it onto the paper, where it stands, for the buttons of the Control tab.
/// A servo keeps its position as long as the spindle output is on, so there is no `M5` here.
pub fn pen_gcode(dev: &Device, down: bool) -> String {
    let at = if down { dev.pen.down } else { dev.pen.up };
    let mut g = String::from("G21 G90\n");
    match dev.pen.lift {
        Lift::Z => g.push_str(&format!("G0 Z{}\n", num(at))),
        Lift::Servo => g.push_str(&format!("M3 S{}\n", num(at))),
    }
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
    match dev.pen.lift {
        Lift::Z => g.push_str(&format!("G0 Z{}\n", num(dev.pen.up))),
        Lift::Servo => g.push_str(&format!("M3 S{}\n", num(dev.pen.up))),
    }
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
    let clean = |s: &str| s.chars().filter(|c| c.is_alphanumeric() || *c == ' ' || *c == '-' || *c == '_' || *c == '.').collect::<String>();
    g.push_str(&format!("(PenCreator: {})\n(Device: {})\n", clean(program_name), clean(&dev.name)));
    g.push_str("G21 G90 G17\n");
    let wait = if dev.pen.delay_ms > 0.0 { format!("G4 P{}\n", num(dev.pen.delay_ms / 1000.0)) } else { String::new() };
    let up = |g: &mut String| {
        match dev.pen.lift {
            Lift::Z => g.push_str(&format!("G0 Z{}\n", num(dev.pen.up))),
            Lift::Servo => g.push_str(&format!("M3 S{}\n", num(dev.pen.up))),
        }
        g.push_str(&wait);
    };
    let down = |g: &mut String| {
        match dev.pen.lift {
            Lift::Z => g.push_str(&format!("G0 Z{}\n", num(dev.pen.down))),
            Lift::Servo => g.push_str(&format!("M3 S{}\n", num(dev.pen.down))),
        }
        g.push_str(&wait);
    };
    up(&mut g);
    let mut pen_down = false;
    let mut layer = usize::MAX;
    let mut feed = f64::NAN;
    let mut current: Option<([u8; 3], u64, Option<String>)> = None;
    for m in &prog.moves {
        if m.layer != layer {
            layer = m.layer;
            if let Some(l) = layers.get(layer) {
                if pen_down {
                    up(&mut g);
                    pen_down = false;
                }
                g.push_str(&format!("(Layer: {})\n", clean(&l.name)));
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
}

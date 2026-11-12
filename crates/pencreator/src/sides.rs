//! The project setup of PenCreator (File → Project setup…, and after File → New): the shared window of baseCreator with a sheet of
//! paper (the usual sizes that fit the plotter), a cylinder on the rotary attachment, and a two-sided drawing: side A and side B are
//! two work areas of one project, the sheet turned over round its horizontal (H) or vertical (V) middle line between them. The side
//! that is shown is `App::scene` (so the editor only ever touches it; the job draws it); the other one waits here with its undo
//! history and is shown turned over as ghosts. The sheet lies in the corner of the zero of the plotter.
use crate::device::Origin;
use crate::{App, History};
use base_core::geom::{Polyline, Pt, Xf};
use base_core::project::Project;
use base_core::scene::{Kind, Scene, Shape};
use base_core::setup::{self, FlipAxis, TwoSided};
use base_ui::i18n::{tr, trf};
use base_ui::project_setup::{Answer, Kind as SetupKind};
use eframe::egui;

/// How far the sheet lies in from the edges of the work area (mm): room for the alignment marks.
pub const PIECE_MARGIN: f64 = 3.0;
/// How far the alignment marks stand off the sheet, and how long their arms are (mm).
const MARK_GAP: f64 = 2.0;
const MARK_ARM: f64 = 10.0;

/// The side that is not shown, and how the two lie.
#[derive(Default)]
pub(crate) struct Sides {
    pub flip: FlipAxis,
    pub ghosts: bool,
    /// Side B is the one shown (in `App::scene`); side A waits in `other`.
    pub shown_b: bool,
    pub other: Scene,
    pub other_history: History,
}

/// What PenCreator asks of the shared window.
fn kind() -> SetupKind {
    SetupKind {
        zero: false,
        cylinder: true,
        marks_tip: "Four corner marks just outside the material, burned on the sheet under it before side A: put the turned-over material back into them for side B.",
        block_note: "The material lies in the corner of the zero of the machine. For a two-sided job lay it in the same place for both sides (the alignment marks show where).",
        cylinder_note: "The workpiece lies on the rotary attachment; Y of the drawing goes round it and the drawing is unrolled from its surface.",
        colour: [0xf4, 0xf1, 0xe8],
        paper: true,
    }
}

/// Put the sides of a drawing (`shown` is the side on the screen) into a project that is saved: its scene becomes side A, and side B
/// goes into the section `two_sided`. Without sides the section is left out.
pub(crate) fn sides_into(p: &mut Project, shown: &Scene, two: Option<&Sides>) {
    p.remove_section(setup::SECTION);
    if let Some(t) = two {
        let (a, b) = if t.shown_b { (&t.other, shown) } else { (shown, &t.other) };
        p.scene = a.clone();
        p.set_section(setup::SECTION, &TwoSided::with_side_b(t.flip, t.ghosts, b));
    }
}

impl App {
    /// The size of the sheet: the project setup, or the work area.
    pub(crate) fn piece_size(&self) -> (f64, f64) {
        match self.piece {
            Some([w, h, _]) if w > 0.0 && h > 0.0 => (w, h),
            _ => (self.device.area.x, self.device.area.y),
        }
    }

    /// Where the sheet lies (its top left corner, design millimetres): in the corner of the zero of the plotter, a little in from the
    /// edges.
    pub(crate) fn piece_origin(&self) -> Pt {
        let (aw, ah) = (self.device.area.x, self.device.area.y);
        let (w, h) = self.piece_size();
        let m = PIECE_MARGIN.min(((aw - w) / 2.0).max(0.0)).min(((ah - h) / 2.0).max(0.0));
        // The design counts down from the back edge; the zero is at the front (bottom) or the back (top), left or right.
        let right = matches!(self.device.origin, Origin::FrontRight | Origin::BackRight);
        let front = matches!(self.device.origin, Origin::FrontLeft | Origin::FrontRight);
        Pt::new(if right { (aw - w - m).max(0.0) } else { m }, if front { (ah - h - m).max(0.0) } else { m })
    }

    /// What a shape of one side lies under on the other side: turned over round the middle line of the sheet where it lies.
    pub(crate) fn side_turn(&self) -> Option<Xf> {
        let t = self.two.as_ref()?;
        let (w, h) = self.piece_size();
        let o = self.piece_origin();
        Some(Xf::translate(-o.x, -o.y).then(setup::turn(t.flip, w, h)).then(Xf::translate(o.x, o.y)))
    }

    /// Show side B (`true`) or side A: the shown drawing and its undo history change places with the waiting ones.
    pub(crate) fn show_side(&mut self, b: bool) {
        let Some(two) = self.two.as_mut() else { return };
        if two.shown_b == b {
            return;
        }
        std::mem::swap(&mut self.scene, &mut two.other);
        std::mem::swap(&mut self.history, &mut two.other_history);
        two.shown_b = b;
        self.ed.sel.clear();
        self.ed.node_sel.clear();
        self.touch_rev();
    }

    /// The drawing of side A and, in a two-sided project, of side B.
    pub(crate) fn sides_scenes(&self) -> (&Scene, Option<&Scene>) {
        match &self.two {
            Some(t) if t.shown_b => (&t.other, Some(&self.scene)),
            Some(t) => (&self.scene, Some(&t.other)),
            None => (&self.scene, None),
        }
    }

    /// Take the sides of a project that was read (side A is shown), and leave their section out of what is kept for the next save.
    pub(crate) fn sides_from_project(&mut self, kept: &mut Project) {
        self.two = kept.section::<TwoSided>(setup::SECTION).ok().flatten().map(|s| Sides { flip: s.flip, ghosts: s.ghosts, shown_b: false, other: s.side_b(), other_history: History::default() });
        kept.remove_section(setup::SECTION);
    }

    /// Open the window with what the project has now (or the work area when it has nothing yet).
    pub(crate) fn open_setup(&mut self) {
        let (aw, ah) = (self.device.area.x, self.device.area.y);
        let piece = self.piece.unwrap_or([aw.min(297.0), ah.min(210.0), 0.1]);
        let rot = &self.device.rotary;
        self.setup = base_ui::project_setup::Setup {
            open: true,
            w: piece[0],
            h: piece[1],
            t: piece[2],
            round: rot.enabled,
            diameter: rot.diameter.clamp(1.0, 1000.0),
            length: aw.min(150.0),
            two_sided: self.two.is_some(),
            flip: self.two.as_ref().map_or(FlipAxis::SideToSide, |t| t.flip),
            marks: self.two.is_none(),
            max: (aw, ah),
            ..Default::default()
        };
    }

    pub(crate) fn setup_window(&mut self, ctx: &egui::Context) {
        let units = base_ui::editor::EditorHost::units(self);
        if base_ui::project_setup::window(ctx, &mut self.setup, units, kind()) == Some(Answer::Apply) {
            self.apply_setup();
        }
    }

    /// Switch the rotary attachment of the plotter in use on or off, in the project and in the list of plotters.
    fn set_rotary(&mut self, on: bool, diameter: f64) {
        for r in std::iter::once(&mut self.device.rotary).chain(self.active.and_then(|i| self.devices.get_mut(i)).map(|d| &mut d.rotary)) {
            r.enabled = on;
            if on {
                r.diameter = diameter;
            }
        }
    }

    /// Put the choices into the project.
    pub(crate) fn apply_setup(&mut self) {
        let st = self.setup.clone();
        if st.round {
            let d = st.diameter.clamp(1.0, 1000.0);
            self.set_rotary(true, d);
            self.status = trf("Project set up: a cylinder of Ø {} mm and {} mm long on the rotary attachment (switched on), zero at {}", &[&format!("{d:.1}"), &format!("{:.1}", st.length), &tr("Front left")]);
            self.touch_rev();
            return;
        }
        let was_rotary = self.device.rotary.enabled;
        if was_rotary {
            self.set_rotary(false, self.device.rotary.diameter);
        }
        // A sheet never bigger than the plotter takes.
        let (w, h) = (st.w.clamp(1.0, self.device.area.x), st.h.clamp(1.0, self.device.area.y));
        self.piece = Some([w, h, st.t.clamp(0.01, 50.0)]);
        self.status = trf("Project set up: material {} x {} x {} mm, zero at {}", &[&format!("{w:.1}"), &format!("{h:.1}"), &format!("{:.1}", st.t), &tr("Front left")]);
        if was_rotary {
            self.status = format!("{} {}", self.status, tr("The rotary attachment of the device is off."));
        }
        if st.two_sided {
            let first = self.two.is_none();
            match self.two.as_mut() {
                Some(t) => t.flip = st.flip,
                None => self.two = Some(Sides { flip: st.flip, ghosts: true, ..Default::default() }),
            }
            if first && st.marks {
                self.show_side(false);
                self.add_alignment_marks();
            }
            self.status = format!("{} — {}", self.status, base_ui::project_setup::flip_name(st.flip));
        } else if self.two.is_some() {
            if self.sides_scenes().1.is_some_and(|b| !b.shapes.is_empty()) {
                self.status = tr("Side B has shapes: the project stays two-sided. Delete them first to make it one-sided.").to_string();
            } else {
                self.show_side(false);
                self.two = None;
            }
        }
        self.ed.view.need_fit = true;
        self.touch_rev();
    }

    /// Four L-shaped marks just outside the corners of the sheet, on the last layer (64) drawn as lines: drawn on the sheet under it
    /// before side A, they show where to put it back after it is turned over.
    pub(crate) fn add_alignment_marks(&mut self) {
        let (w, h) = self.piece_size();
        let o = self.piece_origin();
        let Some(layer) = self.layers.len().checked_sub(1) else { return };
        let before = self.scene.clone();
        let (g, a) = (MARK_GAP, MARK_ARM);
        for (x, y, sx, sy) in [(-g, -g, 1.0, 1.0), (w + g, -g, -1.0, 1.0), (-g, h + g, 1.0, -1.0), (w + g, h + g, -1.0, -1.0)] {
            let (x, y) = (x + o.x, y + o.y);
            let l = Polyline::new(vec![Pt::new(x + sx * a, y), Pt::new(x, y), Pt::new(x, y + sy * a)], false);
            self.scene.add(layer, Kind::Path(vec![l]), Xf::IDENTITY);
        }
        self.history.push(before);
        let l = &mut self.layers[layer];
        l.draw = crate::layers::Draw::Outline;
        l.enabled = true;
        l.name = tr("Alignment marks").to_string();
        self.touch_rev();
    }

    /// The switch over the work area: side A or B, and the ghosts.
    pub(crate) fn sides_bar(&mut self, ui: &mut egui::Ui) {
        let Some(t) = self.two.as_mut() else { return };
        let (shown_b, flip) = (t.shown_b, t.flip);
        let changed = base_ui::two_sided::bar(ui, shown_b, &mut t.ghosts, flip);
        if changed.ghosts {
            self.touch_rev();
        }
        if changed.side {
            self.show_side(!shown_b);
        }
    }

    /// The sheet where it lies (a faint dashed outline), the ghosts of the other side under the shapes of the shown one, and the name of
    /// the shown side.
    pub(crate) fn paint_sides(&self, v: &base_ui::editor::CanvasView) {
        let to = |p: Pt| base_ui::editor::EditorHost::w2s(self, v.origin, p);
        if self.piece.is_some() {
            let (w, h) = self.piece_size();
            let o = self.piece_origin();
            let pts = [o, Pt::new(o.x + w, o.y), Pt::new(o.x + w, o.y + h), Pt::new(o.x, o.y + h), o].map(to);
            v.painter.add(egui::Shape::dashed_line(&pts, egui::Stroke::new(1.2, egui::Color32::from_rgba_unmultiplied(0xb0, 0x7a, 0x30, 200)), 8.0, 4.0));
        }
        let Some(t) = &self.two else { return };
        if t.ghosts {
            let turn = self.side_turn().unwrap_or(Xf::IDENTITY);
            let ghosts: Vec<Shape> = t.other.shapes.iter().map(|s| Shape { xf: s.xf.then(turn), ..s.clone() }).collect();
            let colour = base_ui::two_sided::ghost_colour(self.ed.grid_prefs.bed_fill(self.scheme.is_dark()));
            base_ui::two_sided::paint_ghosts(v.painter, &to, &ghosts, colour);
        }
        base_ui::two_sided::paint_side_name(v.painter, v.bed.left_top() + egui::vec2(6.0, 6.0), t.shown_b);
    }
}

#[cfg(test)]
mod tests {
    use base_core::geom::Xf;
    use base_core::scene::Kind;

    #[test]
    fn a_two_sided_drawing_on_a_sheet_at_the_zero_with_marks_on_the_last_layer() {
        let mut a = crate::tests::app();
        a.open_setup();
        assert!(a.setup.max == (a.device.area.x, a.device.area.y), "the paper sizes are those that fit the plotter");
        a.setup.w = 148.0;
        a.setup.h = 105.0;
        a.setup.two_sided = true;
        a.apply_setup();
        assert_eq!(a.scene.shapes.len(), 4, "four corner marks");
        assert!(a.scene.shapes.iter().all(|s| s.layer == a.layers.len() - 1));
        let o = a.piece_origin();
        assert!(o.x <= 3.0 + 1e-9, "at the left, where the zero of a front-left plotter is: {o:?}");
        let marks = a.scene.shapes.iter().filter_map(|s| s.bounds()).reduce(base_core::geom::Rect::union).unwrap();
        assert!(marks.min.x >= 0.0 && marks.max.y <= a.device.area.y + 1e-9, "on the work area: {marks:?}");
        a.show_side(true);
        let on_b = a.scene.add(0, Kind::Rect { w: 10.0, h: 10.0 }, Xf::translate(20.0, 20.0));
        // Saved: side A is the drawing of the file, side B its section; read back with side A shown.
        let p = base_core::project::Project::from_json(&a.to_project().to_json()).unwrap();
        assert_eq!(p.scene.shapes.len(), 4);
        let mut kept = p.clone();
        let mut b = crate::tests::app();
        b.sides_from_project(&mut kept);
        assert!(b.two.as_ref().is_some_and(|t| t.other.shape(on_b).is_some()));
        // A sheet bigger than the plotter is not taken.
        a.setup.w = 10_000.0;
        a.setup.two_sided = true;
        a.apply_setup();
        assert!(a.piece.unwrap()[0] <= a.device.area.x);
    }
}

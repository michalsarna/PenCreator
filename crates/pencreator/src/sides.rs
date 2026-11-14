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
        solids: false,
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
        self.ed.ghost_sel.clear();
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
            sides: if self.two.is_some() { base_ui::project_setup::Sides::Two } else { base_ui::project_setup::Sides::One },
            flip: self.two.as_ref().map_or(FlipAxis::SideToSide, |t| t.flip),
            marks: self.two.is_none(),
            max: (aw, ah),
            ..Default::default()
        };
    }

    pub(crate) fn setup_window(&mut self, ctx: &egui::Context) {
        let units = base_ui::editor::EditorHost::units(self);
        match base_ui::project_setup::window(ctx, &mut self.setup, units, kind()) {
            Some(Answer::Apply) => self.apply_setup(),
            Some(Answer::Skip) if self.setup.start => self.skip_start_setup(),
            _ => {}
        }
    }

    /// The window of a new project: Skip (or closing it) leaves the project as the program starts one.
    pub(crate) fn open_start_setup(&mut self) {
        self.open_setup();
        self.setup.start = true;
    }

    /// Skip of the window of a new project: one side and no sheet of its own, so the drawing is the whole work area of the plotter,
    /// all of it in the view.
    pub(crate) fn skip_start_setup(&mut self) {
        if self.sides_scenes().1.is_some_and(|b| !b.shapes.is_empty()) {
            return;
        }
        if self.two.is_some() {
            self.show_side(false);
            self.two = None;
        }
        self.piece = None;
        self.ed.view.need_fit = true;
        self.touch_rev();
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
        if st.sides == base_ui::project_setup::Sides::Two {
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

    /// The ghosts of the other side as they lie under the shown one (turned over), while they are shown.
    pub(crate) fn ghosts(&self) -> Vec<Shape> {
        match &self.two {
            Some(t) if t.ghosts => {
                let turn = self.side_turn().unwrap_or(Xf::IDENTITY);
                t.other.shapes.iter().map(|s| Shape { xf: s.xf.then(turn), ..s.clone() }).collect()
            }
            _ => vec![],
        }
    }

    /// The question of Save G-code (or of Preview G-code) in a two-sided job, and the saving or the showing when it is answered.
    pub(crate) fn sides_save_window(&mut self, ctx: &egui::Context) {
        let Some(flip) = self.two.as_ref().map(|t| t.flip) else {
            self.sides_save.open = false;
            return;
        };
        let show = self.sides_save.show;
        if let Some(c) = base_ui::two_sided::save_window(ctx, &mut self.sides_save, flip) {
            if show {
                self.show_sides(c);
            } else {
                self.export_sides(c);
            }
        }
    }

    /// The G-code of both sides of a two-sided job the way `how` says, with the postprocessors: one text (side A, a stop to turn the
    /// material over or none, side B) or two (side A, side B), each saying at its head which side it is. `None` when there is nothing
    /// to make or a postprocessor failed (the reason is in the status line).
    pub(crate) fn sides_texts(&mut self, how: base_ui::two_sided::SaveSides) -> Option<Vec<String>> {
        use base_ui::two_sided::SaveSides;
        if !self.gcode_variables_ok() {
            return None;
        }
        let (a, b) = self.side_programs();
        if a.is_empty() && b.is_empty() {
            self.status = self.nothing_to_draw();
            return None;
        }
        let info = self.gcode_info(&self.gcode_name());
        let text = |prog: &crate::plot::Program| crate::plot::to_gcode_info(prog, &self.device, &self.layers, &self.pens, &info);
        let texts = match how {
            SaveSides::Separate => vec![text(&a), text(&b)],
            SaveSides::OneFile | SaveSides::OneFileWithHold => vec![text(&self.sides_in_one(a, b, how == SaveSides::OneFileWithHold))],
        };
        texts.into_iter().map(|t| self.postprocessed(t)).collect()
    }

    /// The G-code of both sides of a two-sided job: in one file or in two files named `_A` and `_B`.
    pub(crate) fn export_sides(&mut self, how: base_ui::two_sided::SaveSides) {
        use base_ui::i18n::trf;
        use base_ui::two_sided::{side_paths, SaveSides};
        let name = self.gcode_name();
        let Some(texts) = self.sides_texts(how) else { return };
        let Some(p) = rfd::FileDialog::new().add_filter("G-code", &["nc", "gcode", "ngc"]).set_file_name(format!("{name}.nc")).save_file() else { return };
        let paths = match how {
            SaveSides::Separate => {
                let (pa, pb) = side_paths(&p);
                vec![pa, pb]
            }
            _ => vec![p],
        };
        let mut saved = vec![];
        for (path, text) in paths.into_iter().zip(texts) {
            if let Err(e) = std::fs::write(&path, text) {
                self.status = trf("Could not save: {}", &[&e]);
                return;
            }
            saved.push(path);
        }
        self.status = trf("Saved {}", &[&saved.iter().map(|p| p.display().to_string()).collect::<Vec<_>>().join(", ")]);
        if let Some(first) = saved.into_iter().next().filter(|p| self.ed.grid_prefs.gcode_preview_after_save && p.exists()) {
            self.time_gcode_for_device();
            if let Err(e) = base_ui::gcode_view_ui::open(&mut self.gcode_view, first) {
                self.status = e;
            }
        }
    }

    /// The G-code of both sides of a two-sided job in tabs, the way `how` says: one tab with the whole job, or a tab for each side
    /// (`name_A`, `name_B`). Nothing is saved.
    pub(crate) fn show_sides(&mut self, how: base_ui::two_sided::SaveSides) {
        use base_ui::i18n::tr;
        use base_ui::two_sided::{SaveSides, SIDE_B_TEXT};
        let Some(texts) = self.sides_texts(how) else { return };
        let stem = self.gcode_name();
        let slot = self.slot;
        let names = if how == SaveSides::Separate { vec![format!("{stem}_A.nc"), format!("{stem}_B.nc")] } else { vec![format!("{stem}.nc")] };
        if how != SaveSides::Separate {
            // The tab of side B alone goes, unless it has edits.
            self.gcode_texts.retain(|g| g.slot != (slot | SIDE_B_TEXT) || g.dirty());
        }
        let stamp = self.gcode_stamp();
        for (i, (name, text)) in names.into_iter().zip(texts).enumerate() {
            let key = if i == 0 { slot } else { slot | SIDE_B_TEXT };
            match self.gcode_texts.iter_mut().find(|g| g.slot == key) {
                Some(g) => {
                    if !g.dirty() {
                        g.replace(text);
                        g.stamp = stamp;
                    }
                    g.name = name;
                    g.sides = Some(how);
                }
                None => {
                    let mut g = base_ui::gcode_text_ui::GcodeText::new(key, &name, text);
                    g.stamp = stamp;
                    g.sides = Some(how);
                    self.gcode_texts.push(g);
                }
            }
        }
        self.panes.select(base_ui::work_panes::Content::GcodeText(slot));
        self.status = tr("The G-code is shown, nothing is saved").to_string();
    }

    /// The programs of side A and side B, each with the name of its side at its head.
    pub(crate) fn side_programs(&self) -> (crate::plot::Program, crate::plot::Program) {
        let (a, b) = self.sides_scenes();
        let empty = Scene::default();
        let flip = self.two.as_ref().map_or(FlipAxis::SideToSide, |t| t.flip);
        let prog = |s: &Scene, side_b: bool| {
            let mut p = crate::plot::generate_with(s, &self.layers, &self.device, crate::plot::JobOptions { one_by_one: self.one_by_one });
            p.notes.insert(0, crate::plot::Note { at: 0, text: base_ui::two_sided::side_comment(side_b, flip), before_layer: true });
            p
        };
        (prog(a, false), prog(b.unwrap_or(&empty), true))
    }

    /// Side B after side A in one program, with a stop to turn the sheet over between them when `hold`.
    pub(crate) fn sides_in_one(&self, mut a: crate::plot::Program, mut b: crate::plot::Program, hold: bool) -> crate::plot::Program {
        let flip = self.two.as_ref().map_or(FlipAxis::SideToSide, |t| t.flip);
        // The name of side B goes in once, where it starts.
        let note = b.notes.remove(0).text;
        a.then(b, note, hold.then(|| base_ui::two_sided::hold_comment(flip)));
        a
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
            let ghosts = self.ghosts();
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
        a.setup.sides = base_ui::project_setup::Sides::Two;
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
        a.setup.sides = base_ui::project_setup::Sides::Two;
        a.apply_setup();
        assert!(a.piece.unwrap()[0] <= a.device.area.x);
    }

    /// The G-code of both sides in one file says which side is which, stops with the pen up between them when asked, and ends once.
    #[test]
    fn both_sides_in_one_file_with_a_stop_between_them() {
        let mut a = crate::tests::app();
        a.open_setup();
        a.setup.sides = base_ui::project_setup::Sides::Two;
        a.setup.marks = false;
        a.apply_setup();
        a.scene.add(0, Kind::Rect { w: 10.0, h: 10.0 }, Xf::translate(20.0, 20.0));
        a.show_side(true);
        a.scene.add(0, Kind::Rect { w: 5.0, h: 5.0 }, Xf::translate(60.0, 40.0));
        let (pa, pb) = a.side_programs();
        let info = a.gcode_info("job");
        let one = a.sides_in_one(pa.clone(), pb.clone(), true);
        let g = crate::plot::to_gcode_info(&one, &a.device, &a.layers, &a.pens, &info);
        let at = |t: &str| g.find(t).unwrap_or_else(|| panic!("no {t:?} in\n{g}"));
        let (side_a, stop, side_b) = (at("(Side A)"), at("M0 (Turn the material over"), at("(Side B:"));
        assert!(side_a < stop && stop < side_b, "{g}");
        assert_eq!(g.matches("M30").count(), 1);
        assert!(!g[..stop].lines().rev().take_while(|l| !l.starts_with("G1")).any(|l| l.starts_with("G1")));
        // Without the stop, and each side alone.
        let g = crate::plot::to_gcode_info(&a.sides_in_one(pa.clone(), pb.clone(), false), &a.device, &a.layers, &a.pens, &info);
        assert!(!g.contains("M0 (Turn") && g.contains("(Side B:"));
        let g = crate::plot::to_gcode_info(&pb, &a.device, &a.layers, &a.pens, &info);
        assert!(g.contains("(Side B:") && !g.contains("(Side A)"));
    }

    /// The preview has the switch of the side; the picture follows it (the program of the side chosen is shown, never the other one).
    #[test]
    fn the_preview_switches_the_side_and_its_program() {
        use egui_kittest::kittest::Queryable;
        let mut a = crate::tests::app();
        a.open_setup();
        a.setup.sides = base_ui::project_setup::Sides::Two;
        a.setup.marks = false;
        a.apply_setup();
        a.scene.add(0, Kind::Rect { w: 10.0, h: 10.0 }, Xf::translate(20.0, 20.0));
        a.show_side(true);
        a.scene.add(0, Kind::Rect { w: 5.0, h: 5.0 }, Xf::translate(150.0, 40.0));
        a.touch_rev();
        a.show_plot = true;
        let mut h = egui_kittest::Harness::builder().with_accessibility_check(false).with_context_setup(|ctx| {
            base_ui::fonts::install(ctx);
            base_ui::icons::install(ctx);
        }).with_size([1000.0, 700.0]).build_ui_state(|ui, a: &mut crate::App| a.preview_work_panel(ui), a);
        h.run_steps(3);
        let right = |a: &crate::App| a.prog.as_ref().and_then(|p| p.bounds()).map_or(f64::NAN, |b| b[2]);
        assert!(right(h.state()) >= 150.0, "side B: {}", right(h.state()));
        h.get_by_label("Side A").click();
        h.run_steps(3);
        assert!(!h.state().two.as_ref().unwrap().shown_b);
        assert!(right(h.state()) <= 31.0, "side A: {}", right(h.state()));
    }

    /// Preview G-code of a two-sided job asks how, then shows one tab with the whole job or a tab for each side; the tab of side B is
    /// made again with the project like the other one.
    #[test]
    fn the_g_code_of_a_two_sided_job_is_shown_whole_or_a_tab_for_each_side() {
        use base_ui::two_sided::{SaveSides, SIDE_B_TEXT};
        let mut a = crate::tests::app();
        a.open_setup();
        a.setup.sides = base_ui::project_setup::Sides::Two;
        a.setup.marks = false;
        a.apply_setup();
        a.scene.add(0, Kind::Rect { w: 10.0, h: 10.0 }, Xf::translate(20.0, 20.0));
        a.show_side(true);
        a.scene.add(0, Kind::Rect { w: 5.0, h: 5.0 }, Xf::translate(60.0, 40.0));
        a.preview_gcode();
        assert!(a.sides_save.open && a.sides_save.show, "asked first");
        let slot = a.slot;
        a.show_sides(SaveSides::Separate);
        let tab = |a: &crate::App, key: usize| a.gcode_texts.iter().find(|g| g.slot == key).map(|g| (g.name.clone(), g.text.clone(), g.sides));
        let (ta, tb) = (tab(&a, slot).unwrap(), tab(&a, slot | SIDE_B_TEXT).unwrap());
        assert!(ta.0.ends_with("_A.nc") && tb.0.ends_with("_B.nc"));
        assert!(ta.1.contains("(Side A)") && !ta.1.contains("Side B") && tb.1.contains("(Side B"), "{}", ta.1);
        // The tab of side B follows the project (a new note changes what the G-code is made of).
        let before = tb.1.clone();
        a.scene.add(0, Kind::Rect { w: 7.0, h: 7.0 }, Xf::translate(90.0, 40.0));
        a.description = "a note".into();
        a.refresh_gcode_text_after(std::time::Duration::ZERO);
        assert_ne!(tab(&a, slot | SIDE_B_TEXT).unwrap().1, before, "side B made again");
        // One file: one tab with both sides and the stop; the tab of side B goes.
        a.show_sides(SaveSides::OneFileWithHold);
        assert!(tab(&a, slot | SIDE_B_TEXT).is_none());
        let whole = tab(&a, slot).unwrap();
        assert!(whole.1.contains("(Side A)") && whole.1.contains("M0 (Turn") && whole.1.contains("(Side B"), "{}", whole.1);
        assert_eq!(whole.2, Some(SaveSides::OneFileWithHold));
    }

    #[test]
    fn skipping_the_window_of_a_new_project_gives_one_side_and_the_whole_work_area() {
        let mut a = crate::tests::app();
        a.open_setup();
        a.setup.sides = base_ui::project_setup::Sides::Two;
        a.setup.w = 100.0;
        a.setup.h = 80.0;
        a.apply_setup();
        assert!(a.two.is_some() && a.piece.is_some());
        a.open_start_setup();
        assert!(a.setup.start);
        a.skip_start_setup();
        assert!(a.two.is_none() && a.piece.is_none());
        assert!(a.ed.view.need_fit);
        a.open_setup();
        assert!(!a.setup.start);
    }
}

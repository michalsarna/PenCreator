//! The plot of the project: a window that shows what the pen will do, and the export of G-code.
use crate::plot::{self, MoveKind, Program};
use crate::App;
use base_ui::i18n::{tr, trf};
use base_ui::editor::EditorHost;
use base_ui::theme;
use base_ui::window::popup_with;
use eframe::egui::{self, Color32, Pos2, RichText, Stroke};

/// How the plot is looked at: the zoom and the shift on top of "fit to the window", and how much of it is drawn.
pub struct PlotView {
    pub zoom: f32,
    pub pan: egui::Vec2,
    /// From 0 to 1: how much of the program is shown.
    pub progress: f32,
    pub show_travel: bool,
    /// Asked for by a button, done by the next picture (which knows the size of its window).
    pub fit: Option<Fit>,
}

/// What the view is fitted to.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Fit {
    /// The whole sheet.
    Area,
    /// The lines that are drawn.
    Elements,
}

impl Default for PlotView {
    fn default() -> PlotView {
        PlotView { zoom: 1.0, pan: egui::Vec2::ZERO, progress: 1.0, show_travel: true, fit: None }
    }
}

impl PlotView {
    pub fn reset(&mut self) {
        self.zoom = 1.0;
        self.pan = egui::Vec2::ZERO;
    }
}

impl App {
    /// The moves of the pen for the project as it is now.
    pub(crate) fn make_program(&self) -> Program {
        plot::generate(&self.scene, &self.layers, &self.device)
    }

    /// The program is made again when the project changed since it was made.
    pub(crate) fn refresh_program(&mut self) {
        if self.prog.is_none() || self.prog_rev != self.rev {
            self.prog = Some(self.make_program());
            self.prog_rev = self.rev;
        }
    }

    /// The toolpaths over the work area (the Toolpaths button): the lines in the colour of their layer, the moves with
    /// the pen up in light blue.
    pub(crate) fn paint_toolpaths(&mut self, v: &base_ui::editor::CanvasView) {
        if !self.toolpaths_on {
            return;
        }
        self.refresh_program();
        let Some(prog) = self.prog.as_ref() else { return };
        let h = self.device.area.y;
        let to_screen = |p: plot::P2| self.w2s(v.origin, base_core::geom::Pt::new(p[0], h - p[1]));
        let many = prog.moves.len() > 100_000;
        let mut shapes = Vec::with_capacity(prog.moves.len());
        for m in &prog.moves {
            let (a, b) = (to_screen(m.from), to_screen(m.to));
            match m.kind {
                MoveKind::Draw => {
                    let c = self.layers.get(m.layer).map_or(Color32::RED, |l| l.color32());
                    shapes.push(egui::Shape::line_segment([a, b], Stroke::new(1.5, c)));
                }
                MoveKind::Travel if !many => shapes.push(egui::Shape::line_segment([a, b], Stroke::new(0.6, Color32::from_rgb(0x70, 0x90, 0xc0).gamma_multiply(0.8)))),
                MoveKind::Travel => {}
            }
        }
        v.painter.extend(shapes);
    }

    /// The live picture of the camera in a window of its own (the Camera view button).
    pub(crate) fn camera_window(&mut self, ctx: &egui::Context) {
        if !self.show_camera {
            return;
        }
        let mut open = true;
        let mut fit = std::mem::take(&mut self.cam_fit);
        base_ui::window::popup_fit(ctx, "camera_window", &tr("Camera view"), [560.0, 440.0], &mut open, &mut fit, |ui, fit| {
            std::mem::swap(&mut self.cam_fit, fit);
            self.camera_tab(ui);
            std::mem::swap(&mut self.cam_fit, fit);
        });
        self.cam_fit = fit;
        self.show_camera = open;
    }

    /// Open the plot window with a fresh program.
    pub(crate) fn open_preview(&mut self) {
        self.prog = Some(self.make_program());
        self.prog_rev = self.rev;
        self.show_plot = true;
    }

    /// Ask where to write the G-code and write it.
    pub(crate) fn export_gcode(&mut self) {
        let prog = self.make_program();
        if prog.is_empty() {
            self.status = tr("There is nothing to draw: put shapes on layers that are switched on.").to_string();
            return;
        }
        let name = self.path.as_ref().and_then(|p| p.file_stem()).map(|s| s.to_string_lossy().into_owned()).unwrap_or_else(|| "untitled".into());
        let Some(p) = rfd::FileDialog::new().add_filter("G-code", &["nc", "gcode", "ngc"]).set_file_name(format!("{name}.nc")).save_file() else { return };
        let text = plot::to_gcode(&prog, &self.device, &self.layers, &self.pens, &name);
        let Some(text) = self.postprocessed(text) else { return };
        self.status = match std::fs::write(&p, text) {
            Ok(()) => trf("Saved {}", &[&p.display()]),
            Err(e) => trf("Could not save: {}", &[&e]),
        };
        if let Some(n) = &self.post_note {
            self.status.push_str(&format!(" — {n}"));
        }
        if !prog.warnings.is_empty() {
            self.status.push_str(&format!(" ({})", prog.warnings.iter().map(|w| w.text()).collect::<Vec<_>>().join("; ")));
        }
    }

    pub(crate) fn preview_window(&mut self, ctx: &egui::Context) {
        if !self.show_plot {
            return;
        }
        if self.prog.is_none() || self.prog_rev != self.rev {
            self.prog = Some(self.make_program());
            self.prog_rev = self.rev;
        }
        let mut open = true;
        let mut fit = None;
        let tools = |ui: &mut egui::Ui| {
            // Icons are added right to left: the work area button is the one on the left.
            for (what, icon, tip) in [(Fit::Elements, "expand", "Fit all objects"), (Fit::Area, "scan", "Fit work area")] {
                if base_ui::widgets::icon_button(ui, icon, &tr(tip), true, 22.0) {
                    fit = Some(what);
                }
            }
        };
        popup_with(ctx, "preview_plot", &tr("Preview"), [980.0, 700.0], &mut open, tools, |ui| {
            let prog = self.prog.as_ref().expect("the program was made above");
            ui.horizontal_wrapped(|ui| {
                if ui.button(tr("Reset view")).on_hover_text(tr("Back to the first view. Double click on the picture does the same.")).clicked() {
                    self.view.reset();
                }
                ui.checkbox(&mut self.view.show_travel, tr("Moves with the pen up"));
            });
            ui.horizontal(|ui| {
                self.playback.show(ui, &mut self.view.progress);
            });
            let secs = prog.estimate_seconds(&self.device, &self.layers);
            ui.label(RichText::new(trf("Time {} / {}", &[&base_ui::widgets::fmt_time(secs * self.view.progress as f64), &base_ui::widgets::fmt_time(secs)])).strong());
            ui.label(
                RichText::new(trf("Lines: {} m, moves with the pen up: {} m, pen lifts: {}, about {} min", &[&format!("{:.2}", prog.draw_length() / 1000.0), &format!("{:.2}", prog.travel_length() / 1000.0), &prog.lifts(), &format!("{:.1}", secs / 60.0)])).color(theme::text_dim()),
            );
            for w in &prog.warnings {
                ui.label(RichText::new(format!("• {}", w.text())).color(Color32::from_rgb(0xd0, 0x70, 0x20)));
            }
            if prog.is_empty() {
                ui.add_space(30.0);
                ui.label(RichText::new(tr("There is nothing to show yet: draw shapes and put them on layers that are switched on.")).color(theme::text_dim()));
            } else {
                show_plot(ui, &mut self.view, prog, &self.layers, &self.pens, self.device.area.x, self.device.area.y);
            }
        });
        if fit.is_some() {
            self.view.fit = fit;
        }
        self.show_plot = open;
        if !open {
            self.playback.playing = false;
        }
    }
}

/// The paper and the lines on it, with the pen at the end of what is shown.
fn show_plot(ui: &mut egui::Ui, view: &mut PlotView, prog: &Program, layers: &[crate::layers::PenLayer], pens: &crate::pens::PenLibrary, w: f64, h: f64) {
    let size = base_ui::window::fill_size(ui).max(egui::vec2(200.0, 200.0));
    let (rect, resp) = ui.allocate_exact_size(size, egui::Sense::click_and_drag());
    if resp.double_clicked() {
        view.reset();
    }
    if resp.dragged() {
        view.pan += resp.drag_delta();
    }
    if resp.hovered() {
        let scroll = ui.input(|i| i.smooth_scroll_delta.y);
        if scroll != 0.0 {
            view.zoom = (view.zoom * (1.0 + scroll * 0.002)).clamp(0.2, 50.0);
        }
    }
    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, 0.0, theme::panel());
    let fit = ((rect.width() - 40.0) / w as f32).min((rect.height() - 40.0) / h as f32).max(0.01);
    match view.fit.take() {
        Some(Fit::Area) => view.reset(),
        Some(Fit::Elements) => {
            if let Some([x0, y0, x1, y1]) = prog.bounds() {
                // The lines (and half the width of the thickest pen) fill the window; the middle of the box goes to the middle.
                let pad = layers.iter().map(|l| l.pen_width).fold(0.0, f64::max) / 2.0;
                let (bw, bh) = ((x1 - x0 + 2.0 * pad).max(0.5), (y1 - y0 + 2.0 * pad).max(0.5));
                let scale = ((rect.width() - 40.0) / bw as f32).min((rect.height() - 40.0) / bh as f32).max(0.01);
                view.zoom = (scale / fit).clamp(0.2, 50.0);
                let scale = fit * view.zoom;
                let (cx, cy) = ((x0 + x1) / 2.0, (y0 + y1) / 2.0);
                view.pan = egui::vec2(-((cx - w / 2.0) as f32) * scale, ((cy - h / 2.0) as f32) * scale);
            }
        }
        None => {}
    }
    let scale = fit * view.zoom;
    let origin = rect.center() + view.pan - egui::vec2(w as f32 * scale / 2.0, -(h as f32) * scale / 2.0);
    let at = |p: [f64; 2]| Pos2::new(origin.x + p[0] as f32 * scale, origin.y - p[1] as f32 * scale);
    let paper = egui::Rect::from_two_pos(at([0.0, 0.0]), at([w, h]));
    painter.rect_filled(paper, 0.0, Color32::WHITE);
    painter.rect_stroke(paper, 0.0, Stroke::new(1.0, theme::border()), egui::StrokeKind::Outside);
    let total: f64 = prog.moves.iter().map(|m| m.length()).sum();
    let shown = total * view.progress as f64;
    let mut done = 0.0;
    let mut pen_at = None;
    let marks: Vec<crate::pens::Pen> = layers.iter().map(|l| crate::pens_ui::pen_of_layer(l, pens)).collect();
    let mut shapes = vec![];
    for m in &prog.moves {
        let len = m.length();
        if done >= shown {
            break;
        }
        let t = if len > 0.0 { ((shown - done) / len).min(1.0) } else { 1.0 };
        let to = [m.from[0] + (m.to[0] - m.from[0]) * t, m.from[1] + (m.to[1] - m.from[1]) * t];
        done += len;
        pen_at = Some(to);
        match m.kind {
            MoveKind::Draw => {
                if let Some(pen) = marks.get(m.layer) {
                    let dir = (m.to[1] - m.from[1]).atan2(m.to[0] - m.from[0]);
                    crate::pens_ui::mark_shapes(at(m.from), at(to), dir, pen, scale, &mut shapes);
                } else {
                    painter.line_segment([at(m.from), at(to)], Stroke::new(1.0, Color32::BLACK));
                }
            }
            MoveKind::Travel if view.show_travel => {
                painter.extend(egui::Shape::dashed_line(&[at(m.from), at(to)], Stroke::new(1.0, Color32::from_rgb(0x60, 0x90, 0xd0)), 5.0, 4.0));
            }
            MoveKind::Travel => {}
        }
    }
    painter.extend(shapes);
    if let Some(p) = pen_at {
        painter.circle_stroke(at(p), 5.0, Stroke::new(1.5, Color32::from_rgb(0xd0, 0x30, 0x30)));
    }
}

#[cfg(test)]
mod tests {
    use crate::App;
    use base_core::geom::Xf;
    use base_core::scene::Kind;

    #[test]
    fn the_window_follows_the_project_and_is_drawn() {
        let ctx = egui::Context::default();
        let mut a = App::build(&ctx, None, false);
        a.open_preview();
        assert!(a.show_plot && a.prog.as_ref().unwrap().is_empty());
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| a.preview_window(ui.ctx()));
        out.textures_delta.clear();
        a.scene.add(0, Kind::Rect { w: 30.0, h: 30.0 }, Xf::translate(20.0, 20.0));
        a.touch_rev();
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| a.preview_window(ui.ctx()));
        out.textures_delta.clear();
        assert!(!a.prog.as_ref().unwrap().is_empty(), "the new shape is drawn");
        assert_eq!(a.prog_rev, a.rev);
        a.view.progress = 0.4;
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| a.preview_window(ui.ctx()));
        out.textures_delta.clear();
        // The two fits: the whole sheet is the first view; the lines of a small shape zoom in.
        let mut zoom_of = |fit| {
            a.view.fit = Some(fit);
            let mut out = ctx.run_ui(egui::RawInput::default(), |ui| a.preview_window(ui.ctx()));
            out.textures_delta.clear();
            assert!(a.view.fit.is_none(), "the request is done by the picture");
            a.view.zoom
        };
        assert_eq!(zoom_of(super::Fit::Area), 1.0);
        assert!(zoom_of(super::Fit::Elements) > 1.5);
    }
}

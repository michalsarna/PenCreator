//! The plot of the project: a window that shows what the pen will do, and the export of G-code.
use crate::plot::{self, MoveKind, Program};
use crate::App;
use base_ui::i18n::{tr, trf};
use base_ui::theme;
use base_ui::window::popup;
use eframe::egui::{self, Color32, Pos2, RichText, Stroke};

/// How the plot is looked at: the zoom and the shift on top of "fit to the window", and how much of it is drawn.
pub struct PlotView {
    pub zoom: f32,
    pub pan: egui::Vec2,
    /// From 0 to 1: how much of the program is shown.
    pub progress: f32,
    pub show_travel: bool,
}

impl Default for PlotView {
    fn default() -> PlotView {
        PlotView { zoom: 1.0, pan: egui::Vec2::ZERO, progress: 1.0, show_travel: true }
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
        let text = plot::to_gcode(&prog, &self.device, &self.layers, &name);
        self.status = match std::fs::write(&p, text) {
            Ok(()) => trf("Saved {}", &[&p.display()]),
            Err(e) => trf("Could not save: {}", &[&e]),
        };
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
        let mut export = false;
        popup(ctx, "preview_plot", &tr("Preview"), [980.0, 700.0], &mut open, |ui| {
            let prog = self.prog.as_ref().expect("the program was made above");
            ui.horizontal_wrapped(|ui| {
                if ui.button(tr("Reset view")).on_hover_text(tr("Back to the first view. Double click on the picture does the same.")).clicked() {
                    self.view.reset();
                }
                ui.checkbox(&mut self.view.show_travel, tr("Moves with the pen up"));
                if ui.button(tr("Export G-code…")).clicked() {
                    export = true;
                }
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
                show_plot(ui, &mut self.view, prog, &self.layers, self.device.area.x, self.device.area.y);
            }
        });
        if export {
            self.export_gcode();
        }
        self.show_plot = open;
        if !open {
            self.playback.playing = false;
        }
    }
}

/// The paper and the lines on it, with the pen at the end of what is shown.
fn show_plot(ui: &mut egui::Ui, view: &mut PlotView, prog: &Program, layers: &[crate::layers::PenLayer], w: f64, h: f64) {
    let size = ui.available_size().max(egui::vec2(200.0, 200.0));
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
                let l = layers.get(m.layer);
                let width = l.map_or(0.4, |l| l.pen_width) as f32 * scale;
                let color = l.map_or(Color32::BLACK, |l| l.color32());
                painter.line_segment([at(m.from), at(to)], Stroke::new(width.max(1.0), color));
            }
            MoveKind::Travel if view.show_travel => {
                painter.extend(egui::Shape::dashed_line(&[at(m.from), at(to)], Stroke::new(1.0, Color32::from_rgb(0x60, 0x90, 0xd0)), 5.0, 4.0));
            }
            MoveKind::Travel => {}
        }
    }
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
    }
}

//! The pen library window and the drawing of a mark: what a line looks like when the pen makes it.
use crate::pens::{Cap, Pen, PenKind, PenLibrary};
use base_ui::i18n::tr;
use base_ui::theme;
use base_ui::window::popup;
use eframe::egui::{self, Color32, Pos2, Stroke};

/// The state of the window.
#[derive(Default)]
pub struct PensWindow {
    pub open: bool,
    selected: Option<String>,
    /// The pen being edited (a copy; it is written to the library when something changes).
    draft: Option<Pen>,
    message: String,
    /// Something in the library changed: the caller keeps it (in the settings).
    pub changed: bool,
}

impl PensWindow {
    fn select(&mut self, lib: &PenLibrary, id: &str) {
        self.selected = Some(id.to_string());
        self.draft = lib.get(id).cloned();
    }
}

/// The shapes of a line from `a` to `b` (screen points) drawn by the pen at `scale` pixels to the millimetre: the line is as thick
/// as the pen makes a line in its direction (`dir`, radians in the work area), with round or flat ends, and a point (a dot) is the
/// mark of the pen. The pen is see-through by its opacity, so overlaps get darker.
pub fn mark_shapes(a: Pos2, b: Pos2, dir: f64, pen: &Pen, scale: f32, out: &mut Vec<egui::Shape>) {
    let color = pen.color32();
    let t = (pen.thickness(dir) as f32 * scale).max(1.0);
    if (b - a).length() < 0.3 {
        // A dot: an ellipse of the width and the height of the mark, turned by the tip.
        let (w, h) = ((pen.width as f32 * scale).max(1.0), (pen.height as f32 * scale).max(1.0));
        let (s, c) = (pen.angle as f32).to_radians().sin_cos();
        let pts: Vec<Pos2> = (0..16)
            .map(|i| {
                let th = i as f32 / 16.0 * std::f32::consts::TAU;
                let (x, y) = (th.cos() * w / 2.0, th.sin() * h / 2.0);
                // The work area counts Y upwards: the screen turns the tip the other way.
                Pos2::new(a.x + x * c + y * s, a.y - (x * s - y * c))
            })
            .collect();
        out.push(egui::Shape::convex_polygon(pts, color, Stroke::NONE));
        return;
    }
    out.push(egui::Shape::line_segment([a, b], Stroke::new(t, color)));
    if pen.cap == Cap::Round && t > 2.0 {
        out.push(egui::Shape::circle_filled(a, t / 2.0, color));
        out.push(egui::Shape::circle_filled(b, t / 2.0, color));
    }
}

/// How a layer looks in the preview: the pen of the layer, or a round black one of the width of the layer.
pub fn pen_of_layer(layer: &crate::layers::PenLayer, lib: &PenLibrary) -> Pen {
    if let Some(p) = layer.pen.as_deref().and_then(|id| lib.get(id)) {
        return p.clone();
    }
    Pen { id: String::new(), name: String::new(), kind: PenKind::Fineliner, width: layer.pen_width, height: layer.pen_width, angle: 0.0, color: layer.color, opacity: 1.0, cap: Cap::Round, notes: String::new(), builtin: true }
}

fn sample(ui: &mut egui::Ui, pen: &Pen) {
    let (rect, _) = ui.allocate_exact_size(egui::vec2(ui.available_width().min(520.0), 150.0), egui::Sense::hover());
    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, 4.0, Color32::WHITE);
    // Ten pixels to the millimetre, less for a very wide pen, more for a very thin one.
    let scale = (rect.height() / (pen.thickest() as f32 * 6.0 + 4.0)).clamp(2.0, 14.0).max(2.0 / pen.thinnest().max(0.01) as f32).min(40.0);
    let mut shapes = vec![];
    let c = rect.center();
    // Lines in four directions and a dot, to see how thick the pen is each way.
    for (i, deg) in [0.0f64, 45.0, 90.0, 135.0].iter().enumerate() {
        let x = rect.left() + 40.0 + i as f32 * 60.0;
        let r = deg.to_radians();
        let half = rect.height() * 0.32;
        let (dx, dy) = (half * r.cos() as f32, half * r.sin() as f32);
        // The line is drawn along its direction in the work area (Y up), so the screen Y is turned over.
        mark_shapes(Pos2::new(x - dx, c.y + dy), Pos2::new(x + dx, c.y - dy), *deg as f64 * std::f64::consts::PI / 180.0, pen, scale, &mut shapes);
    }
    mark_shapes(Pos2::new(rect.right() - 40.0, c.y), Pos2::new(rect.right() - 40.0, c.y), 0.0, pen, scale, &mut shapes);
    painter.extend(shapes);
}

fn row(ui: &mut egui::Ui, label: &str, editable: bool, add: impl FnOnce(&mut egui::Ui) -> bool) -> bool {
    ui.label(label);
    let mut changed = false;
    ui.add_enabled_ui(editable, |ui| changed = add(ui));
    ui.end_row();
    changed
}

/// The window of the pen library. `lib` is changed in place; the caller keeps it when `win.changed` is set.
pub fn pens_window(ctx: &egui::Context, win: &mut PensWindow, lib: &mut PenLibrary) {
    if !win.open {
        return;
    }
    let mut open = true;
    popup(ctx, "pen_library", &tr("Pen library"), [820.0, 560.0], &mut open, |ui| {
        if win.selected.is_none() {
            if let Some(p) = lib.builtin.first() {
                let id = p.id.clone();
                win.select(lib, &id);
            }
        }
        let total = base_ui::window::fill_size(ui).y;
        egui::ScrollArea::vertical().id_salt("pen_window").max_height(total).auto_shrink([false, true]).show(ui, |ui| {
            ui.horizontal_top(|ui| {
                ui.vertical(|ui| {
                    ui.set_width(240.0);
                    // The list fills the window; the buttons under it stay at its foot.
                    let list_h = base_ui::window::height_above(ui, "pen_left", total);
                    egui::ScrollArea::vertical().id_salt("pen_list").max_height(list_h).min_scrolled_height(list_h).auto_shrink([false, false]).show(ui, |ui| {
                        let mut pick = None;
                        for (title, user) in [(tr("Ready-made pens"), false), (tr("My pens"), true)] {
                            ui.label(egui::RichText::new(title).strong().color(theme::text_dim()));
                            let list: Vec<&Pen> = if user { lib.user.iter().collect() } else { lib.builtin.iter().collect() };
                            if list.is_empty() {
                                ui.label(egui::RichText::new(tr("Nothing here yet")).color(theme::text_dim()));
                            }
                            for p in list {
                                let on = win.selected.as_deref() == Some(p.id.as_str());
                                if ui.add_sized([ui.available_width(), 22.0], egui::Button::selectable(on, &p.name)).clicked() {
                                    pick = Some(p.id.clone());
                                }
                            }
                            ui.add_space(6.0);
                        }
                        if let Some(id) = pick {
                            win.select(lib, &id);
                        }
                    });
                    base_ui::window::bottom(ui, "pen_left", |ui| {
                        ui.add_space(4.0);
                        ui.horizontal_wrapped(|ui| {
                            if ui.button(tr("New pen")).clicked() {
                                let p = Pen { id: String::new(), name: lib.free_name(&tr("New pen")), kind: PenKind::Fineliner, width: 0.4, height: 0.4, angle: 0.0, color: [0x10, 0x10, 0x10], opacity: 1.0, cap: Cap::Round, notes: String::new(), builtin: false };
                                if let Ok(id) = lib.add(p) {
                                    win.changed = true;
                                    win.select(lib, &id);
                                }
                            }
                            if ui.add_enabled(win.selected.is_some(), egui::Button::new(tr("Copy"))).on_hover_text(tr("A copy among your own pens")).clicked() {
                                if let Some(mut c) = win.selected.as_deref().and_then(|id| lib.get(id)).cloned() {
                                    c.name = lib.free_name(&c.name);
                                    if let Ok(id) = lib.add(c) {
                                        win.changed = true;
                                        win.select(lib, &id);
                                    }
                                }
                            }
                            let mine = win.selected.as_deref().is_some_and(|id| lib.user.iter().any(|p| p.id == id));
                            if ui.add_enabled(mine, egui::Button::new(tr("Delete"))).clicked() {
                                if let Some(id) = win.selected.take() {
                                    lib.delete(&id);
                                    win.draft = None;
                                    win.changed = true;
                                }
                            }
                        });
                        ui.horizontal_wrapped(|ui| {
                            if ui.button(tr("Import…")).on_hover_text(tr("Read pens from a file")).clicked() {
                                if let Some(p) = rfd::FileDialog::new().add_filter("PenCreator pens", &["json"]).pick_file() {
                                    win.message = match std::fs::metadata(&p) {
                                        Ok(m) if m.len() > crate::pens::MAX_FILE => tr("The file is too large").to_string(),
                                        _ => match std::fs::read_to_string(&p).map_err(|e| e.to_string()).and_then(|t| lib.import(&t)) {
                                            Ok((n, problems)) => {
                                                win.changed |= n > 0;
                                                let mut m = format!("{} {}", n, tr("pens added"));
                                                if !problems.is_empty() {
                                                    m.push_str(&format!(". {}: {}", tr("Left out"), problems.join("; ")));
                                                }
                                                m
                                            }
                                            Err(e) => format!("{}: {e}", tr("Cannot use the file")),
                                        },
                                    };
                                }
                            }
                            if ui.add_enabled(!lib.user.is_empty(), egui::Button::new(tr("Export…"))).on_hover_text(tr("Save your pens to a file")).clicked() {
                                if let Some(p) = rfd::FileDialog::new().add_filter("PenCreator pens", &["json"]).set_file_name("pens.json").save_file() {
                                    win.message = match std::fs::write(&p, lib.export(None)) {
                                        Ok(()) => format!("{} {}", tr("Saved"), p.display()),
                                        Err(e) => format!("{}: {e}", tr("Could not save")),
                                    };
                                }
                            }
                            if ui.add_enabled(win.selected.is_some(), egui::Button::new(tr("Export this pen…"))).clicked() {
                                if let (Some(id), Some(p)) = (win.selected.clone(), rfd::FileDialog::new().add_filter("PenCreator pens", &["json"]).set_file_name("pen.json").save_file()) {
                                    win.message = match std::fs::write(&p, lib.export(Some(&[id]))) {
                                        Ok(()) => format!("{} {}", tr("Saved"), p.display()),
                                        Err(e) => format!("{}: {e}", tr("Could not save")),
                                    };
                                }
                            }
                        });
                        if !win.message.is_empty() {
                            ui.label(egui::RichText::new(&win.message).color(theme::text_dim()));
                        }
                    });
                });
                ui.separator();
                ui.vertical(|ui| {
                    let Some(draft) = win.draft.as_mut() else {
                        ui.label(egui::RichText::new(tr("Choose a pen")).color(theme::text_dim()));
                        return;
                    };
                    let editable = !draft.builtin;
                    sample(ui, draft);
                    ui.add_space(4.0);
                    if !editable {
                        ui.label(egui::RichText::new(tr("A ready-made pen cannot be changed. Make a copy to change it.")).color(theme::text_dim()));
                    }
                    let mut changed = false;
                    egui::Grid::new("pen_grid").num_columns(2).spacing([10.0, 6.0]).show(ui, |ui| {
                        changed |= row(ui, &tr("Name"), editable, |ui| ui.text_edit_singleline(&mut draft.name).changed());
                        changed |= row(ui, &tr("Type"), editable, |ui| {
                            let mut c = false;
                            egui::ComboBox::from_id_salt("pen_kind").selected_text(tr(draft.kind.label())).width(220.0).show_ui(ui, |ui| {
                                for k in PenKind::ALL {
                                    c |= ui.selectable_value(&mut draft.kind, k, tr(k.label())).changed();
                                }
                            });
                            c
                        });
                        changed |= row(ui, &tr("Width of the mark"), editable, |ui| ui.add(egui::DragValue::new(&mut draft.width).speed(0.05).range(0.01..=50.0).suffix(" mm")).changed());
                        changed |= row(ui, &tr("Height of the mark"), editable, |ui| ui.add(egui::DragValue::new(&mut draft.height).speed(0.05).range(0.01..=50.0).suffix(" mm")).changed());
                        changed |= row(ui, &tr("Turn of the tip"), editable, |ui| ui.add(egui::DragValue::new(&mut draft.angle).range(-360.0..=360.0).suffix(" °")).changed());
                        changed |= row(ui, &tr("Ends of a line"), editable, |ui| {
                            let mut c = false;
                            egui::ComboBox::from_id_salt("pen_cap").selected_text(tr(draft.cap.label())).width(120.0).show_ui(ui, |ui| {
                                for k in Cap::ALL {
                                    c |= ui.selectable_value(&mut draft.cap, k, tr(k.label())).changed();
                                }
                            });
                            c
                        });
                        changed |= row(ui, &tr("Ink colour"), editable, |ui| ui.color_edit_button_srgb(&mut draft.color).changed());
                        changed |= row(ui, &tr("Opacity"), editable, |ui| {
                            let mut pct = draft.opacity * 100.0;
                            let c = ui.add(egui::DragValue::new(&mut pct).range(5.0..=100.0).suffix(" %")).changed();
                            draft.opacity = pct / 100.0;
                            c
                        });
                    });
                    if changed && editable && lib.replace(draft.clone()).is_ok() {
                        win.changed = true;
                    }
                });
            });
        });
    });
    win.open = open;
    let _ = &mut win.changed;
}

/// The pen of a layer in the Layers tab: a choice from the library, the window of the library, and the size of the mark.
/// Returns true when the layer changed.
pub fn layer_pen_row(ui: &mut egui::Ui, layer: &mut crate::layers::PenLayer, lib: &PenLibrary, open_library: &mut bool) -> bool {
    let mut changed = false;
    let current = layer.pen.as_deref().and_then(|id| lib.get(id)).map(|p| p.name.clone()).unwrap_or_else(|| tr("No pen chosen").to_string());
    ui.horizontal(|ui| {
        egui::ComboBox::from_id_salt("layer_pen").selected_text(current).width(200.0).show_ui(ui, |ui| {
            if ui.selectable_label(layer.pen.is_none(), tr("No pen chosen")).clicked() && layer.pen.is_some() {
                layer.pen = None;
                changed = true;
            }
            for p in lib.all() {
                if ui.selectable_label(layer.pen.as_deref() == Some(p.id.as_str()), &p.name).clicked() {
                    layer.use_pen(p);
                    changed = true;
                }
            }
        });
        if ui.button("…").on_hover_text(tr("Pen library")).clicked() {
            *open_library = true;
        }
    });
    changed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_layer_without_a_pen_looks_like_a_round_pen_of_its_width_and_a_chosen_pen_changes_it() {
        let lib = PenLibrary::new();
        let mut l = crate::layers::default_layers().remove(0);
        l.pen_width = 0.7;
        let p = pen_of_layer(&l, &lib);
        assert_eq!((p.width, p.height, p.opacity), (0.7, 0.7, 1.0));
        let marker = lib.builtin.iter().find(|p| p.kind == PenKind::Highlighter).unwrap().clone();
        l.use_pen(&marker);
        let p = pen_of_layer(&l, &lib);
        assert_eq!(p.id, marker.id);
        assert!(p.opacity < 1.0 && l.pen_width == marker.thickest());
    }

    #[test]
    fn a_line_is_as_thick_as_the_pen_makes_it_and_a_dot_is_one_shape() {
        let lib = PenLibrary::new();
        let mut out = vec![];
        let pen = lib.builtin.iter().find(|p| p.kind == PenKind::Pencil).unwrap();
        mark_shapes(Pos2::new(0.0, 0.0), Pos2::new(50.0, 0.0), 0.0, pen, 10.0, &mut out);
        assert!(!out.is_empty());
        let mut dot = vec![];
        mark_shapes(Pos2::new(5.0, 5.0), Pos2::new(5.0, 5.0), 0.0, pen, 10.0, &mut dot);
        assert_eq!(dot.len(), 1);
    }

    #[test]
    fn the_window_draws_and_edits_a_copy() {
        let ctx = egui::Context::default();
        crate::texts::register();
        let mut lib = PenLibrary::new();
        let mut win = PensWindow { open: true, ..Default::default() };
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| pens_window(ui.ctx(), &mut win, &mut lib));
        out.textures_delta.clear();
        assert!(win.selected.is_some() && win.draft.as_ref().unwrap().builtin);
        let mut l = crate::layers::default_layers().remove(0);
        let mut open = false;
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                layer_pen_row(ui, &mut l, &lib, &mut open);
            });
        });
        out.textures_delta.clear();
    }
}

//! The Layers tab: one pen per layer.
use crate::layers::{Draw, MAX_PASSES};
use crate::App;
use base_ui::editor::EditorHost;
use base_ui::i18n::{tr, trf};
use base_ui::theme;
use base_ui::units_ui::drag_len;
use eframe::egui::{self, RichText};

impl App {
    /// How many shapes lie on each layer.
    pub(crate) fn shapes_per_layer(&self) -> Vec<usize> {
        let mut n = vec![0; self.layers.len()];
        for s in &self.scene.shapes {
            if let Some(c) = n.get_mut(s.layer) {
                *c += 1;
            }
        }
        n
    }

    /// Put the selected shapes on a layer.
    pub(crate) fn assign_selection_to_layer(&mut self, layer: usize) {
        let ids = self.scene.unlocked_ids(&self.ed.sel.iter().copied().collect::<Vec<_>>());
        if ids.is_empty() {
            return;
        }
        self.checkpoint();
        for id in ids {
            if let Some(s) = self.scene.shape_mut(id) {
                s.layer = layer;
            }
        }
        self.touch();
    }

    pub(crate) fn layers_panel(&mut self, ui: &mut egui::Ui) {
        let counts = self.shapes_per_layer();
        let active = self.ed.active_layer.min(self.layers.len().saturating_sub(1));
        let mut pick = None;
        let mut changed = false;
        let show_all = self.show_all_layers;
        let mut move_req: Option<(usize, bool)> = None;
        let order = crate::layers::layer_order(&self.layers);
        base_ui::widgets::list_with_splitter(ui, "layer_list", 190.0, |ui| {
            for i in order {
                // Of 64 layers show the ones with shapes and the one being edited (the colour bar has all of them).
                if !(show_all || counts[i] > 0 || i == active) {
                    continue;
                }
                let l = &mut self.layers[i];
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 3.0;
                    // The layers are done in the order of this list: from the top down.
                    if ui.add(egui::Button::image(base_ui::icons::slot(Some("chevron-up"), theme::text())).small()).on_hover_text(tr("Earlier in the job")).clicked() {
                        move_req = Some((i, true));
                    }
                    if ui.add(egui::Button::image(base_ui::icons::slot(Some("chevron-down"), theme::text())).small()).on_hover_text(tr("Later in the job")).clicked() {
                        move_req = Some((i, false));
                    }
                    let (rect, _) = ui.allocate_exact_size(egui::vec2(14.0, 14.0), egui::Sense::hover());
                    ui.painter().rect_filled(rect, 3.0, l.color32());
                    changed |= ui.checkbox(&mut l.enabled, "").on_hover_text(tr("Drawn: taken into the plot and the G-code")).changed();
                    if base_ui::widgets::toggle_icon(ui, "eye", &tr("Shown in the work area"), l.visible) {
                        l.visible = !l.visible;
                        changed = true;
                    }
                    let text = format!("{}  ·  {}", l.name, tr(l.draw.label()));
                    let r = ui.add_sized([ui.available_width() - 44.0, 22.0], egui::Button::selectable(i == active, text));
                    if r.clicked() {
                        pick = Some(i);
                    }
                    ui.label(RichText::new(counts[i].to_string()).color(theme::text_dim())).on_hover_text(tr("Shapes on this layer"));
                });
            }
        });
        if let Some((i, up)) = move_req {
            crate::layers::move_layer(&mut self.layers, i, up);
            changed = true;
        }
        ui.checkbox(&mut self.show_all_layers, trf("Show all {} layers", &[&self.layers.len()]));
        if let Some(i) = pick {
            self.ed.active_layer = i;
        }
        let li = self.ed.active_layer.min(self.layers.len().saturating_sub(1));
        let units = self.device.units;
        let mut assign = false;
        {
            let l = &mut self.layers[li];
            egui::ScrollArea::vertical().id_salt("layer_edit").auto_shrink([false, false]).show(ui, |ui| {
                egui::Grid::new("layer_grid").num_columns(2).spacing([10.0, 6.0]).show(ui, |ui| {
                    ui.label(tr("Name"));
                    changed |= ui.text_edit_singleline(&mut l.name).changed();
                    ui.end_row();
                    ui.label(tr("Colour"));
                    changed |= ui.color_edit_button_srgb(&mut l.color).changed();
                    ui.end_row();
                    ui.label(tr("Drawing"));
                    egui::ComboBox::from_id_salt("layer_draw").selected_text(tr(l.draw.label())).width(200.0).show_ui(ui, |ui| {
                        for d in Draw::ALL {
                            changed |= ui.selectable_value(&mut l.draw, d, tr(d.label())).on_hover_text(tr(d.hint())).changed();
                        }
                    });
                    ui.end_row();
                    ui.label(tr("Pen width"));
                    changed |= drag_len(ui, units, &mut l.pen_width, 0.05, Some((0.01, 20.0))).changed();
                    ui.end_row();
                    ui.label(tr("Speed"));
                    changed |= ui.add(egui::DragValue::new(&mut l.speed).range(1.0..=100000.0).suffix(" mm/min")).changed();
                    ui.end_row();
                    if l.draw.fills() {
                        ui.label(tr("Fill line gap"));
                        changed |= drag_len(ui, units, &mut l.hatch_gap, 0.05, Some((0.02, 100.0))).changed();
                        ui.end_row();
                        ui.label(tr("Fill line angle"));
                        changed |= ui.add(egui::DragValue::new(&mut l.hatch_angle).range(-360.0..=360.0).suffix(" °")).changed();
                        ui.end_row();
                        ui.label(tr("Crossed lines"));
                        changed |= ui.checkbox(&mut l.cross_hatch, "").on_hover_text(tr("A second set of fill lines across the first")).changed();
                        ui.end_row();
                    }
                    ui.label(tr("Drawn times"));
                    changed |= ui.add(egui::DragValue::new(&mut l.passes).range(1..=MAX_PASSES)).on_hover_text(tr("How many times every line is drawn: for a pen that is too light")).changed();
                    ui.end_row();
                });
                ui.add_space(4.0);
                ui.label(RichText::new(tr(l.draw.hint())).color(theme::text_dim()));
                ui.add_space(6.0);
                if ui.button(trf("Put the selected shapes on this layer", &[])).clicked() {
                    assign = true;
                }
                ui.label(RichText::new(tr("A layer is a pen: when the next layer has another pen, the G-code stops so that you can change it.")).color(theme::text_dim()));
            });
        }
        if assign {
            self.assign_selection_to_layer(li);
        }
        if changed {
            self.touch();
        }
    }
}

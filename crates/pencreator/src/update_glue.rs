//! Looking for a newer release on GitHub: once when the program starts (can be switched off in the program options) and on
//! demand from Help > Check for updates…. The check and its window are in base-ui; this says which program they are about.
use crate::{App, VERSION};
use base_ui::dialogs::UpdateConfig;
use base_ui::i18n::tr;
use eframe::egui;

pub static CONFIG: UpdateConfig = UpdateConfig { app_name: "PenCreator", repo: "michalsarna/PenCreator", version: VERSION };

impl App {
    /// Settings → Program options…: what the program does by itself.
    pub(crate) fn program_options_window(&mut self, ctx: &egui::Context) {
        let mut open = self.show_program_options;
        // Cancel gives back the options as they were when the window opened.
        let now = (self.check_updates, self.autosave_opts, self.ed.grid_prefs.units);
        let end = base_ui::options_ui::window(ctx, "program_options", [460.0, 520.0], &mut open, now, |ui| {
            use base_ui::widgets::{options_grid, options_line};
            options_grid(ui, "program_options_updates", |ui| {
                ui.label(tr("Updates"));
                ui.checkbox(&mut self.check_updates, tr("Look for a newer version when the program starts"));
                ui.end_row();
            });
            options_line(ui);
            options_grid(ui, "program_options_autosave", |ui| base_ui::autosave_ui::options_rows(ui, &mut self.autosave_opts));
            options_line(ui);
            options_grid(ui, "program_options", |ui| {
                base_ui::units_ui::display_units_row(ui, &mut self.ed.grid_prefs.units);
                self.web.options_ui(ui, true);
                ui.end_row();
            });
            base_ui::authorship_ui::options_section(ui);
            // The typeface and size of the G-code tab, the same in every program.
            base_ui::gcode_text_ui::options_section(ui);
        });
        if let base_ui::options_ui::OptionsEnd::Cancelled((check_updates, autosave_opts, units)) = end {
            self.check_updates = check_updates;
            self.autosave_opts = autosave_opts;
            self.ed.grid_prefs.units = units;
        }
        self.show_program_options = open;
    }
}

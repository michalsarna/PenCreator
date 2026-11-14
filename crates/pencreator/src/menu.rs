//! What the menus of PenCreator can do and how they are laid out. The standard editing commands come from base-ui.
use base_ui::i18n::Lang;
use base_ui::menu::{key, machine_menu, overlay_entries, standard_menus, Entry, MachineMenu, MenuParts, Menus, StdAct};
use base_ui::theme::Scheme;

/// How many recent files the File menu keeps.
pub const MAX_RECENT: usize = 10;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Act {
    New,
    Open,
    Save,
    SaveAs,
    ImportLightburn,
    ImportSvg,
    ImportAi,
    ImportDxf,
    ImportUgsd,
    ImportPlugin,
    ViewGcode,
    Plugins,
    Pens,
    Knives,
    GenerateShape,
    OnlineClipart,
    Preview,
    /// The preview window shown or hidden, as the Preview… button does (View menu).
    TogglePreview,
    Firmware,
    DeviceSettings,
    Calibration,
    Diagnostics,
    ControlDevices,
    ControlDeviceOfMachine,
    StartFrom,
    CheckProgram,
    ExportGcode,
    ExportSvg,
    ExportCam,
    OpenRecent(u8),
    ClearRecent,
    Quit,
    Std(StdAct),
    ViewOptions,
    ProgramOptions,
    CheckUpdates,
    SetLang(Lang),
    SetScheme(Scheme),
    OpenGithub,
    DownloadBackup,
    RestoreBackup,
    About,
    Print,
    QuickGuide,
    ExportDiagnostics,
    SwitchDevice,
    ShareSer2net,
    Frame,
    StartJob,
    ToggleOverlay,
    /// Draw the no-go zones on the work area.
    ToggleZones,
    /// File → Project setup…: the sheet, a cylinder, a two-sided drawing.
    ProjectSetup,
    /// The device settings, on their page of the no-go zones.
    ZoneSettings,
    PreviewGcode,
    LayerPresets,
}

impl From<StdAct> for Act {
    fn from(a: StdAct) -> Act {
        Act::Std(a)
    }
}

/// The menus: the layout of baseCreator (`standard_menus`) with the entries of PenCreator in their places. The macOS app menu (About,
/// Quit) is added by `native_menu`.
pub fn menus() -> Menus<Act> {
    use Entry::{Item, Sep};
    standard_menus(MenuParts {
        new: Act::New,
        open: Act::Open,
        save: Act::Save,
        save_as: Act::SaveAs,
        quit: Act::Quit,
        about: Act::About,
        view_gcode: Act::ViewGcode,
        generate: Act::GenerateShape,
        import_plugin: Act::ImportPlugin,
        online_clipart: Act::OnlineClipart,
        plugins: Act::Plugins,
        view_options: Act::ViewOptions,
        program_options: Act::ProgramOptions,
        set_lang: Act::SetLang,
        set_scheme: Act::SetScheme,
        github: Act::OpenGithub,
        download_backup: Act::DownloadBackup,
        restore_backup: Act::RestoreBackup,
        check_updates: Act::CheckUpdates,
        file_extra: vec![Item(Act::ProjectSetup, "Project setup…", None)],
        import_svg: Act::ImportSvg,
        import_ai: Act::ImportAi,
        import_dxf: Act::ImportDxf,
        import_lightburn: Act::ImportLightburn,
        import_ugsd: Act::ImportUgsd,
        export_svg: Act::ExportSvg,
        export_cam: Act::ExportCam,
        import_extra: vec![],
        export_first: vec![Item(Act::ExportGcode, "Export G-code…", None)],
        export_extra: vec![],
        output_extra: vec![Item(Act::Print, "Print…", key("CmdOrCtrl+P", "Ctrl+P"))],
        view_extra: [overlay_entries(Act::ToggleOverlay, Act::Std(StdAct::BackgroundPicture)), vec![Item(Act::ToggleZones, "Show no-go zones", None)],
            // What the buttons of the main window show: the preview and the G-code.
            vec![Sep, Item(Act::TogglePreview, "Preview", None), Item(Act::PreviewGcode, "Preview G-code", None)],
        ]
        .concat(),
        background: false,
        machine: machine_menu(MachineMenu {
            switch_device: Act::SwitchDevice,
            device_settings: Act::DeviceSettings,
            firmware: Act::Firmware,
            calibration: Act::Calibration,
            own: vec![Item(Act::ZoneSettings, "No-go zones…", None)],
            control_devices: Act::ControlDevices,
            control_device_of_machine: Act::ControlDeviceOfMachine,
            diagnostics: Act::Diagnostics,
            share: Act::ShareSer2net,
            frame: Act::Frame,
            start_job: Act::StartJob,
            start_from: Act::StartFrom,
            check_program: Act::CheckProgram,
        }),
        process: ("Plotter", vec![Item(Act::Pens, "Pen library…", None), Item(Act::Knives, "Knife library…", None), Item(Act::LayerPresets, "Layer presets…", None), Sep, Item(Act::PreviewGcode, "Preview G-code", None), Item(Act::Preview, "Plot of the job…", key("CmdOrCtrl+Shift+P", "Ctrl+Shift+P")), Item(Act::ExportGcode, "Export G-code…", key("CmdOrCtrl+E", "Ctrl+E"))]),
        settings_extra: vec![],
        help_first: vec![Item(Act::QuickGuide, "Quick guide", key("F1", "F1"))],
        help_extra: vec![Item(Act::ExportDiagnostics, "Export diagnostics (zip)…", None)],
        zones: true,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every key of the menus belongs to one entry only (Ctrl+Shift+G was Ungroup and Export G-code at once), and the menus have the
    /// layout of all the programs.
    #[test]
    fn no_two_entries_of_the_menus_share_a_key_and_the_layout_is_the_common_one() {
        let m = menus();
        assert!(base_ui::menu::key_clashes(&m).is_empty(), "{:?}", base_ui::menu::key_clashes(&m));
        let titles: Vec<&str> = m.iter().map(|(t, _)| *t).collect();
        assert_eq!(&titles[..6], ["File", "Edit", "Insert", "Arrange", "View", "Machine"]);
        assert_eq!(&titles[7..], ["Settings", "Help"]);
        // View ends with the preview and the G-code, as the buttons of the main window.
        let view = &m.iter().find(|(t, _)| *t == "View").unwrap().1;
        let items: Vec<Act> = view.iter().filter_map(|e| match e { Entry::Item(a, ..) => Some(*a), _ => None }).collect();
        assert_eq!(items.iter().rev().take(2).copied().collect::<Vec<_>>(), [Act::PreviewGcode, Act::TogglePreview]);
    }
}

🌐 **English** · [Polski](Architektura)

# Architecture

PenCreator is one binary crate, `crates/pencreator`, on top of **baseCreator** (`base-core`: geometry, the scene, the SVG / PDF / DXF / LightBurn / UGS readers, the project container, the G-code helpers, the machine link; `base-ui`: the window, the menus, the editor, the panes and tabs, clipart, camera, the Control tab, the G-code viewer and the G-code text tab, plugins). baseCreator is pinned by a git tag.

| File | What it holds |
|---|---|
| `main.rs` | The application: open, save, the start screen, the bars, the side panel, the keys, the windows. |
| `menu.rs` | The entries of the menus (the layout is `standard_menus` of base-ui) and a test that no two entries share a key. |
| `native_menu.rs` | The macOS menu bar. |
| `workspaces.rs` | The open drawings as tabs. |
| `panes_glue.rs` | The panes of the work area and the G-code text tab of a project. |
| `device.rs`, `device_ui.rs` | The plotters, the start screen list, the device settings window, the Device and Camera tabs. |
| `layers.rs`, `layers_ui.rs` | The layers (pens) and the Layers tab. |
| `layer_presets.rs` | What a layer preset holds. |
| `pens.rs`, `pens_ui.rs` | The pen library and its window, and how a pen draws a line in the preview. |
| `plot.rs` | The strokes, the fill, the pictures drawn with lines, the order, the pen changes and the G-code. |
| `preview_ui.rs` | The preview, the toolpaths, Preview G-code and Export G-code. |
| `machine_ui.rs` | The link to the machine: connect, start, frame, check, the pen buttons. |
| `section.rs` | The `pen` section of the project file. |
| `ugs_import.rs` | The layers for a UGS Designer design. |
| `gcode_view_glue.rs`, `plugins_glue.rs`, `autosave_glue.rs`, `backup_ui.rs`, `update_glue.rs` | The G-code viewer, plugins, automatic saving, backup and the update check, joined to the program. |
| `texts.rs`, `warn.rs` | PenCreator's own translations and the warnings of the program. |
| `screenshots.rs` | The renderer of the pictures (`cargo test -p pencreator render_screenshots -- --ignored`). |
| `grbl_sim_tests.rs` | Tests of the jobs against GRBL 1.1h compiled for the PC (grbl-sim). |

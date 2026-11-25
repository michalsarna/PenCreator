🌐 [English](Architecture) · [Polski](Architektura) · **简体中文** · [한국어](Architecture-KO)

> ⚠️ 本页由英文版机器翻译而成，可能有不准确之处。如有疑问，请以[英文版](Architecture)为准。欢迎提出修改建议。

# 架构

PenCreator 是一个二进制 crate `crates/pencreator`，构建在 **baseCreator** 之上（`base-core`：几何、场景、SVG / PDF / DXF / LightBurn / UGS 读取器、项目容器、G-code 辅助工具、机器连接；`base-ui`：窗口、菜单、编辑器、窗格和标签页、剪贴画、摄像头、控制标签页、G-code 查看器和 G-code 文本标签页、插件）。baseCreator 通过 git 标签固定版本。

| 文件 | 内容 |
|---|---|
| `main.rs` | 应用程序：打开、保存、开始屏幕、各种栏、侧面板、快捷键、窗口。 |
| `menu.rs` | 菜单项（布局来自 base-ui 的 `standard_menus`），以及检查任何两个菜单项都不共用快捷键的测试。 |
| `native_menu.rs` | macOS 菜单栏。 |
| `workspaces.rs` | 作为标签页打开的绘图。 |
| `panes_glue.rs` | 工作区的窗格以及项目的 G-code 文本标签页。 |
| `device.rs`、`device_ui.rs` | 绘图仪、开始屏幕列表、设备设置窗口、设备和摄像头标签页。 |
| `layers.rs`、`layers_ui.rs` | 图层（笔）和图层标签页。 |
| `layer_presets.rs` | 图层预设包含的内容。 |
| `pens.rs`、`pens_ui.rs` | 笔库及其窗口，以及笔在预览中如何绘制线条。 |
| `plot.rs` | 笔画、填充、用线条绘制的图片、顺序、换笔和 G-code。 |
| `preview_ui.rs` | 预览、刀具路径、预览 G-code 和导出 G-code。 |
| `machine_ui.rs` | 与机器的连接：连接、开始、边框、检查、笔按钮。 |
| `section.rs` | 项目文件中的 `pen` 部分。 |
| `ugs_import.rs` | 为 UGS Designer 设计设置的图层。 |
| `gcode_view_glue.rs`、`plugins_glue.rs`、`autosave_glue.rs`、`backup_ui.rs`、`update_glue.rs` | G-code 查看器、插件、自动保存、备份和更新检查与程序的衔接。 |
| `texts.rs`、`warn.rs` | PenCreator 自己的翻译以及程序的警告。 |
| `screenshots.rs` | 截图渲染器（`cargo test -p pencreator render_screenshots -- --ignored`）。 |
| `grbl_sim_tests.rs` | 针对在 PC 上编译的 GRBL 1.1h（grbl-sim）测试作业。 |

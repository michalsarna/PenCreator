🌐 **English** · [Polski](Architektura)

# Architecture

PenCreator is one binary crate, `crates/pencreator`, on top of **baseCreator** (`base-core`: geometry, scene, SVG / PDF / LightBurn readers, the project container; `base-ui`: window, editor, clipart, camera, the Description tab). baseCreator is pinned by git tag. `main.rs` is the application (open, save, tabs, windows), `menu.rs` the menus, `native_menu.rs` the macOS menu bar and `screenshots.rs` the renderer of the pictures (`cargo test -p pencreator render_screenshots -- --ignored`).

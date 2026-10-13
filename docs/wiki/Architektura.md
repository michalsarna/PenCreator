🌐 [English](Architecture) · **Polski**

# Architektura

PenCreator to jedna skrzynka binarna, `crates/pencreator`, nad **baseCreatorem** (`base-core`: geometria, scena, czytniki SVG / PDF / LightBurna, kontener projektu; `base-ui`: okno, edytor, cliparty, kamera, karta Description). baseCreator jest przypięty tagiem git. `main.rs` to aplikacja (otwieranie, zapis, karty, okna), `menu.rs` menu, `native_menu.rs` pasek menu macOS, a `screenshots.rs` generator obrazów (`cargo test -p pencreator render_screenshots -- --ignored`).

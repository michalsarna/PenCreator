🌐 [English](Architecture) · **Polski**

# Architektura

PenCreator to jedna skrzynka binarna, `crates/pencreator`, nad **baseCreatorem** (`base-core`: geometria, scena, czytniki SVG / PDF / DXF / LightBurna / UGS, kontener projektu, pomoce do G-code, połączenie z maszyną; `base-ui`: okno, menu, edytor, panele i karty, cliparty, kamera, karta Sterowanie, przeglądarka G-code i karta z tekstem G-code, wtyczki). baseCreator jest przypięty tagiem git.

| Plik | Co zawiera |
|---|---|
| `main.rs` | Aplikacja: otwieranie, zapis, ekran startowy, paski, panel boczny, klawisze, okna. |
| `menu.rs` | Pozycje menu (układ to `standard_menus` z base-ui) i test, że żadne dwie pozycje nie mają tego samego klawisza. |
| `native_menu.rs` | Pasek menu macOS. |
| `workspaces.rs` | Otwarte rysunki jako karty. |
| `panes_glue.rs` | Panele obszaru roboczego i karta z tekstem G-code projektu. |
| `device.rs`, `device_ui.rs` | Plotery, lista na ekranie startowym, okno ustawień urządzenia, zakładki Urządzenie i Kamera. |
| `layers.rs`, `layers_ui.rs` | Warstwy (pisaki) i zakładka Warstwy. |
| `layer_presets.rs` | Co zawiera preset warstwy. |
| `pens.rs`, `pens_ui.rs` | Biblioteka pisaków i jej okno oraz to, jak pisak rysuje linię w podglądzie. |
| `plot.rs` | Pociągnięcia, wypełnienie, obrazy rysowane liniami, kolejność, zmiany pisaka i G-code. |
| `preview_ui.rs` | Podgląd, ścieżki, Podgląd G-code i Eksportuj G-code. |
| `machine_ui.rs` | Połączenie z maszyną: łączenie, start, obrys, sprawdzanie, przyciski pisaka. |
| `section.rs` | Sekcja `pen` pliku projektu. |
| `ugs_import.rs` | Warstwy dla projektu UGS Designera. |
| `gcode_view_glue.rs`, `plugins_glue.rs`, `autosave_glue.rs`, `backup_ui.rs`, `update_glue.rs` | Przeglądarka G-code, wtyczki, automatyczny zapis, kopia zapasowa i sprawdzanie aktualizacji, połączone z programem. |
| `texts.rs`, `warn.rs` | Własne tłumaczenia PenCreatora i ostrzeżenia programu. |
| `screenshots.rs` | Generator obrazów (`cargo test -p pencreator render_screenshots -- --ignored`). |
| `grbl_sim_tests.rs` | Testy zadań na GRBL 1.1h skompilowanym na komputer (grbl-sim). |

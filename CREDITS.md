# Credits

PenCreator is built on [baseCreator](https://github.com/michalsarna/baseCreator) and, through it, on the projects
listed in its [CREDITS.md](https://github.com/michalsarna/baseCreator/blob/main/CREDITS.md). Directly:

* **[egui / eframe](https://github.com/emilk/egui)** by Emil Ernerfeldt and contributors (MIT / Apache-2.0): the interface toolkit.
* **[rfd](https://github.com/PolyMeilex/rfd)** (MIT): native file dialogs.
* **[GitHub Actions](https://github.com/features/actions)** and the Rust toolchain action by dtolnay: building and testing.
* **[Inkscape](https://inkscape.org)** (GPL-2.0-or-later): renders the icon from its SVG (`tools/make-icons.sh`); the program does not contain it. The pencil icon was drawn for PenCreator.
* **[Lucide](https://lucide.dev/)** icons (ISC) and **[Noto fonts](https://fonts.google.com/noto)** (SIL OFL 1.1), through baseCreator.
* **[Material Design Icons](https://github.com/Templarian/MaterialDesign-SVG)** by Pictogrammers (Apache-2.0), through baseCreator: the pictures of the clipart gallery.
* **[resvg / usvg](https://github.com/linebender/resvg)** (MPL-2.0), **[lopdf](https://github.com/J-F-Liu/lopdf)** (MIT) and **[i_overlay](https://github.com/iShape-Rust/iOverlay)** (MIT), through baseCreator: reading SVG and PDF files and the geometry of shapes.
* **[LightBurn](https://lightburnsoftware.com/)**: the format of its projects is read for import; PenCreator contains none of its code.
* **[Universal G-code Sender](https://github.com/winder/Universal-G-Code-Sender)**: the format of its Designer files (`.ugsd`) is read for import; PenCreator contains none of its code.
* **[muda](https://github.com/tauri-apps/muda)** and **[objc2](https://github.com/madsmtm/objc2)**, through baseCreator: the native macOS menu bar.

## Packaging and build services

* **[cargo-bundle](https://github.com/burtonageo/cargo-bundle)** (macOS app bundle), **[cargo-deb](https://github.com/kornelski/cargo-deb)**
  (Debian packages) and **[NSIS](https://nsis.sourceforge.io/)** (Windows installer).
* **[GitHub Actions](https://github.com/features/actions)**, the Rust toolchain action by dtolnay, **rust-cache** by Swatinem
  and **CodeQL**: building, testing and releasing.
* **Jakub ([CNC KpK](https://www.youtube.com/@CNC-KpK))**: ideas and suggestions for improvements, from the point of view of industrial CNC machines.

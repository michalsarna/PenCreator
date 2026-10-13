<p align="center"><img src="assets/icons/icon-256.png" alt="PenCreator icon" width="128"></p>

# PenCreator

Open-source pen plotter design and control software, written in Rust. **Early skeleton: there are no plotting features yet.**

It shares its core and its interface code with [LightCreator](https://github.com/michalsarna/LightCreator) (laser) and
[MillCreator](https://github.com/michalsarna/MillCreator) (CNC mill) through [baseCreator](https://github.com/michalsarna/baseCreator),
pinned to a tag in `crates/pencreator/Cargo.toml`.

## What it does today

It looks like its siblings, because the window and its parts come from baseCreator:

![The start screen](docs/screenshots/01-start.png)
![An SVG file open in the editor](docs/screenshots/02-editor-properties.png)
![The clipart gallery](docs/screenshots/03-clipart.png)

* The window has its own flat frame and title bar, a start screen (new design, open a file, recent files, language and
  colour scheme), a menu bar (File, Edit, Arrange, View, Settings, Help; the native menu bar on macOS), a control bar,
  a tool palette on the left, an arrange bar and the Properties, Clipart and Description tabs on the right, and a status bar.
* The clipart gallery of baseCreator: 310 included pictures in 9 categories, search, your own pictures, and online search of open icon collections; a picture is placed as ordinary shapes.
* 30 languages for the shared texts and the colour schemes of LightCreator (Settings menu); the choice is kept.
* Opens an SVG, PDF or Illustrator (PDF-compatible) file, or a `.cre` project (File menu, drag and drop, or the path on the command line), imports a LightBurn project (File → Import: `.lbrn2` shapes, text outlines and bitmaps), and saves projects as `.cre`. The project file is shared with LightCreator, MillCreator and other programs on baseCreator: it holds the drawing, the description and one section per program. PenCreator keeps the sections of other programs untouched when it saves, so a project made for a laser or a mill can be opened and saved here without losing its settings.
* Draws and edits shapes on a work area (an A4 sheet, 297 × 210 mm) with the editor of baseCreator: select, node edit, rectangle, ellipse, triangle,
  star, polygon, heart, spiral, line, pen, pan and zoom; selection handles to move, resize, turn and slant; the usual
  commands (undo and redo, select all, copy, paste, duplicate, delete, group, ungroup); grid and snapping (View menu and View options…).
  Keyboard: tool letters as in LightCreator (V, N, R, E, ...), Ctrl+Z / Y, Ctrl+A / C / V / D / G, Delete.

There are no pen layers, no plotting paths, no G-code or HPGL and no machine control yet; see [FEATURES.md](FEATURES.md).
The work area is fixed.

## Install

Ready-made packages for Windows, macOS and Linux are on the [Releases page](https://github.com/michalsarna/PenCreator/releases); see [INSTALL.md](INSTALL.md). Documentation (English and Polish) is in the [wiki](https://github.com/michalsarna/PenCreator/wiki).

## Building

```sh
cargo run -p pencreator                       # start empty
cargo run -p pencreator -- drawing.svg        # open a file
cargo test --workspace
```

Add `--features camera` for live camera capture. On Linux the GUI needs the usual X11/Wayland and OpenGL libraries
(`libxkbcommon-x11`, `libgl1`).

To work on baseCreator and PenCreator together, point Cargo at your checkout of baseCreator (do not commit this file):

```toml
# .cargo/config.toml
[patch."https://github.com/michalsarna/baseCreator"]
base-core = { path = "../baseCreator/crates/base-core" }
base-ui   = { path = "../baseCreator/crates/base-ui" }
```

## Icon

`assets/icons/pencreator.svg` is the icon: the orange background and the brown base of the baseCreator icon, with a pencil
drawing a line on the base. `tools/make-icons.sh assets/icons/pencreator.svg pencreator` renders the PNGs, the `.ico` and
the `.icns` (needs inkscape; the `.icns` needs macOS). The screenshots are made by
`cargo test -p pencreator render_screenshots -- --ignored`.

<p align="center">
  <a href="SUPPORT.md">
    <img src="https://img.buymeacoffee.com/button-api/?text=Buy%20me%20a%20coffee&emoji=%E2%98%95&slug=michalsarna&button_colour=FFDD00&font_colour=000000&font_family=Cookie&outline_colour=000000&coffee_colour=ffffff" alt="Buy me a coffee" height="48">
  </a>
</p>

## Credits and licence

PenCreator is licensed under either of [Apache License 2.0](LICENSE-APACHE) or the [MIT license](LICENSE), at your option. Copyright (c) 2026 PenCreator contributors.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in PenCreator by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

It is built on many open-source projects, fonts and icon sets: see [CREDITS.md](CREDITS.md) and [`assets/LICENSES.md`](assets/LICENSES.md).

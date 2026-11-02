🌐 **English** · [Polski](Home-PL)

<p align="center"><img src="https://raw.githubusercontent.com/michalsarna/PenCreator/main/assets/icons/icon-256.png" alt="PenCreator" width="128"></p>

# PenCreator

Open-source software for pen plotters, written in Rust. **Early:** it has not yet been tried on a real plotter. Devices, pens as layers, fill lines, a plot preview and G-code for GRBL; it writes G-code and can stream it to a GRBL plotter. It is the sibling of [LightCreator](https://github.com/michalsarna/LightCreator) (laser) and [MillCreator](https://github.com/michalsarna/MillCreator) (CNC mill) and uses the same window, editor and files through [baseCreator](https://github.com/michalsarna/baseCreator).

![Editor](https://raw.githubusercontent.com/michalsarna/PenCreator/main/docs/screenshots/05-layers.png)

## Where to start

| I want to… | Go to |
|---|---|
| Install the program | [Installation](Installation) |
| Make a first job, step by step | [Quick Start](Quick-Start) |
| Set the layers, choose pens, understand pen changes | [Layers and Pens](Layers-and-Pens) |
| Find a command in the menus | [Menus](Menus) |
| Learn the keys | [Keyboard Shortcuts](Keyboard-Shortcuts) |
| Share a project with LightCreator and MillCreator | [Project File](Project-File) |
| Edit shapes, import pictures, tabs and panes | [Design Tools](Design-Tools) |
| Drive the machine: jog, zero, firmware, calibration | [Machine Control](Machine-Control) |
| See what it looks like | [Screenshots](Screenshots) |
| Find the settings file | [Settings and Files](Settings-and-Files) |
| Read about the code | [Architecture](Architecture) |
| Know what is there and what is not | [Safety and Limitations](Safety-and-Limitations) |

## At a glance

* **Editor** of baseCreator: shapes, node editing, text, grouping, alignment, undo, grid and snapping, on the work area of the device (A4 for the Dummy). Drawings, the preview, the G-code text and the camera are tabs, in up to three panes.
* **Import:** SVG, PDF / Illustrator, DXF, LightBurn projects (`.lbrn2`), UGS Designer files (`.ugsd`); 310 clipart pictures with online search.
* **Project file:** `.cre`, shared with LightCreator and MillCreator; the sections of other programs are kept when you save.
* **Interface:** 30 languages (including PenCreator's own texts) and 14 colour schemes.
* **Devices and pens:** a list of plotters on the start screen (**I have no device** makes the Dummy), 64 layers that are pens (colour, width, speed, outline, fill with lines or a picture drawn with lines), a pen library (**Plotter → Pen library…**), a plot of the job, the G-code as text in a tab, and G-code for GRBL.
* **Machine control:** jogging, zero, macros, firmware settings, calibration, statistics (see [Machine Control](Machine-Control)).
* **Planned:** HPGL, single-line fonts.

Latest release: **v0.5.3** · [Releases](https://github.com/michalsarna/PenCreator/releases) · [Issues](https://github.com/michalsarna/PenCreator/issues) · Licence: [MIT](https://github.com/michalsarna/PenCreator/blob/main/LICENSE) or [Apache-2.0](https://github.com/michalsarna/PenCreator/blob/main/LICENSE-APACHE), at your option

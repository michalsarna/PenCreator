# Versions

Each release gets the next version number. Newest first.

## v0.4.3 (not released yet)

* **DXF import:** File → Import → Import DXF… (also Open and drag and drop): lines, polylines with bulges, arcs, circles, ellipses, splines and blocks, in millimetres by the unit of the file, with the layers of the DXF on the free layers of the project; lines that meet become paths. **Auto-join paths…**, **Close path with tolerance…** and **Export selection as SVG / DXF…**.

## v0.4.2

* **Preview as a tab:** the preview window has a button in its title line that pins it as a tab of the work area, next to the drawings and the camera view, and a button in the tab that makes it a window again.
* **Plugins:** every plugin that has something to set (a G-code postprocessor, for example *G-code tools*) has a tab of its own in the right-hand panel; the postprocessor is no longer chosen in the Plugins window. The official plugins are translated into Polish, German, Italian, Finnish, Chinese and Hindi.
* **Import / Adjust image:** the dialog follows the size of the window, with the buttons at the bottom, and a new **Levels** column: a histogram of the greys with two sliders for the range.
* **Trace image:** the dialog follows the size of the window; the preview zooms with the scroll wheel, moves by dragging and shows the whole picture again on a double click.
* **Tool palette:** when the window is too low for it (Full HD with the taskbar showing), the buttons grow smaller instead of the palette scrolling.
* Updated dependencies. baseCreator v1.34.1.

## v0.4.1

* **Pen change at a fixed place:** the device settings, page *Pen*, have a place for the pen change (X and Y in the coordinates of the work area, it may lie outside of it), optional G-code to send there (up to 20 lines, for a mechanism that changes the pen by itself) and a switch for the stop (`M0`). When the next layer needs another pen, the pen goes up, the plotter drives to that place, runs the lines and stops so that the pen can be changed by hand or by the mechanism; the way there is part of the preview and the estimate. Without a place the job stops where it is, as before. Tried in the simulator only.
* **Draw one element at a time, or only the selection:** two options in the Control tab, under Frame / From line… (this session only, not saved; they can be used together). **One element at a time** draws each element, a group (even over several layers) or a shape that is in no group, completely, with all its layers and all their passes, before the next one, the nearest element first; it is meant for layers that use the same pen (with different pens the pen is still changed at every switch). **Selection only** draws just the shapes that are selected on the work area when the G-code is saved or the job is started. Both change the G-code, the preview and the estimate, and Frame follows the selection.
* **Machine menu:** *Device settings…* opens the settings of the device, like the button of the Device tab.
* **Camera view:** a tab of the work area next to the drawings (there is no floating window); the picture is fitted to the window up as well as down, in steps of 0.01 so that it does not tremble. A stream that is not drawn is stopped.
* **Windows:** the pop-up windows are moved only by their title line (the pointer is a hand there, a normal one over the icons) and no longer stick to the pointer after a click. The buttons of the device window and of the other dialogs stand at the foot of the window under a line, and the lists of the pen library and the other windows fill their window.
* **Jogging:** jog, jog cancel, Home, Unlock, Pause and Resume follow the GRBL version of the Connection page; hold to move is only for GRBL 1.1 and newer.
* The tests run the generated jobs against the real GRBL firmware (a simulator) in CI.
* baseCreator v1.30.0.

## v0.4.0

* **Raise the pen / Lower the pen** on the Control tab: the pen goes up or down where it stands, with the Z axis or the servo of the device.
* **GRBL versions:** the Connection page of the device has a *GRBL version* choice: 1.1 or newer (the default), 1.0, 0.9, 0.8 or 0.7 or older. The older boards report the position in their own format and have no `$J=` jog (the program jogs with ordinary moves); GRBL 0.7 has no real-time commands (no status, Pause, Home or Unlock) and cuts lines longer than 49 characters. A servo pen lift (`M3 S…`) works with every version.
* The firmware settings table, the quick guide and the clipart window no longer grow to the height of the screen.
* baseCreator v1.24.0.

## v0.3.9

* Small refinements to the look of the program.
* baseCreator v1.22.0.

## v0.3.8

* **Tabs of open drawings:** *New* opens a new tab (at most 10) instead of replacing the drawing; each tab has its own undo, view and file, a dot marks unsaved work, **Ctrl+1…9** (0 = the last) switch tabs and **Ctrl+W** closes one. Closing the program goes through the tabs one by one and asks about each unsaved one; every tab has its own recovery file; closing the last tab closes the program.
* **Control tab:** Home, Unlock, Pause and Resume in a block of 2 × 2 with coloured icons (also in the Device tab), START and a **STOP** that is live only while the plotter works, **Frame** and **From line…** with icons, the Zero buttons in a grid of 2 × 2, Move above the jog buttons and Speed under them.
* The bar over the work area keeps the editing controls, Toolpaths, Preview…, Camera view, the size of the selection and **Save G-code**.
* The ring round the active layer colour is no longer cut off; the Generate shape… and Import windows of plugins keep their height instead of growing to the height of the program window.
* `Fit work area` / `Fit all` moved to **Ctrl+Shift+0** / **Ctrl+Shift+9** (Ctrl+0…9 switch tabs).
* baseCreator v1.20.1.

## v0.3.7

* **G-code viewer:** *File → View G-code file…*, a dropped `.nc` / `.gcode` / `.tap` file, or the **View G-code file…** button on the start screen (no project is started; closing the viewer returns to the start screen) shows the toolpath: flat, or in 3D when Z changes, with zoom, pan, rotation, **Reset view**, hidden rapid moves, colour by height, size, cutting length and a time estimate.
* **Navigation cube** (like in FreeCAD) in the 3D views, i.e. in the G-code viewer and in the 3D view of a plugin's import window: click a face, an edge or a corner to look from there, the arrows turn the view by 15°, drag the cube to rotate.
* The edge of the side panel no longer lights up and takes the drag while the pointer is over a window; the list in the Plugins window is not taller than its window.
* `examples/`: a badge with four pens as a project (`.cre`) and its G-code.
* baseCreator v1.17.0.

## v0.3.6

* **Pens as tools:** a library of ready-made and own pens (width and height of the mark, tip turn, ink colour, opacity, ends), a pen choice on the layer, and the preview draws the lines the way the pen makes them.
* **Pictures are drawn:** the *Picture* drawing of a layer (dithered rows, dots, hatching by tone) with the dithering methods of baseCreator; a picture put on a layer switches it.
* **Preview:** *Fit to the work area* and *Fit to the elements*, as icons in the title line (like in LightCreator).
* **Plugins:** the import window has a form, a preview and a 3D view (for example the STL projection plugin); the generator window has a preview and pictures (for example the box nets to cut out of card); PDF / Illustrator import can use the layers of the file. Plugins can import a KiCad board, Gerber, EAGLE and EasyEDA boards (to plot the traces) from the official list.
* The Device tab uses the shared XY jog pad; the About window shows the baseCreator version.
* egui / eframe 0.37; baseCreator v1.15.0.

## v0.3.5

* **Properties tab:** when one picture is selected, an *Image* section has the buttons **Adjust image…** and **Trace image…** (shared by all programs), so tracing a picture into outlines does not have to be looked for in the menu.
* baseCreator v1.5.0.

## v0.3.4

* **Same shape tools everywhere:** the operations on shapes that LightCreator had (union, intersection, subtract, exclusive or, offset shape, round corners, convert to curves) are now in the shared editor of baseCreator and in the Arrange menu and the right-click menu of every program (Shape operations submenu).
* baseCreator v1.4.0.

## v0.3.3

* **Links open again:** the download page from the update window, Help → Project page on GitHub and the list of plugins did nothing, because the program was built without the link-opening feature of its GUI library. A test keeps it.
* **Right-click menu** on a shape (copy, paste, duplicate, delete, group, lock, flip, turn, arrange, adjust and trace a picture), shared with the other programs.
* Documentation: design tools, pictures, the side panel and the layer bar.
* baseCreator v1.3.0.

## v0.3.2

* **Pictures:** File → Import → Import image… (or drop a PNG, JPG, BMP, GIF or WebP file) with the same import dialog as LightCreator; Arrange → Adjust image… and **Trace image…** turn a picture into outlines to plot.
* **Side panel:** the tabs can be dragged into another order (remembered, also in backups), and a button collapses the panel to a narrow strip with the tab names written vertically; a click on a name opens the panel on that tab.
* **Layer bar:** more room between the colours and at both ends, so the outline of the first one is not pressed against the arrow.
* baseCreator v1.2.0.

## v0.3.1

* **Control tab redesigned:** big position readout with X0/Y0/Z0 next to it, Zero all / Zero X and Y, Go to X0 Y0 / Z to safe height, a separate Move section (XY and Z lines, a text field for any step), speeds in their own section (loaded from the device each time), bigger centred jog buttons and a slimmer Probe section.
* **One move per click:** a jog click used to be sent again on every frame while the mouse button was down; now it moves once (unless Hold to move is on).
* **Alert window:** when the machine alarms, stops or stops answering during a job, a big window explains what happened, with red **STOP** and orange **UNLOCK** buttons.
* **Layer bar:** the selected layer's outline is no longer clipped, and the bar ends exactly at the work panel.
* **Layer order:** layers can be moved earlier/later in the job (chevrons in the Layers panel); the plot follows that order.
* **Update check:** Help → Check for updates, and an option to look for a newer version at start (Settings → Program options).
* Windows (preview, …) keep their size; double Esc selects the Select tool.

## v0.3.0

* **Layer bar** under the editor, the same in all programs: one row of **64 layer colours** (arrows or the wheel scroll it; a dot marks layers with shapes). A click puts the selected shapes on that layer (or, with nothing selected, makes it the layer to draw on); a double click opens its options. One palette in all programs. Projects with fewer layers open with the rest added.
* **Autosave and recovery:** a project with a file is saved to it while you work; one without a name goes to a recovery file, offered as a big **Recover** button on the start screen, and removed at the first save.
* **Ctrl / Cmd** click and drag add to the selection, like Shift.
* Two new buttons at the bottom of the tool strip: **Generate shape** (plugins) and **Online clipart**.
* **Plugins:** an official list (Settings → Plugins… → Official plugins) with Install / Update; the Plugins window no longer grows to the height of the screen.
* **Security:** plugin zips are installed from exactly the bytes that were checked, with real size limits; backups are restored with limits; temporary files (print, ser2net) are private; rustls 0.23.46 (four advisories); build actions pinned.
* macOS releases are only the `.dmg` (no more `.tar.gz`).
* baseCreator v1.0.0.

## v0.2.9

* **Official plugin list** (Settings → Plugins… → Official plugins): all plugins of the repository *baseCreator-plugins* (also those for other programs) with **Install** and **Update**, checksum check, and a switch to look for updates at the start (off by default). A fractals plugin (Sierpinski, Koch, Hilbert, dragon curve, Pythagoras tree) and more are in the list. Guide for authors and the way to send a plugin: the plugins repository.
* baseCreator v0.9.9.

## v0.2.8

* **Plugins** (Settings → Plugins…): packs without code (clipart, presets, translations, data files), importers of other file formats, generators of shapes with a form and G-code postprocessors; scripts (Rhai) run in a sandbox, the output of a postprocessor is checked and controller commands are refused. File → Import → *Import with a plugin…* and *Generate shape…*. Shared with the other programs on baseCreator; see the Plugins wiki page.
* Plugins can bring machine presets of class `pen`.
* baseCreator v0.9.8.

## v0.2.7

* **Import of UGS Designer files** (`.ugsd`, File → Import → Import UGS design…, also drag and drop): shapes, rounded rectangles, ellipses, text and pictures; every way of cutting of the file (cut type, depth, feed, spindle speed) becomes a layer (pockets and fills are filled, the rest is outlined). Hidden objects are drawn too.
* **Imports (LightBurn and UGS) are really imports now:** the shapes are added to the open drawing, selected and can be undone with one step; the layers of the file are used only for layers that have no shapes yet. They no longer replace the project or appear in the recent files.
* baseCreator v0.9.7.

## v0.2.6

* **Machine control:** the **Control** tab (work and machine position, zero and G54 … G59, jogging by steps or by holding a button, keyboard jogging, macros with variables, G-code before and after every job) and the **Machine** menu (settings of the controller `$$` with search, backup before the first change, restore, factory reset; calibration; diagnostics with statistics, maintenance reminders and a zip; **Start from line…**; **Check the program**). A job stops at a change of the pen.
* **Copies of the settings of the controller** are kept with the device (made at the first connection and whenever the settings differ from the newest copy, with a message saying what changed); they are part of the backup of the program.
* Alarms and errors of GRBL are explained in the console and the status bar, in 30 languages. A device can be read from the controller and started from a usual sheet size.
* baseCreator v0.9.6.

## v0.2.5

* The Dummy device: choose **Landscape** or **Portrait** for its A4 sheet, under the device list on the start screen and on the Device tab (new texts in 30 languages).
* No more moving of settings from the old per-program folders (v0.2.4 did it once); the shared folder is simply used. baseCreator v0.9.5.

## v0.2.4

* One data folder for all programs: `baseCreator` (macOS `~/Library/Application Support/baseCreator`). The clipart gallery is now **shared** by LightCreator, MillCreator and PenCreator; settings, device profiles, materials and tools stay separate per program in `apps/<program>/app.ron`.
* The first start after the update moves the old settings and pictures into the shared folder (the old folder is left untouched, identical pictures are not duplicated). `BASECREATOR_DIR` selects another folder.
* The gallery index is written safely and re-read, so two programs open at once do not lose each other's pictures.
* **Help → Download backup…** and **Restore from backup…**: one zip with the settings of PenCreator and the shared gallery. Restoring replaces the settings and adds the pictures to the gallery (nothing is lost or doubled); a backup made by another program is refused. The same entries as in LightCreator, made in baseCreator.
* baseCreator v0.9.4.

## v0.2.3

* A **Console** tab, as in LightCreator: the traffic with the controller, status polls on request, auto-scroll, clear and a line to send a command. It replaces the small console of the Device tab.
* baseCreator v0.9.2.

## v0.2.2

* The control bar has **Toolpaths**, **Preview…**, **Camera view**, **Frame**, **Start** / **Stop** and **Save G-code**, as in MillCreator; the device has a Connection page (serial port or network socket) and the Device tab connects to a GRBL plotter, homes, pauses and shows a console.
* **The animation of the preview works** (its buttons were inert).
* The preview window no longer has an *Export G-code…* button.
* baseCreator v0.9.1.

## v0.2.1

* The preview window is the same as MillCreator's and has an **animation**: back to the start, play / pause, speed (×0.25 to ×25) and repeat, with the time and the length.
* The pointer is the tool's only over the work area; the line of the side panel and the line between the list of layers and their options light up red and thicken under the pointer, and the latter is dragged to resize the list.
* baseCreator v0.9.0.

## v0.2.0

* Devices with the start screen of MillCreator (list, add, edit, duplicate, export, import, **I have no device**; **New** and **Open…** without a device use the Dummy), cameras with a Camera tab.
* 16 pen layers (colour, width, speed, outline / fill with lines / both, crossed lines, draw several times), the `pen` project section.
* The plot window and G-code for GRBL (Z axis or servo, `M0` between pens).
* PenCreator's own texts in 30 languages.
* The device section of the start screen is centred: *Add device…*, *Edit…*, *Duplicate*, *Import…* and, in a second row, *I have no device*; *Export…* and *Delete* are in the window of *Edit…*. baseCreator v0.8.1.

## v0.1.0

* First release: an early skeleton. The window, menus and editor of baseCreator, the clipart gallery, SVG / PDF / Illustrator and LightBurn import, the shared `.cre` project file with the Description tab; no plotting yet.

🌐 **English** · [Polski](Menu)

# Menus

The menu bar has nine menus, in this order: **File · Edit · Insert · Arrange · View · Machine · Plotter · Settings · Help**. LightCreator and MillCreator have the same layout (it comes from baseCreator); only the menu of the process differs (**Laser**, **Milling**, **Plotter**).

* On **Windows and Linux** the menu bar is in the window, under the title bar. It is shown in the editor, not on the start screen.
* On **macOS** the menus are in the system menu bar at the top of the screen. It has two more menus: the app menu **PenCreator** (**About PenCreator**, the usual entries of the system, **Quit**) and **Window** (minimise and maximise). That is why **Quit** is not at the end of **File** and **About** is not at the end of **Help** on a Mac.
* An entry that cannot be used now is grey (for example **Trace image…** when no picture is selected, **Save** on the start screen). A switch has a tick when it is on.
* The keys are listed next to the entries; all of them are on the page [Keyboard Shortcuts](Keyboard-Shortcuts). Read the note about macOS there.

## File

| Entry | Key | What it does |
|---|---|---|
| **New** | Ctrl+N | A new, empty drawing on the sheet of the device, in a tab of its own (at most 10 drawings are open at once). |
| **Open…** | Ctrl+O | Opens a project (`.cre`), an SVG, PDF or Illustrator (`.ai`) file in a tab of its own; a DXF or UGS design (`.ugsd`) is imported into the drawing. A project that is open already is shown, not opened twice. See [Quick Start](Quick-Start) and [Project File](Project-File). |
| **Open recent** | | The last 10 files; **Clear list** empties the list. |
| **Save** | Ctrl+S | Writes the project as `.cre`. A drawing that has no project file yet (new, or opened from SVG or PDF) asks for a name first. |
| **Save as…** | Ctrl+Shift+S | Writes the project under a new name. |
| **Import → Import image…** | | A picture (PNG, JPG, BMP, GIF, WebP) through the import dialog. See [Design Tools](Design-Tools). |
| **Import → Import DXF…** | | The lines, arcs and curves of a DXF drawing are added to the drawing. |
| **Import → Import LightBurn project…** | | The shapes of a LightBurn project (`.lbrn2`, `.lbrn`) are added to the drawing. |
| **Import → Import UGS design…** | | The shapes of a UGS Designer file (`.ugsd`) are added to the drawing. |
| **Import → Import with a plugin…** | | A file of a format that an installed plugin reads. See [Plugins](Plugins). |
| **Import → Generate shape…** | | The generators of the installed plugins. |
| **Export → Export G-code…** | | The same as **Plotter → Export G-code…**. |
| **Export → Export selection as SVG…** / **Export selection as DXF…** | | Writes the selected shapes (all of them when nothing is selected) as SVG or DXF. |
| **View G-code file…** | | Opens a G-code file (`.gcode`, `.nc`, `.gc`, `.gco`, `.ngc`, `.tap`, `.cnc`) in the **G-code analysis** tab: its toolpaths, without changing the drawing. |
| **Quit** | Ctrl+Q | Closes the program; every drawing with unsaved work is asked about first. Windows and Linux only (on macOS it is in the app menu). |

## Edit

| Entry | Key | What it does |
|---|---|---|
| **Undo** | Ctrl+Z | Takes back the last change of the drawing (up to 100 steps). |
| **Redo** | Ctrl+Y | Does it again (Ctrl+Shift+Z works too). |
| **Copy** | Ctrl+C | Copies the selected shapes. |
| **Paste** | Ctrl+V | Pastes the copied shapes. |
| **Duplicate** | Ctrl+D | Copies the selected shapes and pastes them at once. |
| **Delete** | Del | Deletes the selected shapes (with the **Node edit (N)** tool, the selected nodes). |
| **Select all** | Ctrl+A | Selects every shape on the layers that are shown. |

## Insert

The tools of the left palette that put new objects on the page: **Rectangle (R)**, **Ellipse (E)**, **Triangle (Y)**, **Star (S)**, **Polygon (G)**, **Heart (K)**, **Spiral (I)**, **Line (L)**, **Polyline / pen (P)** and **Text (T)**. The tool in use is ticked. Then:

* **Generate shape…**: the generators of the installed plugins (the same as in **File → Import**).
* **Online clipart…**: opens the **Clipart** tab and the search of free pictures on the internet.

New shapes go on the active layer (see [Layers and Pens](Layers-and-Pens)).

## Arrange

Most entries work on the selected shapes; the same commands are in the right-click menu of a shape and on the arrange bar right of the work area. They are explained on [Design Tools](Design-Tools).

| Entries | Key | What they do |
|---|---|---|
| **Align left**, **Align centre (H)**, **Align right**, **Align top**, **Align centre (V)**, **Align bottom** | | Line the selected shapes up. |
| **Centre on bed** | | Puts the selection in the middle of the work area. |
| **Centre on each other** | | Moves the selected shapes (or groups) to the middle of the selection, one over the other (two or more). |
| **Flip horizontal**, **Flip vertical** | | Mirrors the selection. |
| **Rotate 90° CW**, **Rotate 90° CCW** | | Turns the selection by a quarter. |
| **Group** | Ctrl+G | Makes one group of the selected shapes. |
| **Ungroup** | Ctrl+Shift+G | Breaks the group up. |
| **Lock** | Ctrl+L | Locked shapes cannot be moved or changed. |
| **Unlock** | Ctrl+Shift+L | Frees them again. |
| **Bring to front**, **Send to back** | | Changes the order of the shapes in the drawing. |
| **Edit nodes** | N | The **Node edit (N)** tool on the selected shape. |
| **Convert to path** | | Makes a path of a shape (a rectangle, a star…), so that its nodes can be edited. |
| **Union**, **Intersection**, **Subtract**, **Exclusive or** | | Joins, cuts and combines closed shapes. |
| **Offset shape…**, **Round corners…**, **Convert to curves** | | A larger or smaller outline, rounded corners, curves in place of lines. |
| **Auto-join paths…**, **Close path with tolerance…** | | Joins open paths whose ends are near each other, closes paths whose ends are near. |
| **Dogbone / T-bone corners…** | | Notches in sharp corners (for milling; the same in all three programs). |
| **Grid array…**, **Circular array…**, **Copy along path…**, **Text on path…** | | Copies in a grid, round a centre, along a path; text bent along a path. |
| **Trim by shape…**, **Remove duplicates**, **Break apart**, **Simplify…** | | Cuts lines by a shape, removes doubles, splits a shape into its outlines, takes away needless points. |
| **Serial text…**, **Barcode / QR code…**, **Nest shapes…**, **Measure…** | | Numbered texts, codes, packing shapes on an area, measuring. |
| **Adjust image…**, **Trace image…** | | Changes the selected picture, or turns it into outlines. Grey unless a picture is selected. |

## View

| Entry | Key | What it does |
|---|---|---|
| **Grid**, **Snap to grid**, **Snap to objects** | | Switches (ticked when on): the grid on the work area, snapping to the grid, snapping to other shapes. |
| **Zoom in** | Ctrl++ | Zooms in about the middle of the view. |
| **Zoom out** | Ctrl+- | Zooms out. |
| **Fit bed to window** | Ctrl+Shift+0 | Shows the whole work area. |
| **Fit all objects** | Ctrl+Shift+9 | Shows all the shapes. |
| **Auto-fit work area to window** | | A switch: the work area stays fitted when the window changes size. Zooming or panning by hand switches it off. |
| **Split view: side by side**, **Split view: one above the other** | | Divides the work area into panes (up to three). With one pane, its last tab goes to a new pane (when the pane has two tabs or more). See [Design Tools](Design-Tools#tabs-and-panes). |
| **Single view** | | Puts all the tabs together in one pane again. |
| **Background picture…** | | A photo or a scan under the drawing, to draw over. |

## Machine

| Entry | What it does |
|---|---|
| **Device settings…** | The settings of the device of the project (pages General, Machine, Pen, G-code, Connection, Camera, Jogging, Macros, Maintenance, Notes). See [Quick Start](Quick-Start) and [Machine Control](Machine-Control). |
| **Firmware settings ($$)…** | The settings of the GRBL controller, with copies and a backup. |
| **Calibration…** | Steps per millimetre and the squareness of the axes. |
| **Diagnostics and statistics…** | The inputs of the controller, the statistics and maintenance of the machine, a report to save. |
| **Start from line…** | Opens the window that starts the job at a line of the program (when there is something to draw); starting needs a connected machine. |
| **Check the program** | The controller goes through the program in check mode (`$C`); nothing moves. Needs a connected machine. |

All of them are explained on [Machine Control](Machine-Control).

## Plotter

| Entry | Key | What it does |
|---|---|---|
| **Pen library…** | | The ready-made pens and your own ones. See [Layers and Pens](Layers-and-Pens#the-pen-library). |
| **Plot of the job…** | Ctrl+Shift+P | Opens the **Preview** tab: what the pen will draw, with the moves with the pen up, the time and an animation. See [Quick Start](Quick-Start). |
| **Export G-code…** | Ctrl+E | Asks where to save and writes the G-code of the project (the **Save G-code** button does the same). |

## Settings

| Entry | What it does |
|---|---|
| **View options…** | The grid, the colours of the work area, the thickness of the lines, **Show layer names**, **Preview G-code after saving**, **Show variable names**. |
| **Program options…** | **Look for a newer version when the program starts** (on by default). |
| **Plugins…** | Installs, switches and removes plugins. See [Plugins](Plugins). |
| **Language** | 30 languages; the choice is kept. |
| **Colour scheme** | 14 colour schemes (and a festive one around some holidays). |

## Help

| Entry | What it does |
|---|---|
| **Project page on GitHub** | Opens the page of PenCreator in the browser. |
| **Download backup…** | Saves the settings and the shared clipart gallery in a zip. See [Settings and Files](Settings-and-Files). |
| **Restore from backup…** | Reads such a zip back after a question. |
| **Check for updates…** | Looks for a newer release. |
| **About** | The version, the licence and the credits. Windows and Linux only (on macOS: **About PenCreator** in the app menu). |

🌐 **English** · [Polski](Skroty-klawiszowe)

# Keyboard Shortcuts

Ctrl is Cmd on a Mac. The keys work in the editor (not on the start screen), and none of them is taken while a text field has the keyboard: typing in a field, in the **Description** tab or in the G-code text tab types the letters. On a Mac the system menu bar does not show every key; see [On macOS](#on-macos).

## Keys of the menus

These are shown next to the entries of the menus (see [Menus](Menus)). In the menu bar of the window they work even when the menu is closed; an entry that is grey at that moment does nothing.

| Key | Command |
|---|---|
| Ctrl+N | **File → New** |
| Ctrl+O | **File → Open…** |
| Ctrl+S | **File → Save** |
| Ctrl+Shift+S | **File → Save as…** |
| Ctrl+Q | **File → Quit** |
| Ctrl+Z | **Edit → Undo** |
| Ctrl+Y or Ctrl+Shift+Z | **Edit → Redo** |
| Ctrl+C | **Edit → Copy** |
| Ctrl+V | **Edit → Paste** |
| Ctrl+D | **Edit → Duplicate** |
| Delete or Backspace | **Edit → Delete** (with the **Node edit (N)** tool and nodes selected: deletes the nodes) |
| Ctrl+A | **Edit → Select all** |
| Ctrl+G | **Arrange → Group** |
| Ctrl+Shift+G | **Arrange → Ungroup** |
| Ctrl+L | **Arrange → Lock** |
| Ctrl+Shift+L | **Arrange → Unlock** |
| N | **Arrange → Edit nodes** (the **Node edit (N)** tool) |
| Ctrl++ (Ctrl+=) | **View → Zoom in** |
| Ctrl+- | **View → Zoom out** |
| Ctrl+Shift+0 | **View → Fit bed to window** |
| Ctrl+Shift+9 | **View → Fit all objects** |
| Ctrl+Shift+P | **Plotter → Plot of the job…** |
| Ctrl+E | **Plotter → Export G-code…** |

No two entries of the menus share a key; a test of the program checks it.

## Tabs of drawings

| Key | What it does |
|---|---|
| Ctrl+1 … Ctrl+9, Ctrl+0 | Shows the drawing of the 1st … 9th tab, Ctrl+0 the 10th (at most 10 drawings are open). |
| Ctrl+W | Closes the drawing that is shown; when it has unsaved work you are asked first (**Save**, **Discard**, **Cancel**). |

## Tools

A letter without Ctrl chooses a tool of the left palette (the letter is in the tip of its button):

| Key | Tool | Key | Tool |
|---|---|---|---|
| V | **Select (V)** | L | **Line (L)** |
| N | **Node edit (N)** | P | **Polyline / pen (P)** |
| R | **Rectangle (R)** | T | **Text (T)** |
| E | **Ellipse (E)** | H | **Pan (H)** |
| Y | **Triangle (Y)** | Z | **Zoom in (Z)** |
| S | **Star (S)** | X | **Zoom out (X)** |
| G | **Polygon (G)** | | |
| K | **Heart (K)** | | |
| I | **Spiral (I)** | | |

| Key | What it does |
|---|---|
| Esc | Clears the selection (and a polyline being drawn). |
| Esc twice (within half a second) | Back to the **Select (V)** tool. |
| Enter | Ends the polyline being drawn with **Polyline / pen (P)**. |

## Mouse with keys

| Action | What it does |
|---|---|
| Wheel (or pinch) on the work area | Zooms about the pointer. |
| Middle button drag | Pans the view (with **Pan (H)** the left button does it). |
| Shift+click or Ctrl+click | Adds a shape to the selection, or takes it out. |
| Shift while drawing a rectangle, ellipse, triangle, star, polygon, heart or spiral | Keeps the width and the height equal. |
| Dragging a corner handle | Keeps the proportions; with Shift the width and the height change freely. |
| Shift while turning | Turns in steps of 15°. |
| Shift with **Zoom in (Z)** / **Zoom out (X)** | Zooms the other way. |
| Right click, or double click, with **Polyline / pen (P)** | Ends the polyline; a click near the first point closes it. |
| Right click on a shape | The menu of the shape (see [Design Tools](Design-Tools)). |
| Middle click on a tab of a pane | Closes the tab. |
| Double click on the line between panes | Makes the panes the same size. |
| Wheel over the row of layers under the work area | Scrolls the row sideways. |

In the **Preview** tab the wheel zooms, a drag moves the picture and a double click goes back to the first view.

## Control tab

With **Keyboard** ticked on the **Control** tab (off by default), the arrow keys move X and Y and Page Up / Page Down move Z, by the step chosen there. See [Machine Control](Machine-Control).

## On macOS

The menus are in the system menu bar of macOS. All the keys of this page work there too:

* **Cmd+N**, **Cmd+O**, **Cmd+S**, **Cmd+Shift+S** and **Cmd+Q** (in the app menu **PenCreator**) belong to the system menu bar, which shows them next to their entries.
* The other keys of the menus (**Cmd+E**, **Cmd+Shift+P**, **Cmd+L**, **Cmd+Shift+L**, **Cmd+=** / **Cmd+-**, **Cmd+Shift+0** / **Cmd+Shift+9**, the editing keys) are taken by the program itself in its window. They work the same as on Windows and Linux, but the system menu bar does not show them next to their entries; the table above lists them.

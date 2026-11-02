🌐 **English** · [Polski](Narzedzia-projektowe)

# Design Tools

PenCreator edits shapes with the same tools as LightCreator and MillCreator (baseCreator). The keys of the tools are on [Keyboard Shortcuts](Keyboard-Shortcuts), the menus on [Menus](Menus).

## Editing

* **Right mouse button** on a shape opens its menu: **Copy**, **Paste**, **Duplicate**, **Delete**, **Select all**, **Group**, **Ungroup**, **Lock**, **Unlock**, the flips and quarter turns, the submenu **Arrange** (order, alignment, centring, **Edit nodes**, **Convert to path**) and the submenu **Shape operations** (**Union**, **Intersection**, **Subtract**, **Exclusive or**, **Offset shape…**, **Round corners…**, **Convert to curves**; they are also in the **Arrange** menu); for a picture also **Adjust image…** and **Trace image…**. The entries that do not fit the selection are grey.
* **Esc twice** (a second Esc within half a second) goes back to the **Select (V)** tool.
* **The row of layers** under the work area: one swatch for each of the 64 layers (a dot marks the layers with shapes); a click puts the selected shapes on that layer, a double click opens its settings. See [Layers and Pens](Layers-and-Pens).

## Importing

* **DXF:** **File → Import → Import DXF…** (or **File → Open…**, or drag and drop) reads an ASCII `.dxf` (R12 and newer): lines, polylines with bulges, arcs, circles, ellipses, splines and blocks as outlines, in millimetres (the unit of the file is read), arcs and circles as curves; lines that meet become one path. The layers of the DXF go to the layers of the project that have no shapes yet, one after the other. What is not a line is left out and named in the status line. **Arrange → Auto-join paths…** / **Close path with tolerance…** mend paths that are not quite closed, and **File → Export → Export selection as SVG…** / **Export selection as DXF…** write shapes back out.
* **LightBurn and UGS Designer:** **File → Import → Import LightBurn project…** (`.lbrn2`, `.lbrn`) and **Import UGS design…** (`.ugsd`) add the shapes to the drawing. The ways of cutting of a UGS design set up the layers that have no shapes yet (a pocket or a fill becomes **Fill with lines**, the others **Outline**).
* **Pictures (bitmaps):** **File → Import → Import image…** (or drop a PNG, JPG, BMP, GIF or WebP file on the window) opens the import dialog: rotate, flip, brightness, contrast, gamma, auto levels, negative and size. **Adjust image…** (in the *Image* section of the **Properties** tab, or **Arrange → Adjust image…**) changes a picture on the page, and **Trace image…** (the same section, or **Arrange → Trace image…**) turns its dark (or light) areas into outlines (threshold, speckle filter, simplification, curve smoothing) that are drawn like any other shape.
* **Pictures drawn with lines:** a picture put on a layer switches that layer to the drawing **Picture** (once; you can change it back), and then the picture itself is drawn with lines, by the **Picture method** of the layer: **Dithered rows**, **Dots** or **Hatching by tone**. On a layer with another drawing a picture is only shown, not drawn. See [Layers and Pens](Layers-and-Pens#the-kinds-of-drawing).

## Arranging, copying and generating

The **Insert** menu has the tools of the left palette that insert objects (**Rectangle (R)**, **Ellipse (E)**, **Triangle (Y)**, **Star (S)**, **Polygon (G)**, **Heart (K)**, **Spiral (I)**, **Line (L)**, **Polyline / pen (P)** and **Text (T)**; the tool in use is ticked), then **Generate shape…** (the generators of plugins) and **Online clipart…**.

Commands of the **Arrange** menu (the same in LightCreator, MillCreator and PenCreator):

* **Auto-join paths…** joins open paths whose ends are nearer than a tolerance; **Close path with tolerance…** closes the ones whose two ends are that near. **Dogbone / T-bone corners…** adds a notch to every sharp corner so that a round bit reaches it (it is meant for milling).
* **Grid array…** copies the selection in rows and columns. **Circular array…** puts copies round the middle of the selection over a *Total angle* (360° goes all the way round), turned with their place or only moved. At most 2000 copies are made.
* **Copy along path…**: select the shapes, then the path **last**; copies are put every *Distance* along it and may be turned with it. **Text on path…**: select the text, then the path last; the outlines of the text are bent along the path from *Start* (afterwards they are outlines, not text).
* **Trim by shape…**: select the lines, then a closed shape **last**; the parts inside (or outside) are kept, cut where the lines cross its outline. **Remove duplicates** deletes shapes with the same outline on the same layer (the selection, or the whole drawing). **Break apart** makes one shape of every outline. **Simplify…** takes away the points within a *Tolerance* of the line.
* **Serial text…**: select a text; it is copied in a grid with its `{n}` counted up, or, with **Read a CSV file…**, once for every row, `{1}`, `{2}`… being the columns. **Barcode / QR code…** puts a QR code or a Code 39 barcode on the active layer as outlines (make them large enough for the pen).
* **Nest shapes…** packs the selected shapes in rows from the corner of the page on an area you give, with a distance between them (by their boxes, not their outlines; quarter turns can be allowed). **Measure…** shows the size, position, area and length of the selection and, for two shapes, the distances between them.

**File → Export → Export selection as SVG…** / **Export selection as DXF…** writes the selected shapes (all of them when nothing is selected).

## Background picture (View menu) and the trace dialog

**View → Background picture…** puts a photo, a plan or a scan **under the drawing**: **Load a picture from a file…** (PNG, JPG, BMP, WebP, GIF). Tick **Move the corners** and drag its four corners onto the corners of the paper or of the work area (the picture follows with a perspective transform); **Opacity**, **Show**, **Fit to the work area** and **Remove** are in the same window. The picture is not saved with the project. In the **Trace image** dialog the picture can be zoomed with the scroll wheel or the **Zoom in**, **Zoom out** and **Reset view** buttons, and the green **Trace** button makes the outlines.

## The side panel

* Drag a tab to put it in another place (the order is remembered, also in a backup). The small arrow at the left of the tabs hides the panel; a narrow strip with the names of the tabs stays, and a click on a name opens the panel on that tab.
* The tabs: **Properties**, **Layers**, **Device**, **Control**, **Console**, **Clipart**, **Description**, **Camera** (when the device has a camera that is switched on) and a tab for every plugin that has settings.

## Tabs and panes

Every open drawing, the **Preview**, the G-code of a project as text (**Preview G-code: name.nc**), the **G-code analysis** of a file and the **Camera view** are **tabs** above the work area.

* **View → Split view: side by side** or **Split view: one above the other** divides the work area into up to **three panes**; **Single view** puts all the tabs together again. Every pane has its own bar of tabs, with a **+** for a new drawing.
* **Drag a tab** to another pane (or inside its bar to another place). Or use its menu (right click): **Move to pane N** (the other panes, and a new one when there is room and the pane keeps another tab), **Open in a separate window** (for the preview, the G-code text and the camera view; the window remembers its place, and **Back to the main window** or closing the window puts the tab back) and **Close tab**. A middle click on a tab closes it too.
* Drag the line between two panes to change their sizes; a double click on it makes them equal.
* The last tab of a pane closed closes the pane, and the others close up.
* Nothing is opened twice: opening a project that is open already shows its tab, and a project has one preview, one camera view and one G-code text.
* One drawing is the **active** one: the tools, the menus and the keys work on it. A drawing in another pane is shown for looking, with a zoom of its own; a click on it makes it the active one.
* The layout of the panes is remembered.

## The G-code text tab

**Preview G-code** (the button next to **Save G-code**) makes the program of the project (with the postprocessors that are on) and shows it **as text in a tab**, coloured (G and M words, axes, speeds, comments) and open to edits. **Save G-code** in the tab writes it to a file; **Make again** makes it from the project again (with edits it first asks **Lose the edits?**); **Analyse** shows its toolpaths in the **G-code analysis** tab. A tab with edits shows *Changed by hand* and a dot, and asks whether to save them when it is closed. Without edits the text is made again by itself after the project is saved, the notes change, a setting of the device changes or the postprocessors change. **Settings → View options… → Preview G-code after saving** (off by default) opens the saved file in the analysis after every **Save G-code**.

## View options

**Settings → View options…** has the main and the secondary grid, the colours of the work area and of the space around it, the line thickness, and:

* **Show layer names** (off by default): the row of layers under the work area writes the name of a layer that has shapes beside its colour (long names are cut, the whole one is in the tip; the row scrolls sideways when it is full).
* **Preview G-code after saving** (off by default): see above.
* **Show variable names** (off by default): beside the label of every field that has a variable for the own G-code, its name is shown in italics and brackets, for example *(machine_name)* beside the name of the device or *(layer_name)* beside the name of a layer; a click copies `{machine_name}` to the clipboard, ready to be pasted into a box of own G-code (see [Machine Control](Machine-Control#running-a-job)).

## Layer presets

**Save preset…** and **Load preset** above the settings of a layer open the window **Layer presets**: the pen settings of the layer are kept under a name and put on any layer later. See [Layers and Pens](Layers-and-Pens#layer-presets).

🌐 **English** · [Polski](Narzedzia-projektowe)

# Design Tools

PenCreator edits shapes with the same tools as LightCreator and MillCreator (baseCreator). The keys of the tools are on [Keyboard Shortcuts](Keyboard-Shortcuts), the menus on [Menus](Menus).

## Text

Choose **Text (T)** and click on the page: a text appears and the **Properties** tab opens with the field to type in. There you set the font, the size, bold and italic, the letter and line spacing and the alignment. **Convert to curves** turns the text into shapes whose nodes can be edited. A new text takes the settings of the text changed last.

## Editing

* **Right mouse button** on a shape opens its menu: **Copy**, **Paste**, **Duplicate**, **Delete**, **Select all**, **Group**, **Ungroup**, **Lock**, **Unlock**, the flips and quarter turns, the submenu **Arrange** (order, alignment, centring, **Edit nodes**, **Convert to path**) and the submenu **Shape operations** (**Union**, **Intersection**, **Subtract**, **Exclusive or**, **Offset shape…**, **Round corners…**, **Convert to curves**; they are also in the **Arrange** menu); for a picture also **Adjust image…** and **Trace image…**. The entries that do not fit the selection are grey.
* **Esc twice** (a second Esc within half a second) goes back to the **Select (V)** tool.
* **The row of layers** under the work area: one swatch for each of the 64 layers (a dot marks the layers with shapes); a click puts the selected shapes on that layer, a double click opens its settings. See [Layers and Pens](Layers-and-Pens).

## Importing

* **DXF:** **File → Import → Import DXF…** (or **File → Open…**, or drag and drop) reads an ASCII `.dxf` (R12 and newer): lines, polylines with bulges, arcs, circles, ellipses, splines and blocks as outlines, in millimetres (the unit of the file is read), arcs and circles as curves; lines that meet become one path. The layers of the DXF go to the layers of the project that have no shapes yet, one after the other. What is not a line is left out and named in the status line. **Arrange → Auto-join paths…** / **Close path with tolerance…** mend paths that are not quite closed, and **File → Export → Export selection as SVG…** / **Export selection as DXF…** write shapes back out.
* **LightBurn, MillMage and UGS Designer:** **File → Import → Import LightBurn / MillMage project…** (`.lbrn2`, `.lbrn`, `.mage`) and **Import UGS design…** (`.ugsd`) add the shapes to the drawing. The ways of cutting of a UGS design set up the layers that have no shapes yet (a pocket or a fill becomes **Fill with lines**, the others **Outline**).
* **Files of other programs, both ways:** DXF (the current kind and R12, *legacy*), Illustrator (also the old PostScript kind), SVG with its pictures, and LightBurn (`.lbrn2`, `.lbrn`) and MillMage (`.mage`) projects are read, and written back the same. **File → Export** writes them back: **Export DXF…** (the current kind), **Export DXF R12 (legacy)…**, **Export Adobe Illustrator…**, **Export LightBurn project…** (`.lbrn2`, or `.lbrn` when you name the file so) and **Export MillMage project…**, besides **Export SVG…**. Curves are kept (DXF flattens them to 0.01 mm); pictures go into SVG, LightBurn and MillMage files and are left out of DXF and Illustrator files (the program says how many).
* **Pictures (bitmaps):** **File → Import → Import image…** (or drop a PNG, JPG, BMP, GIF or WebP file on the window) opens the import dialog: rotate, flip, brightness, contrast, gamma, auto levels, negative and size. **Adjust image…** (in the *Image* section of the **Properties** tab, or **Arrange → Adjust image…**) changes a picture on the page, and **Trace image…** (the same section, or **Arrange → Trace image…**) turns its dark (or light) areas into outlines (threshold, speckle filter, simplification, curve smoothing) that are drawn like any other shape. **Arrange → Style trace…** draws a picture again as lines in one of 19 styles (squiggles, a spiral, stippling, hatching, flow lines, one continuous line…): see [Style Trace](Style-Trace).
* **Pictures drawn with lines:** a picture put on a layer switches that layer to the drawing **Picture** (once; you can change it back), and then the picture itself is drawn with lines, by the **Picture method** of the layer: **Dithered rows**, **Dots** or **Hatching by tone**. On a layer with another drawing a picture is only shown, not drawn. See [Layers and Pens](Layers-and-Pens#the-kinds-of-drawing).

## Arranging, copying and generating

The **Insert** menu has the tools of the left palette that insert objects (**Rectangle (R)**, **Ellipse (E)**, **Triangle (Y)**, **Star (S)**, **Polygon (G)**, **Heart (K)**, **Spiral (I)**, **Line (L)**, **Polyline / pen (P)** and **Text (T)**; the tool in use is ticked), then **Generate shape…** (the generators of plugins) and **Online clipart…**.

Commands of the **Arrange** menu (the same in LightCreator, MillCreator and PenCreator):

* **Auto-join paths…** joins open paths whose ends are nearer than a tolerance; **Close path with tolerance…** closes the ones whose two ends are that near. **Dogbone / T-bone corners…** adds a notch to every sharp corner so that a round bit reaches it (it is meant for milling).
* **Grid array…** copies the selection in rows and columns. **Circular array…** puts copies round the middle of the selection over a *Total angle* (360° goes all the way round), turned with their place or only moved. At most 2000 copies are made.
* **Copy along path…**: select the shapes, then the path **last**; copies are put every *Distance* along it and may be turned with it. **Text on path…**: select the text, then the path last; the outlines of the text are bent along the path from *Start* (afterwards they are outlines, not text).
* **Trim by shape…**: select the lines, then a closed shape **last**; the parts inside (or outside) are kept, cut where the lines cross its outline. **Remove duplicates** deletes shapes with the same outline on the same layer (the selection, or the whole drawing). **Break apart** makes one shape of every outline. **Simplify…** takes away the points within a *Tolerance* of the line.
* **Serial text…**: select a text; it is copied in a grid with its `{n}` counted up, or, with **Read a CSV file…**, once for every row, `{1}`, `{2}`… being the columns. **Barcode / QR code…** puts a QR code or a Code 39 barcode on the active layer as outlines (make them large enough for the pen).
* **Nest shapes…** packs the selected shapes on an area of *Area width* × *Area height* from the corner of the page, with a *Distance* between them. With **By the real outlines** (on by default) it packs them by their true shapes: a small part goes into the hole of a big one, an L into the corner of another, a group stays one part, and **Turning** lets a part turn by half, quarter or eighth turns. **What does not fit goes on new sheets** puts the rest on new sheets: every sheet is a new tab with the same layers and material (save it and run it after this one); without it what does not fit stays where it was. Switched off, the shapes are packed by their boxes in rows (*Allow turning* lays a tall one down). **Measure…** shows the size, position, area and length of the selection and, for two shapes, the distances between them.

**File → Export → Export selection as SVG…** / **Export selection as DXF…** writes the selected shapes (all of them when nothing is selected).

## Camera overlay (View menu) and the trace dialog

**View → Overlay from…** puts a photo, a plan or a scan **under the drawing** (**Load picture…**), or the live picture of the camera of the device (**Start camera**, **Freeze frame**; also the **Overlay** button on the camera tab). Tick **Align: drag the four corners on the work area** and drag its corners onto the corners of the material or of the work area (the picture follows with a perspective transform), or give **Reference points**, which are kept with the device. **Opacity**, **Show overlay**, **Reset to work area**, **Flip H**, **Flip V**, **Rotate 90°** and **Remove** are in the same window, and **View → Show camera overlay** shows or hides it. The picture is saved with the project. In the **Trace image** dialog the picture can be zoomed with the scroll wheel or the **Zoom in**, **Zoom out** and **Reset view** buttons, and the green **Trace** button makes the outlines.

## The side panel

* Drag a tab to put it in another place (the order is remembered, also in a backup). The small arrow at the left of the tabs hides the panel; a narrow strip with the names of the tabs stays, and a click on a name opens the panel on that tab.
* The tabs: **Properties**, **Layers**, **Device**, **Control**, **Console**, **Clipart**, **Description**, **Camera** (when the device has a camera that is switched on) and a tab for every plugin that has settings.

## Tabs and panes

Step by step: [Panes and Tabs](Panes-and-Tabs).

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

## Ruler, selection and the node bar

* **Ruler (M):** the tool after **Zoom out (X)** in the palette. Drag from one point to another, or click two points: a line over the drawing shows the length, ΔX, ΔY and the angle, in the units of the program. A point near a node of a shape takes the node itself (otherwise the grid when snapping is on). A click on a straight segment of a shape (a side of a rectangle, a line) measures that segment. The measurement stays until the next one, **Esc** or another tool; nothing in the drawing is changed.
* **Right click with a tool** (a shape, the pen, the node tool, the ruler…) works as a double **Esc**: back to **Select (V)** with nothing selected; a line being drawn with the pen is finished first. With the select tool the right click opens the context menu as before.
* **Several objects selected:** besides the frame of the whole selection, every object has a thin dashed frame of its own.
* **The node bar** shows the keys after the names: **Corner (C)**, **To line (L)**, **To curve (S)**, **Add node (I)**, **Delete node (D)**, **Round (R)**, **Break (B)**. **Open / close** is gone: **Break (B)** opens a path, and a path is closed by dropping one of its ends onto the other.
* **A layer that is not made** (its output switched off) shows its objects faded towards the work area.

## Preview: buttons and dimensions

* The options of the bar of the preview are **buttons**: light blue while they are on (as the button of the camera view), a click turns them on or off.
* **Dimensions** shows the sizes of the job as on a technical drawing: the width and height of the whole, of every part (when there are a few), and how far the whole is from the zero of the machine when the zero is in the picture.

* **Pictures:** TIFF (`.tif`, `.tiff`) is read too, as PNG, JPEG, BMP, GIF and WebP.

## Dimensions on the work area, the ruler and the node menu

* **Dimensions** (the button next to **Toolpaths** on the bar over the work area) shows the drawing as on a technical drawing: the width and height of the whole and of every outline (the outside and the holes of a shape each by itself), and the diameters of the round ones, the same ones named once (**4 × Ø 6**). It changes nothing in the drawing.
* **Ruler (M)** takes a node of a shape (a square marks it), the middle of a segment (a diamond) or the centre of a circle or an ellipse (a circle). A click on a circle shows its radius from the centre to the point clicked and its diameter; on an ellipse both radii and the radius to the point; on a curve its length, and when the curve is an arc of a circle, the radius drawn from the centre to the point clicked and its length.
* **Node edit:** a right click on a node or a segment opens the menu of the node bar; **Smooth** has the key **G**, and the radius of **Round (R)** stands after its button.

## Units

* **Settings → Program options… → Units** chooses the units the program shows lengths in: millimetres, centimetres, metres, inches or feet, or **As the device**. They are used by the sizes in **Properties**, the ruler, the **Dimensions** over the drawing and in the preview, and the technical drawing.
* **The units of the device** (millimetres or inches, in the device window) are those of the machine: the G-code is written in them. A device in inches gets **G20** and every length and feed in inches; a device in millimetres **G21** and millimetres. Make sure they are the units the controller works in.
* When the device counts in the other system than the program (inches against millimetres), the dimensions and the ruler show both: the units of the program first, those of the machine in brackets, for example **25.4 mm [1.000"]**.

## Drag knife

A plotter can cut with a **drag knife** (vinyl, paper, card, stencils) in place of a pen: in the device window, page **Pen**, tick **The tool is a drag knife**.

* **Knife:** a knife of the **knife library** sets the **offset of the tip** (how far the tip of the blade trails behind its axis), the corners and the overcut at once; the ready-made ones are the 30°, 45° and 60° blades (0.15, 0.25, 0.5 mm) and a drag knife in a holder of a CNC machine. The maker of the blade gives the exact offset. **Knife library…** (also in **Plotter → Knife library…**) opens the library: a ready-made knife cannot be changed, but **Copy** makes one of your own with a name, the values and notes, kept with the settings of the program (and in its backup). The values on the device stay its own: a change in the library does not change a device that took the knife earlier.
* The program makes good the offset: the axis goes ahead of the tip, and at a corner sharper than **Turn round corners sharper than** (15° by default) it goes round the corner on a small arc, so the blade turns on the spot and the corner stays sharp. Every cut starts by turning the blade from the way it pointed after the last cut, and a closed outline is cut on past its start by **Cut on past the start** (0.5 mm), so it closes.
* The holes are cut before the parts round them, so a part stays held until it is cut out. A knife only cuts outlines: fills and pictures are left out (the preview says so), and outlines smaller than about four offsets come out rounded (a warning).
* **The cutter makes good the offset itself:** most vinyl cutters (HPGL) do it in the controller; tick it and the outlines are cut as they are drawn.
* The blade goes down and up as the pen does: with the Z axis **down** is the depth of the cut.
* The preview draws the path of the axis in grey and what the tip cuts in the colour of the layer.

Not tried on a real cutter yet: try the first cut on a spare piece. The whole story, with the knife library and the HPGL commands of a cutter: [Drag Knife](Drag-Knife).

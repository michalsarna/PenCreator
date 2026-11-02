🌐 **English** · [Warstwy i pisaki](Warstwy-i-pisaki)

# Layers and Pens

A drawing has **64 layers**. Every shape lies on one layer, and **a layer is a pen**: its colour, the width of its line, its speed and the way the shapes on it are drawn (outlines, fill lines or a picture). The layers are drawn one after the other, and when the next layer needs another pen the job stops so that you can change it.

![The Layers tab: a pen is a layer](https://raw.githubusercontent.com/michalsarna/PenCreator/main/docs/screenshots/05-layers.png)

## The layers of a new drawing

| Layer | Name | Drawing |
|---|---|---|
| 1 | Black pen | **Outline** |
| 2 | Red pen | **Outline** |
| 3 | Blue pen | **Outline** |
| 4 | Green pen | **Outline** |
| 5 | Black fill | **Fill with lines** (black, like layer 1) |
| 6 … 64 | Layer 6 … Layer 64 | **Outline** |

All of them start with a **Pen width** of 0.4 mm, a **Speed** of 3000 mm/min, a **Fill line gap** of 0.4 mm at 45°, one pass and no pen of the library. A drawing opened from SVG or PDF, and a project of LightCreator or MillCreator, gets these layers too.

## The Layers tab

The top of the tab is the **list of layers**. It shows the layers that have shapes and the one you are working on; tick **Show all 64 layers** to see all of them. Each row has:

* **▲ / ▼** (**Earlier in the job** / **Later in the job**): the list is in the order of the job, from the top down; the arrows move the layer one place. The order is saved with the project.
* the colour of the layer;
* a checkbox, **Drawn: taken into the plot and the G-code**. A layer that is not ticked is left out of the preview, the toolpaths, the G-code and the job, but its shapes stay on the work area;
* the eye, **Shown in the work area**. A hidden layer is only hidden on the work area (and **Select all** leaves its shapes out); it is still drawn by the job when it is ticked;
* the name and the drawing of the layer: a click makes it the **active layer**, whose settings are shown below and where new shapes go;
* the number of shapes on the layer.

Under the list are **Save preset…** and **Load preset** (see [Layer presets](#layer-presets)) and the settings of the active layer:

| Setting | What it does |
|---|---|
| **Name** | The name of the layer. It is written into the G-code as a comment `(Layer: name)`, and is the variable `{layer_name}` of the G-code of a pen change. |
| **Colour** | The colour of the shapes on the work area. It is part of the pen: two layers of different colours are two pens (see [Pen changes](#pen-changes)). |
| **Drawing** | **Outline**, **Fill with lines**, **Fill and outline** or **Picture** (see below). |
| **Pen** | A pen of the [pen library](#the-pen-library), or **No pen chosen**. The **…** button opens the library. Choosing a pen sets the **Pen width** to the larger size of its mark. |
| **Pen width** | The width of the line the pen makes, 0.01 … 20 mm. Without a pen of the library it is the thickness of the line in the preview. |
| **Speed** | The speed of the pen on the paper, 1 … 100000 mm/min (the `F` of the `G1` moves). |
| **Picture method** | Only for **Picture**: **Dithered rows**, **Dots** or **Hatching by tone**. |
| **Dithering** | Only for **Dithered rows** and **Dots**: how the picture is turned into dots: **Threshold**, **Floyd-Steinberg**, **Jarvis**, **Stucki**, **Atkinson**, **Ordered** or **Grayscale** (each has a tip). |
| **Fill line gap** | For a fill and for a picture: the distance between the lines, 0.02 … 100 mm. |
| **Fill line angle** | For a fill and for a picture: the direction of the lines, −360 … 360°. |
| **Crossed lines** | For a fill: a second set of fill lines across the first. |
| **Drawn times** | How many times every line is drawn, 1 … 5: for a pen that is too light. An open line is drawn back and forth. |

Under the settings a line explains the chosen drawing, and **Put the selected shapes on this layer** moves the selected shapes there (locked shapes stay where they are).

## The kinds of drawing

* **Outline**: the pen follows the lines of the shapes. Open lines work too.
* **Fill with lines**: closed shapes are filled with parallel lines; holes stay empty.
* **Fill and outline**: the fill first, then the outline over it, so the edge is clean.
* **Picture**: a picture (bitmap) on the layer is drawn with lines: **Dithered rows** (every row of the dithered picture is drawn where it is dark), **Dots** (a dot for every dark cell) or **Hatching by tone** (crossed hatching: the darker the picture, the more directions of lines). The other shapes of such a layer are drawn as outlines. A picture that is put on a layer switches that layer to **Picture** once; you can change it back.

On a layer with another drawing a picture is not drawn: the preview and the status line say *picture(s) on the layers cannot be drawn: trace them to lines first* (**Arrange → Trace image…**).

The fill is drawn before the outlines, and the lines of a layer are put in an order that shortens the moves with the pen up. A very small gap makes a huge number of lines: a job makes at most 200 000 fill lines, the rest of the fill is left out and a warning says so.

## The row of layers under the work area

One swatch for each of the 64 layers, in a row that scrolls sideways (the arrows at its ends or the wheel). A dot marks a layer that has shapes, a faded swatch a hidden layer, a ring the active layer. The tip of a swatch gives its number and name.

* **A click** puts the selected shapes on that layer (locked shapes stay) and makes it the active layer; with nothing selected it only makes it the active layer.
* **A double click** opens the **Layers** tab on that layer.
* With **Settings → View options… → Show layer names** (off by default) a layer that has shapes shows its name beside its swatch (long names are cut; the whole name is in the tip).

## Layer presets

**Save preset…** and **Load preset** above the settings of a layer both open the window **Layer presets**:

* type a name and press **Save preset…** to keep the settings of the active layer under that name (at most 60 presets);
* the **Search** field narrows the list; a click on a preset puts its settings on the active layer; **✎** renames it and **✖** forgets it;
* **Export…** writes all the presets to a file and **Import…** adds those of a file.

A preset keeps everything but the name, the colour, the visibility, whether the layer is drawn and its place in the order: the drawing, the pen, the width, the speed, the picture method and dithering, the fill and the number of passes. A preset whose pen is not in the library any more leaves the layer without a pen (its width stays), and the status line says *The pen of the preset is not in the library: the layer has no pen*. The presets are kept in `pen_layer_presets.json` in the shared data folder (see [Settings and Files](Settings-and-Files)).

## The pen library

**Plotter → Pen library…** (or the **…** button next to **Pen** in the Layers tab) opens the window **Pen library**. On the left are the **Ready-made pens** and **My pens**; on the right a sample of the chosen pen (lines in four directions and a dot, drawn the way the pen makes them) and its settings.

### What a pen is

| Setting | What it does |
|---|---|
| **Name** | The name in the list, in the **Pen** of a layer and in the comments of the G-code. |
| **Type** | **Pencil**, **Fountain pen (nib)**, **Fineliner**, **Felt-tip marker**, **Highlighter** or **Brush pen**. |
| **Width of the mark** | The size of the tip across, 0.01 … 50 mm. |
| **Height of the mark** | The size of the tip along, 0.01 … 50 mm; the same as the width for a round tip. |
| **Turn of the tip** | The direction of the width of a chisel or a nib, in degrees. A line along the width is as thick as the width; a line across it as thin as the height. |
| **Ends of a line** | **Round ends** or **Flat ends**. |
| **Ink colour** | The colour of the line in the preview. |
| **Opacity** | 5 … 100 %: less is see-through (a highlighter), and where the lines cross they get darker. |

The pen of a layer changes the **Preview**: every line is drawn as thick as the pen makes it in its direction, with its ends, its ink colour and its opacity. A layer without a pen of the library is drawn in its own colour with round ends, as thick as its **Pen width**.

### Ready-made pens

| Pen | Type | Mark | Ends | Ink | Opacity |
|---|---|---|---|---|---|
| Fineliner 0.2 mm | Fineliner | 0.2 mm | round | black | 100 % |
| Fineliner 0.4 mm | Fineliner | 0.4 mm | round | black | 100 % |
| Fineliner 0.8 mm | Fineliner | 0.8 mm | round | dark blue | 100 % |
| Pencil 0.5 mm | Pencil | 0.5 mm | round | grey | 75 % |
| Fountain pen nib 1.8 x 0.3 mm | Fountain pen (nib) | 1.8 × 0.3 mm, turned 45° | flat | dark blue | 95 % |
| Felt-tip marker 1 mm | Felt-tip marker | 1 mm | round | red | 90 % |
| Chisel marker 3 x 1 mm | Felt-tip marker | 3 × 1 mm, turned 45° | flat | black | 95 % |
| Highlighter 4 x 1 mm | Highlighter | 4 × 1 mm, turned 45° | flat | yellow | 40 % |
| Brush pen 2 mm | Brush pen | 2 mm | round | black | 85 % |

A ready-made pen cannot be changed (*A ready-made pen cannot be changed. Make a copy to change it.*).

### Your own pens

* **New pen** adds a pen to **My pens** (a 0.4 mm black fineliner to start from); **Copy** makes a copy of the chosen pen among your own; **Delete** removes one of your own pens. A change in the fields is kept at once.
* **Import…** reads pens from a file (`.json`); every pen gets a name of its own (a number is added when the name is taken), and a pen that is not valid is left out and named in the message. **Export…** writes all your own pens to a file; **Export this pen…** writes only the chosen one (a ready-made one too).
* Your pens are kept with the settings of the program (at most 200). A project carries the pens of yours that its layers use: opened on another computer, they join **My pens** there.

## Pen changes

The pen of a layer is its **colour**, its **Pen width** and its **Pen** of the library together. The layers are drawn in the order of the list; when the next layer differs from the last one in any of the three, the pen is changed. Two layers with the same pen, colour and width are drawn one after the other without a stop; two layers that differ only in colour stop for a change.

* **Without a place for the change** (the default): the pen goes up and the job stops where it is, with `M0 (Change the pen to: …)`.
* **With a place** (device settings, page **Pen**, **Change the pen at a fixed place**): the pen goes up, the plotter drives to that place, sends the **G-code at the place** (for a mechanism that changes the pen; the variables `{layer_name}`, `{layer_index}` and `{total_layers}` can be used there) and, with **Stop there (M0)** ticked, stops with `M0 (Change the pen to: …)`. With it unticked only the comment `(Change the pen to: …)` is written and the job goes on. The way to the place is part of the preview and of the time.

The text after *Change the pen to:* names the pen of the library and the layer (`Fineliner 0.4 mm, Black pen`), or, without a pen of the library, the layer and its width (`Red pen, 0.4 mm`). When the job is sent from the program, the stop opens the window **The job is paused** with that text: change the pen, then press **Continue the job** (or **Stop the job**). See [Machine Control](Machine-Control).

With the job option **One element at a time** (Control tab) an element is drawn with all its layers before the next one: with different pens on those layers the pen is changed at every switch.

## The G-code

The program writes G-code for GRBL. With the defaults (the pen lifted by the Z axis, up 3 mm, down 0, a wait of 150 ms, the machine zero at the front left) it looks like this (the notes after ← are not in the file):

```
(PenCreator: name of the project)
(Device: name of the device)
(Note: the lines of the Description tab)
G21 G90 G17
G0 Z3            ← pen up (with a servo: M3 S<up>)
G4 P0.15         ← the wait after the pen moves, when it is not 0
(Layer: Black pen)
(Pen: Black pen, 0.4 mm)
G0 X… Y…         ← a move with the pen up
G0 Z0            ← pen down (with a servo: M3 S<down>)
G4 P0.15
G1 X… Y… F3000   ← the lines, at the speed of the layer
…
(Layer: Red pen)
M0 (Change the pen to: Red pen, 0.4 mm)
…
G0 X0 Y0         ← pen up, back to the front left corner of the work area
M5               ← only with a servo
M30
```

* The notes of the project (the **Description** tab) become `(Note: …)` lines after the name: brackets are turned into `<` `>`, `;` and `%` into commas, empty lines are left out, a long line is cut into pieces of 100 characters, and at most 60 lines are written.
* The numbers are in millimetres, in the coordinates of the machine: the corner you chose as **Machine zero (0,0)** of the device is X0 Y0.
* The **Very start of the file**, the **Very end of the file** and the **Own file format** of the device (page **G-code**) and the postprocessors of plugins are applied last. See [Machine Control](Machine-Control#running-a-job) and [Plugins](Plugins).

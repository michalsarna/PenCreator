🌐 **English** · [Polski](Noz-wleczony)

# Drag knife

A plotter can **cut** in place of drawing: vinyl, flex, paper, card, stencils. The tool is a **drag knife**: a blade that turns freely in its holder, its tip trailing a little behind the axis of the holder, the way a castor of a chair trails behind its pin. PenCreator moves the axis so that the **tip** goes along your lines.

## How the knife is made good

* **Offset of the tip:** how far the tip of the blade is behind its axis (0.15 to 0.5 mm for the usual blades; the maker of the blade gives it). The program sends the axis ahead of the tip by that much, so the cut lies on the line and not beside it.
* **Turn round corners sharper than** (15° by default): at a sharper corner the axis goes round it on a small arc, the blade turns on the spot and the corner stays sharp. A gentler corner is cut straight through.
* **Cut on past the start** (0.5 mm by default): a closed outline is cut on this far past its start, so it closes and the part comes out.
* **The way the blade points** is carried from one cut to the next: every cut starts by turning the blade from where it was left.
* **Holes first:** the holes are cut before the parts round them, so a part stays held by the material until it is cut out.
* Only **outlines** are cut: fills and pictures are left out (the preview says so). An outline smaller than about four offsets comes out rounded; the program warns about it.

## The knife library

**Plotter → Knife library…** (or the **Knife library…** button next to the knife of a device) opens the library. On the left: **Ready-made knives** and **My knives**; on the right the values of the knife chosen.

| Ready-made knife | Offset of the tip | Turn round corners sharper than | Cut on past the start | For |
|---|---|---|---|---|
| 30° blade | 0.15 mm | 15° | 0.4 mm | thin film and paper, fine detail and small letters |
| 45° blade | 0.25 mm | 15° | 0.5 mm | vinyl, flex, paper and thin card: the usual blade of a cutter |
| 60° blade | 0.5 mm | 15° | 0.8 mm | thick vinyl, card, rubber, foam and sandblast stencils |
| Drag knife in a holder of a CNC machine | 0.5 mm | 10° | 1.0 mm | a holder in the collet of the spindle (see MillCreator) |

* A ready-made knife cannot be changed. **Copy** puts a copy of it among **My knives**, where you give it a **Name**, the values and **Notes** (the blade, the material, the force that worked). **Delete** removes a knife of your own.
* Your knives are kept with the settings of the program and in its backup (**Settings → Backup**).
* A device **copies** the values of a knife: a later change in the library does not change a device that took the knife earlier. When the values of a device match no knife of the library the list shows **Own values**.

## Setting up a cutter

1. Open the device window and go to the **Pen** page.
2. Tick **The tool is a drag knife**.
3. Pick the **Knife** from the list, or set its values by hand.
4. **The cutter makes good the offset itself:** most vinyl cutters (HPGL) do it in their controller. Tick it and the outlines are cut as they are drawn (still holes first); leave it off for a plotter that does not (a GRBL plotter with a knife holder).
5. The blade goes down and up as the pen does (the Z axis, the servo or your own G-code): with the Z axis, **Pen down** is the depth of the cut.

The **preview** draws the path of the axis in grey and what the tip cuts in the colour of the layer.

## HPGL cutters: the commands of the device

Plotters and cutters understand HPGL a little differently. When the controller of a device is **HPGL plotter (not GRBL)** (page **Connection**), the device window has an **HPGL** page, only for such devices, where you set what this one gets. Commands end with `;`; `{speed}` and `{force}` are filled in.

| Field | Default | When it is sent |
|---|---|---|
| **At the start** | `IN;SP1;PU;` | once, when the plotter connects |
| **At the end** | `PU;` | at the end of the job (`M2`, `M30`), after the knife is lifted |
| **Pen or knife down** | `PD;` | before every cut |
| **Pen or knife up** | `PU;` | after every cut and on a stop |
| **Speed** | `VS{speed};`, **at most** 38 | when the speed of the drawing changes, never above the top speed; empty: never (the cutter keeps its own) |
| **Force** | `FS{force};`, 0 g | once at the start; 0: never (the cutter keeps its own) |

**Back to the usual commands** puts the defaults back. A device made before this page gets the defaults. The start and the force are sent when the plotter connects: after you change them, connect again.

## The first cut

Cut a small test (a square with a circle in it) on a spare piece first. A rounded corner means the offset is too small, a little loop or a hook at a corner means it is too large; an outline that does not close wants a longer **Cut on past the start**. When it is right, **Copy** the knife into **My knives** with the values and the force in its notes.

See also: [Design Tools](Design-Tools#drag-knife), [Machine Control](Machine-Control), [Safety and Limitations](Safety-and-Limitations).

# Features

## Available now

* **Devices (plotters):** a list on the start screen (add, edit, duplicate, export and import as files, delete), kept between runs. Sheet size (work area), machine zero (any corner), the speed of the moves with the pen up, how the pen is lifted (Z axis or a servo with `M3 S…`) with the up and down values and a wait, notes, and up to 8 cameras (an address or a camera of the computer) with a live Camera tab. **I have no device** makes the Dummy; **New** and **Open…** without a device use it.
* **Pens as layers:** 16 layers with a colour, a pen width, a speed, outline / fill with lines / both, the angle and gap of the fill lines, crossed lines, and how many times every line is drawn. Layers can be switched off (not drawn) and hidden (not shown).
* **The plotter:** a GRBL controller on a serial port or a network socket (set in the device: port, baud rate, streaming, homing or clearing an alarm after connecting); Connect, Home, Unlock, Pause / Resume, the state and position, a **Console** tab (the traffic with the controller, a line to send a command); Frame (the pen goes round the box of the drawing, lifted), Start and Stop of the job and Save G-code on the control bar. The Toolpaths button draws the paths over the work area; Camera view shows the live camera.
* **Plot and G-code:** strokes in the order that shortens the moves with the pen up, fill lines that zigzag and leave holes empty, a preview window (zoom, pan, an animation with play, pause, speed and repeat, length, pen lifts, time) and G-code for GRBL (`M0` to change the pen between layers, the machine zero by corner). Pictures cannot be drawn and are reported.
* **Project file:** `.cre`, shared with LightCreator and MillCreator: the drawing, the description, the `pen` section (layers and the plotter it was made for, with a warning when it is opened for another sheet size) and the sections of other programs, which are kept as they were.
* The window of LightCreator's look: own frame and title bar, start screen, control bar, status bar, a Properties tab, the clipart gallery (310 included pictures, your own, online search), a Description tab, the colour schemes, fonts and 30 languages (PenCreator's own texts too).
* Reading SVG, PDF and Illustrator (PDF-compatible) files and LightBurn projects (`.lbrn2`: shapes, text outlines, bitmaps) into a scene of shapes.
* The editor of baseCreator: drawing tools, selection, node editing, undo and redo, copy and paste, group, align, flip, rotate, order, grid and snapping.
* Menus (with the native macOS menu bar and recent files), icons, tool palette, arrange bar, View options and the About window of baseCreator.

## Planned

Nothing below exists yet; the list is a plan, not a promise.

* Jogging and probing.
* HPGL output.
* Text in single-line fonts.
* Pen pressure and a second pen holder; hatching that follows the shape.

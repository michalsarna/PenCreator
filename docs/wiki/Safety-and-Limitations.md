🌐 **English** · [Polski](Bezpieczenstwo-i-ograniczenia)

# Safety and Limitations

PenCreator makes plotting paths and writes **G-code for GRBL**. It can **connect to a GRBL plotter and stream the job** (Device tab: **Connect**; Control tab: **Start**, **STOP**, **Frame**, **From line…**), **but this has not been tried on a real plotter**; you can also save the file and send it with your own program. The G-code has been checked by automatic tests and in a GRBL simulator (the pen is up before every move with the pen up, a pen change stops with `M0`), but **it has not been tried on a real plotter**.

* Set **Pen up** and **Pen down** of the device with care: with the Z axis they are heights in mm (down 0 is the paper), with a servo they are `S` values. Wrong values can press a pen into the machine or the table. Try **Raise the pen** and **Lower the pen** on the Control tab first, run a job with the pen lifted or without a pen, and stay at the machine.
* The pen starts at the machine zero of the device (the corner you choose); the first move goes from there, and at the end the pen goes back to that corner.
* Jogging, the pen buttons, **Frame** and the pauses for a pen change send real moves. **Frame** lifts the pen and goes round the box of the drawing: check that the lifted pen is clear of the paper and of the clamps.
* A picture (bitmap) is drawn only on a layer whose **Drawing** is **Picture**; on any other layer it is left out with a warning (trace it to lines first, or switch the layer to **Picture**). Text is drawn as the outlines of the font (each letter round its edge), not with single-line letters.
* Fill lines are parallel lines at an angle; a very small gap makes a huge number of lines, and after 200 000 fill lines the rest of the fill is left out with a warning.
* No HPGL and no single-line fonts yet.
* On macOS some keys of the menus (Cmd+E, Cmd+Shift+P, Cmd+L and others) do not run their commands yet; use the menus. See [Keyboard Shortcuts](Keyboard-Shortcuts#on-macos).
* What is tested: the editor and the project file (the sections of other programs are kept), the opening of files, devices, layers, the order of the strokes, the fill, the G-code, the menus and their keys, and the translations, by automatic tests on every change; the G-code of jobs also in a simulated GRBL controller. The screens were checked in the headless renderer, not on a real machine.

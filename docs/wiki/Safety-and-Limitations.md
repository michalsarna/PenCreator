🌐 **English** · [Polski](Bezpieczenstwo-i-ograniczenia)

# Safety and Limitations

PenCreator makes plotting paths and writes **G-code for GRBL**. It can **connect to a GRBL plotter and stream the job** (Device tab: Connect; Control tab: Start, Stop, Frame), **but this has not been tried on a real plotter**; you can also save the file and send it with your own program. The G-code has been checked by automatic tests (the pen is up before every move with the pen up, a pen change stops with `M0`) but **has not been tried on a real plotter**.

* Set **Pen up** and **Pen down** of the device with care: with the Z axis they are heights in mm (down 0 is the paper), with a servo they are `S` values. Wrong values can press a pen into the machine or the table. Try a job with the pen lifted first, and stay at the machine.
* The pen starts at the machine zero of the device (the corner you choose); the first move goes from there.
* Pictures (bitmaps) on the layers cannot be drawn; trace them to lines first. Text is drawn as the outlines of the font (double lines), not single-line letters.
* Fill lines are parallel lines at an angle; a very small gap makes a huge number of lines and part of the fill is left out with a warning.
* No jogging and no HPGL yet. Frame lifts the pen and goes round the box of the drawing: check that the lifted pen height is clear.
* What is tested: the editor and the project file (the sections of other programs are kept), the opening of files, devices, layers, the order of the strokes, the fill, the G-code and the translations, by automatic tests on every change. The screens were checked in the headless renderer, not on a real machine.

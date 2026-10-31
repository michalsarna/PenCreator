🌐 **English** · [Polski](Sterowanie-maszyna)

# Machine control

PenCreator drives a GRBL machine from the **Control** tab and the **Machine** menu. All of it comes from baseCreator, so MillCreator, PenCreator and LightCreator behave the same. **Nothing here has been tried on a real machine yet**: test with the tool lifted or no pen, and keep a hand on the stop.

## Reading the machine

Connect on the **Device** tab. The program reads the controller (`$I` for its firmware, `$$` for its settings, `$G` for its state). In the device settings, on the page **Machine**, **Use these values for the device** copies the travel, and the speed of moves from the controller into the device, so you need not type them. New devices can also start from **A usual sheet size…** (a starting point only).

## The Control tab

* **Position:** the work position and the machine position of X, Y and Z, in big figures. **X0 Y0 Z0** (on the left of each axis) make the place where the tool is the zero of the work (`G10 L20`); click a number to type the position. Under them: **Zero all**, **Zero X and Y**, **Go to X0 Y0** (lifts to the safe height first) and **Z to safe height**. **Coordinates** picks G54 … G59.
* **Move:** the distance of one click for XY and for Z is a field you can type in; the buttons next to it put a value there (XY 0.1 … 100, Z 0.1 … 20; the values are set in the device settings, page **Jogging**). **Speed** has the XY and Z speeds of the device (a change here lasts until the device's own change).
* **Jog buttons** (big, in the middle): the arrows, **Z▲ Z▼**; the square stops the move. One click is one move. With **Hold to move** the tool moves while the button is held down and stops when it is let go. **Keyboard** (off by default, because the arrows also move shapes) uses the arrow keys and Page Up / Page Down by the chosen step.
* **Raise the pen / Lower the pen** (under Home, Unlock, Pause and Resume): move the pen where it stands, with the Z axis or the servo of the device, to the *up* or the *down* position set on the page **Pen** (and wait for the delay set there). Handy to set the pen height by hand before a job; they are off while a job runs.
* **Macros:** buttons that send G-code you wrote (page **Macros** of the device settings). Variables in brackets are replaced when the macro is sent: `[posx] [posy] [posz]` (work position), `[mposx]` …, `[xmin] [xmax] [ymin] [ymax]` (the box of the job), `[safez]`, `[prbx] [prby] [prbz]` (the last probe). `$RST` is not allowed in a macro. The same page has G-code that runs **before** and **after** every job (also after Stop).

## Running a job

* **Start** streams the whole job; **From line…** (also in the menu *Machine*) starts at a line of the program: the machine is first brought into the state the program has there (units, coordinate system, pen, feed, position), at the safe height, and the rest follows. It refuses when it cannot work that out (for example after `G53` or `G92`).
* **Check the program** (menu *Machine*, or *Check the program before every run* on the page **Jogging**): the controller goes through the program in check mode (`$C`). Nothing moves, and the lines it does not understand are reported with their line number. With the option on, the real job starts only when the check found nothing.
* **Job options** (under Frame and From line…; this session only, not saved; used together or alone, they change the G-code, the preview and the estimate): **One element at a time** draws each element, a group (even over several layers) or a shape that is in no group, completely, with all its layers and all their passes, before the next one, the nearest element to the start corner first (off: a layer after a layer). It is meant for layers that use the same pen: with different pens the pen is changed (a pause) at every switch. **Selection only** draws just the shapes that are selected on the work area when the G-code is saved or the job is started; nothing selected means nothing to draw.
* **Change of the pen at a fixed place:** in the device settings, page **Pen**, switch on *Change the pen at a fixed place* and give the place (X and Y, in the coordinates of the work area; it may lie outside of it, e.g. where a holder or a mechanism stands). When the next layer needs another pen, the pen goes up, the plotter drives to that place, sends the *G-code at the place* (up to 20 lines, for a mechanism that changes the pen by itself) and stops there (`M0`) so that the pen can be changed by hand or by the mechanism; *Stop there (M0)* can be switched off for a mechanism that needs no waiting. The way to the place is part of the preview and the estimate. Without a place the job stops where it is. This was tried only in the simulator, not on a machine with a pen changer.
* **Preview as a tab:** the icon with a tab in the title line of the preview window (left of the close button) pins the preview as a tab of the work area, next to the drawings and the camera view. In the tab the icon with a small window makes it a window again; the cross of the tab closes the preview. The *Preview…* button opens it again the way it was last (window or tab).
* **Pauses:** When the program reaches a change of the pen, the machine stops there and a window says which pen to put in. **Continue the job** goes on, **Stop the job** ends it. While the job waits you can jog, zero and probe on the Control tab.
* Errors and alarms that come during a job are counted and the first line of each is remembered.

## Machine menu

* **Firmware settings ($$):** the settings of the controller, with search and a filter that shows only those that differ from the typical value. A change is written after a question; the settings are read again afterwards. **The first change of a session saves all settings to a file** in the data folder (`firmware-backups`). **Back up to a file…** and **Restore from a file…** do it by hand (a file is compared with the controller before anything is written, and read back afterwards); **Factory settings ($RST=$)…** needs the word RESET.
* **Calibration:** *steps per millimetre* (ask for a move, measure it, get the new `$100` / `$101` / `$102`, write it) and *squareness of the axes* (type the two diagonals of a rectangle: the angle and the shift are shown).
* **Diagnostics and statistics:** the input pins of the controller (limit switches, probe, door, hold, reset, start), the firmware, jobs, running time, alarms and errors of this machine, and **maintenance reminders** (for example "oil the rails every 50 hours"; set them on the page **Maintenance**). **Export diagnostics (zip)…** saves a report, the settings, the console and the program.

## Copies of the controller settings

Every device keeps copies of the settings of its controller (`$$`). One is made the first time the machine is connected, and again whenever the settings read at connecting differ from the newest copy: the status bar and the console say how many settings changed and which (`$110: 3000 → 5000`), so you notice a change made from another program or by the controller itself. A change made here (a write in **Firmware settings**) gets its own copy without an alarm. **Make a copy now** in the window of the firmware settings, and the same in the device settings, page **Machine**, make one by hand; **Restore…** compares a copy with the controller and writes the differences back after a question; **Delete** removes one. The newest eight are kept. The copies are part of the device, so they are in the **backup of the program** and in an exported device file.

## Alarms and errors

**GRBL versions.** Choose the version of the board under *GRBL version* on the device's Connection page: **1.1 or newer** (the default; also grblHAL and FluidNC), **1.0**, **0.9**, **0.8** or **0.7 or older**. GRBL 0.8, 0.9 and 1.0 report the position as `<Idle,MPos:…,WPos:…>` and have no `$J=` jog command, so the program jogs with ordinary `G91` / `G1` moves; such a jog cannot be cancelled halfway, it runs to the end of its step. GRBL 0.7 has no real-time commands at all: no status report (so no position), no Pause, Resume, Home or Unlock, and Stop only drops the lines not yet sent while the board finishes what it already has; that firmware also cuts lines longer than 49 characters. A servo that lifts the pen on the spindle output (`M3 S…`) works the same with every version.

When GRBL says `ALARM:n` or `error:n`, the console gets a line under it that explains it, and the status bar shows it. The explanations are in the 30 languages of the program.

> ⚠️ Probing, jogging and the pauses send real moves. Keep the stop within reach.

## When the machine stops

An alarm, an error in a job or a controller that does not answer opens a **window in the middle of the screen**: what happened, and two big buttons, **STOP** (red: stops the job and the spindle) and **UNLOCK** (orange: clears the alarm, `$X`). Look at why it stopped before you unlock: the tool may be in the material.

## Overrides and saved positions

With a GRBL 1.1 or newer controller that reports them, the Control tab has **Overrides**: the feed can be changed in steps of 10 % and 1 % while a job runs, and the rapids slowed to 50 % or 25 %. **Saved positions** remembers where the tool is (machine position, with a name) and goes back there with the pen up first; there is no Z axis, so only X and Y move. The list is kept in `saved_positions.json` in the shared data folder. **Smoothieware** is a firmware choice of the Connection page; it is streamed like GRBL and untried on a real board.

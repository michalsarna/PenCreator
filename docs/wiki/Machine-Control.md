🌐 **English** · [Polski](Sterowanie-maszyna)

# Machine control

PenCreator drives a GRBL machine from the **Control** tab and the **Machine** menu. All of it comes from baseCreator, so MillCreator, PenCreator and LightCreator behave the same. **Nothing here has been tried on a real machine yet**: test with the tool lifted or no pen, and keep a hand on the stop.

## Reading the machine

Connect on the **Device** tab. The program reads the controller (`$I` for its firmware, `$$` for its settings, `$G` for its state). In the device settings, on the page **Machine**, **Use these values for the device** copies the travel, and the speed of moves from the controller into the device, so you need not type them. New devices can also start from **A usual sheet size…** (a starting point only).

## The Control tab

* **Position:** the work position and the machine position of X, Y and Z. **X0 Y0 Z0** make the place where the tool is the zero of the work (`G10 L20`); click a number to type the position. **Coordinates** picks G54 … G59.
* **Move:** step buttons (their sizes are set in the device settings, page **Jogging**), the arrows and **Z▲ Z▼**; the square in the middle stops the move. With **Hold to move** the tool moves while the button is held down and stops when it is let go. **Keyboard** (off by default, because the arrows also move shapes) uses the arrow keys and Page Up / Page Down.
* **Go to X0 Y0** lifts to the safe height first; **Z to safe height**.
* **Macros:** buttons that send G-code you wrote (page **Macros** of the device settings). Variables in brackets are replaced when the macro is sent: `[posx] [posy] [posz]` (work position), `[mposx]` …, `[xmin] [xmax] [ymin] [ymax]` (the box of the job), `[safez]`, `[prbx] [prby] [prbz]` (the last probe). `$RST` is not allowed in a macro. The same page has G-code that runs **before** and **after** every job (also after Stop).

## Running a job

* **Start** streams the whole job; **From line…** (also in the menu *Machine*) starts at a line of the program: the machine is first brought into the state the program has there (units, coordinate system, pen, feed, position), at the safe height, and the rest follows. It refuses when it cannot work that out (for example after `G53` or `G92`).
* **Check the program** (menu *Machine*, or *Check the program before every run* on the page **Jogging**): the controller goes through the program in check mode (`$C`). Nothing moves, and the lines it does not understand are reported with their line number. With the option on, the real job starts only when the check found nothing.
* **Pauses:** When the program reaches a change of the pen, the machine stops there and a window says which pen to put in. **Continue the job** goes on, **Stop the job** ends it. While the job waits you can jog, zero and probe on the Control tab.
* Errors and alarms that come during a job are counted and the first line of each is remembered.

## Machine menu

* **Firmware settings ($$):** the settings of the controller, with search and a filter that shows only those that differ from the typical value. A change is written after a question; the settings are read again afterwards. **The first change of a session saves all settings to a file** in the data folder (`firmware-backups`). **Back up to a file…** and **Restore from a file…** do it by hand (a file is compared with the controller before anything is written, and read back afterwards); **Factory settings ($RST=$)…** needs the word RESET.
* **Calibration:** *steps per millimetre* (ask for a move, measure it, get the new `$100` / `$101` / `$102`, write it) and *squareness of the axes* (type the two diagonals of a rectangle: the angle and the shift are shown).
* **Diagnostics and statistics:** the input pins of the controller (limit switches, probe, door, hold, reset, start), the firmware, jobs, running time, alarms and errors of this machine, and **maintenance reminders** (for example "oil the rails every 50 hours"; set them on the page **Maintenance**). **Export diagnostics (zip)…** saves a report, the settings, the console and the program.

## Copies of the controller settings

Every device keeps copies of the settings of its controller (`$$`). One is made the first time the machine is connected, and again whenever the settings read at connecting differ from the newest copy: the status bar and the console say how many settings changed and which (`$110: 3000 → 5000`), so you notice a change made from another program or by the controller itself. A change made here (a write in **Firmware settings**) gets its own copy without an alarm. **Make a copy now** in the window of the firmware settings, and the same in the device settings, page **Machine**, make one by hand; **Restore…** compares a copy with the controller and writes the differences back after a question; **Delete** removes one. The newest eight are kept. The copies are part of the device, so they are in the **backup of the program** and in an exported device file.

## Alarms and errors

When GRBL says `ALARM:n` or `error:n`, the console gets a line under it that explains it, and the status bar shows it. The explanations are in the 30 languages of the program.

> ⚠️ Probing, jogging and the pauses send real moves. Keep the stop within reach.

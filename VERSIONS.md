# Versions

Each release gets the next version number. Newest first.

## v0.2.4

* One data folder for all programs: `baseCreator` (macOS `~/Library/Application Support/baseCreator`). The clipart gallery is now **shared** by LightCreator, MillCreator and PenCreator; settings, device profiles, materials and tools stay separate per program in `apps/<program>/app.ron`.
* The first start after the update moves the old settings and pictures into the shared folder (the old folder is left untouched, identical pictures are not duplicated). `BASECREATOR_DIR` selects another folder.
* The gallery index is written safely and re-read, so two programs open at once do not lose each other's pictures.
* **Help → Download backup…** and **Restore from backup…**: one zip with the settings of PenCreator and the shared gallery. Restoring replaces the settings and adds the pictures to the gallery (nothing is lost or doubled); a backup made by another program is refused. The same entries as in LightCreator, made in baseCreator.
* baseCreator v0.9.4.

## v0.2.3

* A **Console** tab, as in LightCreator: the traffic with the controller, status polls on request, auto-scroll, clear and a line to send a command. It replaces the small console of the Device tab.
* baseCreator v0.9.2.

## v0.2.2

* The control bar has **Toolpaths**, **Preview…**, **Camera view**, **Frame**, **Start** / **Stop** and **Save G-code**, as in MillCreator; the device has a Connection page (serial port or network socket) and the Device tab connects to a GRBL plotter, homes, pauses and shows a console.
* **The animation of the preview works** (its buttons were inert).
* The preview window no longer has an *Export G-code…* button.
* baseCreator v0.9.1.

## v0.2.1

* The preview window is the same as MillCreator's and has an **animation**: back to the start, play / pause, speed (×0.25 to ×25) and repeat, with the time and the length.
* The pointer is the tool's only over the work area; the line of the side panel and the line between the list of layers and their options light up red and thicken under the pointer, and the latter is dragged to resize the list.
* baseCreator v0.9.0.

## v0.2.0

* Devices with the start screen of MillCreator (list, add, edit, duplicate, export, import, **I have no device**; **New** and **Open…** without a device use the Dummy), cameras with a Camera tab.
* 16 pen layers (colour, width, speed, outline / fill with lines / both, crossed lines, draw several times), the `pen` project section.
* The plot window and G-code for GRBL (Z axis or servo, `M0` between pens).
* PenCreator's own texts in 30 languages.
* The device section of the start screen is centred: *Add device…*, *Edit…*, *Duplicate*, *Import…* and, in a second row, *I have no device*; *Export…* and *Delete* are in the window of *Edit…*. baseCreator v0.8.1.

## v0.1.0

* First release: an early skeleton. The window, menus and editor of baseCreator, the clipart gallery, SVG / PDF / Illustrator and LightBurn import, the shared `.cre` project file with the Description tab; no plotting yet.

🌐 **English** · [Polski](Ustawienia-i-pliki)

# Settings and Files

The settings of PenCreator are stored in one file, `app.ron`: the language, the colour scheme, the grid and the view options, the recent files, **your devices** (key `devices`), **your own pens** (key `pens`), the order of the tabs of the side panel and whether it is hidden, the layout of the panes, and whether to look for updates.

| System | Location |
|---|---|
| macOS | `~/Library/Application Support/baseCreator/apps/pencreator/app.ron` |
| Windows | `%APPDATA%\baseCreator\apps\pencreator\app.ron` |
| Linux | `$XDG_DATA_HOME/baseCreator/apps/pencreator/app.ron`, by default `~/.local/share/baseCreator/apps/pencreator/app.ron` |

* **Shared folder.** LightCreator, MillCreator and PenCreator keep their data in one folder, `baseCreator` (the table shows the program's own settings inside it). Set the environment variable `BASECREATOR_DIR` to use another folder. In it:
  * `clipart/`: the **clipart gallery, shared** by the programs: a picture saved or downloaded in one program shows up in the others (with an `index.json` recording name, category, source, licence and author);
  * `apps/pencreator/app.ron`: the settings above; `apps/pencreator/firmware-backups/`: the copies of the controller settings made before the first change of a session;
  * `pen_layer_presets.json`: the [layer presets](Layers-and-Pens#layer-presets);
  * `saved_positions.json`: the saved positions of the Control tab;
  * `camera_recording.json`: the folder, the video format and the time-lapse settings of the camera; `camera/`: the photos and videos of a project that has no file yet;
  * `plugins/`: the [plugins](Plugins).
* **Devices** can be saved to a file and read again: **Export…** in the window of the device settings, **Import…** on the start screen.
* **Pens** can be saved to a file and read again in **Plotter → Pen library…** (**Export…**, **Export this pen…**, **Import…**).
* The settings are written every half minute and when the program closes. **Delete `app.ron` to reset everything.**
* **Automatic saving:** about every 20 seconds after a change (and when the program closes), a project that has a file is written to that file; a drawing that has no file yet is written to a recovery file in the shared folder. The next start offers such work (*A project that was not saved was found*: **Recover** or **Discard it**); saving the drawing under a name removes its recovery file.
* **Help → Download backup…** saves a zip with the settings of PenCreator (the list of devices, your pens, the language, the colour scheme, the view options, the recent files) and the whole shared clipart gallery, with a README. **Help → Restore from backup…** reads such a file back after a confirmation: the settings are replaced and the pictures are **added** to the gallery, so nothing saved by another program in the meantime is lost. A backup made by another program is refused. LightCreator and MillCreator have the same two entries.
* Projects (`.cre`), G-code and exported SVG or DXF are saved wherever you choose. The photos and videos of the camera go next to the project (see [Machine Control](Machine-Control#camera-rtsp-photos-video-and-time-lapse)).
* The **Settings** menu has **View options…**, **Program options…** (**Look for a newer version when the program starts**), **Plugins…**, the **Language** (30) and the **Colour scheme** (14).

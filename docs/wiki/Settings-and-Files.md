🌐 **English** · [Polski](Ustawienia-i-pliki)

# Settings and Files

Language, colour scheme, view options, recent files and **your devices** (key `devices`) are stored in one file, `app.ron`:

| System | Location |
|---|---|
| macOS | `~/Library/Application Support/baseCreator/apps/pencreator/app.ron` |
| Windows | `%APPDATA%\baseCreator\apps\pencreator\app.ron` |
| Linux | `$XDG_DATA_HOME/baseCreator/apps/pencreator/app.ron`, by default `~/.local/share/baseCreator/apps/pencreator/app.ron` |

* **Shared folder.** LightCreator, MillCreator and PenCreator keep their data in one folder, `baseCreator` (the table shows the program's own settings inside it). The **clipart gallery is shared**: a picture saved or downloaded in one program shows up in the others (the `clipart` folder next to `apps`, with an `index.json` recording name, category, source, licence and author). Each program keeps its own settings, device profiles, materials and tools in `apps/<program>/app.ron`. Set the environment variable `BASECREATOR_DIR` to use another folder. A program started for the first time after the update moves its old settings and pictures in (the old folder is left untouched, identical pictures are not duplicated).
* A device can also be saved to a file (**Export…**) and read again (**Import…**) on the start screen.
* The file is written when the program closes. **Delete it to reset everything.**
* Projects (`.cre`) and SVG are saved wherever you choose.
* **Settings** has the language (30 for the shared texts) and the colour scheme (14), and **View options…**

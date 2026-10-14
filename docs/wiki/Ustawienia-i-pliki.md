🌐 [English](Settings-and-Files) · **Polski**

# Ustawienia i pliki

Język, schemat kolorów, opcje widoku, ostatnie pliki i **Twoje urządzenia** (klucz `devices`) są w jednym pliku, `app.ron`:

| System | Położenie |
|---|---|
| macOS | `~/Library/Application Support/baseCreator/apps/pencreator/app.ron` |
| Windows | `%APPDATA%\baseCreator\apps\pencreator\app.ron` |
| Linux | `$XDG_DATA_HOME/baseCreator/apps/pencreator/app.ron`, domyślnie `~/.local/share/baseCreator/apps/pencreator/app.ron` |

* Twoje cliparty leżą obok, w folderze `clipart`.
* Urządzenie można też zapisać do pliku (**Eksportuj…**) i wczytać ponownie (**Importuj…**) na ekranie startowym.
* Plik jest zapisywany przy zamknięciu programu. **Usuń go, żeby wszystko zresetować.**
* **Help → Download backup…** zapisuje plik zip z ustawieniami programu PenCreator (lista urządzeń, język, schemat kolorów, ostatnie pliki) i całą wspólną galerią clipartów, z README. **Help → Restore from backup…** wczytuje taki plik po potwierdzeniu: ustawienia są zastępowane, a obrazki **dodawane** do galerii, więc nic, co w międzyczasie zapisał inny program, nie ginie. Kopia zrobiona w innym programie jest odrzucana. LightCreator ma te same dwie pozycje.
* Projekty (`.cre`) i SVG zapisują się tam, gdzie wskażesz.
* **Settings** ma język (30 dla wspólnych tekstów) i schemat kolorów (14) oraz **View options…**

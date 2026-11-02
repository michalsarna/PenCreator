🌐 [English](Settings-and-Files) · **Polski**

# Ustawienia i pliki

Ustawienia PenCreatora są w jednym pliku, `app.ron`: język, schemat kolorów, siatka i opcje widoku, ostatnie pliki, **Twoje urządzenia** (klucz `devices`), **Twoje własne pisaki** (klucz `pens`), kolejność zakładek panelu bocznego i to, czy jest schowany, układ paneli oraz to, czy szukać aktualizacji.

| System | Położenie |
|---|---|
| macOS | `~/Library/Application Support/baseCreator/apps/pencreator/app.ron` |
| Windows | `%APPDATA%\baseCreator\apps\pencreator\app.ron` |
| Linux | `$XDG_DATA_HOME/baseCreator/apps/pencreator/app.ron`, domyślnie `~/.local/share/baseCreator/apps/pencreator/app.ron` |

* **Wspólny folder.** LightCreator, MillCreator i PenCreator trzymają dane w jednym folderze, `baseCreator` (tabela pokazuje ustawienia programu w jego wnętrzu). Zmienna środowiskowa `BASECREATOR_DIR` wskazuje inny folder. W nim:
  * `clipart/`: **galeria clipartów, wspólna** dla programów: obrazek zapisany lub pobrany w jednym programie pojawia się w pozostałych (z plikiem `index.json` z nazwą, kategorią, źródłem, licencją i autorem);
  * `apps/pencreator/app.ron`: ustawienia opisane wyżej; `apps/pencreator/firmware-backups/`: kopie ustawień kontrolera robione przed pierwszą zmianą w sesji;
  * `pen_layer_presets.json`: [presety warstw](Warstwy-i-pisaki#presety-warstw);
  * `saved_positions.json`: zapisane pozycje karty Sterowanie;
  * `camera_recording.json`: folder, format wideo i ustawienia poklatkowego kamery; `camera/`: zdjęcia i filmy projektu, który nie ma jeszcze pliku;
  * `plugins/`: [wtyczki](Wtyczki).
* **Urządzenia** można zapisać do pliku i wczytać ponownie: **Eksportuj…** w oknie ustawień urządzenia, **Importuj…** na ekranie startowym.
* **Pisaki** można zapisać do pliku i wczytać ponownie w **Ploter → Biblioteka pisaków…** (**Eksportuj…**, **Eksportuj ten pisak…**, **Importuj…**).
* Ustawienia są zapisywane co pół minuty i przy zamknięciu programu. **Usuń `app.ron`, żeby wszystko zresetować.**
* **Automatyczny zapis:** mniej więcej 20 sekund po zmianie (i przy zamknięciu programu) projekt, który ma plik, jest zapisywany do tego pliku; rysunek bez pliku trafia do pliku odzyskiwania we wspólnym folderze. Następne uruchomienie proponuje taką pracę (*Znaleziono projekt, który nie został zapisany*: **Odzyskaj** albo **Odrzuć go**); zapisanie rysunku pod nazwą usuwa jego plik odzyskiwania.
* **Pomoc → Pobierz kopię zapasową…** zapisuje plik zip z ustawieniami PenCreatora (lista urządzeń, Twoje pisaki, język, schemat kolorów, opcje widoku, ostatnie pliki) i całą wspólną galerią clipartów, z README. **Pomoc → Przywróć z kopii zapasowej…** wczytuje taki plik po potwierdzeniu: ustawienia są zastępowane, a obrazki **dodawane** do galerii, więc nic, co w międzyczasie zapisał inny program, nie ginie. Kopia zrobiona w innym programie jest odrzucana. LightCreator i MillCreator mają te same dwie pozycje.
* Projekty (`.cre`), G-code oraz wyeksportowane SVG i DXF zapisują się tam, gdzie wskażesz. Zdjęcia i filmy kamery trafiają obok projektu (zobacz [Sterowanie maszyną](Sterowanie-maszyna#kamera-rtsp-zdjęcia-wideo-i-poklatkowe)).
* Menu **Ustawienia** ma **Opcje widoku…**, **Opcje programu…** (**Szukaj nowszej wersji przy uruchamianiu programu**), **Wtyczki…**, **Język** (30) i **Schemat kolorów** (14).

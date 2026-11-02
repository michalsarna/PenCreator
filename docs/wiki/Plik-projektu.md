🌐 [English](Project-File) · **Polski**

# Plik projektu

PenCreator zapisuje **`.cre`**, plik projektu baseCreatora, który zapisują też LightCreator i MillCreator: jeden plik JSON z rysunkiem, opisem i **jedną sekcją na program** (`laser`, `mill`, `pen`). Każdy program zapisuje swoją sekcję i zostawia nietknięte sekcje pozostałych, więc projekt przechodzi między programami bez strat.

Sekcja `pen` zawiera 64 warstwy (nazwa, kolor, pokazana i rysowana, sposób rysowania, pisak z biblioteki, szerokość pisaka, prędkość, odstęp i kąt linii wypełnienia, linie krzyżowe, ile razy rysować, sposób rysowania obrazu i rastrowanie, miejsce w kolejności zadania), **Twoje własne pisaki, których używają warstwy** (dzięki temu projekt otwarty na innym komputerze przynosi je ze sobą; dołączają tam do listy **Moje pisaki**) oraz notatkę o ploterze, dla którego zrobiono projekt (nazwa i rozmiar arkusza); gdy otworzysz projekt dla urządzenia o innym arkuszu, pasek stanu o tym mówi. Warstwa z liczbą poza zakresem jest zastępowana warstwą domyślną.

Opis (karta **Opis**) jest częścią projektu; PenCreator zapisuje go też do G-code jako wiersze `(Note: …)`. Projekt, który ma plik, jest do niego zapisywany sam mniej więcej 20 sekund po zmianie (zobacz [Ustawienia i pliki](Ustawienia-i-pliki)).

Projekt zrobiony w LightCreatorze lub MillCreatorze otwiera się tu z rysunkiem, opisem i domyślnymi pisakami; przy zapisie ich sekcje wracają do pliku bez zmian. Plik z nowszym formatem kontenera (lub nowszą sekcją `pen`), niż zna program, jest odrzucany z komunikatem.

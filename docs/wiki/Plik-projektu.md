🌐 [English](Project-File) · **Polski**

# Plik projektu

PenCreator zapisuje **`.cre`**, plik projektu baseCreatora, który zapisują też LightCreator i MillCreator: jeden plik JSON z rysunkiem, opisem i **jedną sekcją na program** (`laser`, `mill`, `pen`). Każdy program zapisuje swoją sekcję i zostawia nietknięte sekcje pozostałych, więc projekt przechodzi między programami bez strat.

Sekcja `pen` zawiera 16 warstw (kolor, grubość pisaka, prędkość, kontur / wypełnienie, odstęp i kąt linii wypełnienia, linie krzyżowe, ile razy rysować) oraz notatkę o ploterze, dla którego zrobiono projekt (nazwa i rozmiar arkusza); gdy otworzysz projekt dla urządzenia o innym arkuszu, pasek stanu o tym mówi. Projekt zrobiony w LightCreatorze lub MillCreatorze otwiera się tu z rysunkiem, opisem i domyślnymi pisakami; przy zapisie ich sekcje wracają do pliku bez zmian. Plik z nowszym formatem kontenera (lub nowszą sekcją `pen`), niż zna program, jest odrzucany z komunikatem.

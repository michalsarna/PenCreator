🌐 [English](Project-File) · **Polski**

# Plik projektu

PenCreator zapisuje **`.cre`**, plik projektu baseCreatora, który zapisują też LightCreator i MillCreator: jeden plik JSON z rysunkiem, opisem i **jedną sekcją na program** (`laser`, `mill`, później `pen`). Każdy program zapisuje swoją sekcję i zostawia nietknięte sekcje pozostałych, więc projekt przechodzi między programami bez strat.

PenCreator nie ma jeszcze własnej sekcji. Projekt zrobiony w LightCreatorze lub MillCreatorze otwiera się tu z rysunkiem i opisem; przy zapisie ich sekcje wracają do pliku bez zmian. Plik z nowszym formatem kontenera, niż zna program, jest odrzucany z komunikatem.

🌐 [English](Safety-and-Limitations) · **Polski**

# Bezpieczeństwo i ograniczenia

PenCreator tworzy ścieżki plotowania i zapisuje **G-code dla GRBL**. Może **połączyć się z ploterem GRBL i przesłać zadanie** (karta Urządzenie: **Połącz**; karta Sterowanie: **Start**, **STOP**, **Obrys**, **Od wiersza…**), a plik można też zapisać i wysłać własnym programem. G-code sprawdzają automatyczne testy i symulator GRBL (pisak jest podniesiony przed każdym ruchem z podniesionym pisakiem, zmiana pisaka zatrzymuje `M0`), a PenCreator był testowany na ploterze zbudowanym na wzór [Drawing Robota](https://www.printables.com/model/137296-drawing-robot-arduino-uno-cnc-shield-grbl) (Arduino UNO, CNC Shield, GRBL). Twoja maszyna może się różnić. **Zmiana pisaka w stałym miejscu nie była jeszcze sprawdzona na prawdziwym ploterze.**

* **Pisak podniesiony** i **Pisak opuszczony** w urządzeniu ustawiaj ostrożnie: przy osi Z to wysokości w mm (opuszczony 0 to papier), przy serwie to wartości `S`; przy **Własnym G-code** to wpisane przez Ciebie wiersze (np. `M3 S1000` i `M5` dla elektromagnesu albo `M280 P0 S…` dla serwa Marlina), wysyłane bez zmian. Złe wartości mogą wcisnąć pisak w maszynę lub stół. Najpierw wypróbuj **Podnieś pisak** i **Opuść pisak** na karcie Sterowanie, uruchom zadanie z podniesionym pisakiem albo bez pisaka i bądź przy maszynie.
* Pisak zaczyna w zerze maszyny urządzenia (wybranym narożniku); pierwszy ruch idzie stamtąd, a na końcu pisak wraca do tego narożnika.
* Przesuw, przyciski pisaka, **Obrys** i pauzy na zmianę pisaka wysyłają prawdziwe ruchy. **Obrys** podnosi pisak i obchodzi prostokąt rysunku: sprawdź, czy podniesiony pisak nie zahaczy o papier i zaciski.
* Obraz (bitmapa) jest rysowany tylko na warstwie, której **Rysowanie** to **Obraz**; na każdej innej jest pomijany z ostrzeżeniem (najpierw zwektoryzuj go na linie albo przełącz warstwę na **Obraz**). Tekst jest rysowany konturami czcionki (każda litera dookoła krawędzi), nie literami jednoliniowymi.
* Linie wypełnienia to równoległe linie pod kątem; bardzo mały odstęp daje ogromną liczbę linii, a po 200 000 linii wypełnienia reszta wypełnienia jest pomijana z ostrzeżeniem.
* Nie ma jeszcze HPGL ani czcionek jednoliniowych.
* Co jest testowane: edytor i plik projektu (sekcje innych programów są zachowywane), otwieranie plików, urządzenia, warstwy, kolejność pociągnięć, wypełnienie, G-code, menu i ich klawisze oraz tłumaczenia, automatycznymi testami przy każdej zmianie; G-code zadań także na symulowanym kontrolerze GRBL. Ekrany są też sprawdzane w rendererze bez okna przy każdej zmianie.

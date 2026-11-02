🌐 [English](Safety-and-Limitations) · **Polski**

# Bezpieczeństwo i ograniczenia

PenCreator tworzy ścieżki plotowania i zapisuje **G-code dla GRBL**. Może **połączyć się z ploterem GRBL i przesłać zadanie** (karta Urządzenie: **Połącz**; karta Sterowanie: **Start**, **STOP**, **Obrys**, **Od wiersza…**), **ale nie było to sprawdzone na prawdziwym ploterze**; plik można też zapisać i wysłać własnym programem. G-code sprawdzają automatyczne testy i symulator GRBL (pisak jest podniesiony przed każdym ruchem z podniesionym pisakiem, zmiana pisaka zatrzymuje `M0`), ale **nie był wypróbowany na prawdziwym ploterze**.

* **Pisak podniesiony** i **Pisak opuszczony** w urządzeniu ustawiaj ostrożnie: przy osi Z to wysokości w mm (opuszczony 0 to papier), przy serwie to wartości `S`. Złe wartości mogą wcisnąć pisak w maszynę lub stół. Najpierw wypróbuj **Podnieś pisak** i **Opuść pisak** na karcie Sterowanie, uruchom zadanie z podniesionym pisakiem albo bez pisaka i bądź przy maszynie.
* Pisak zaczyna w zerze maszyny urządzenia (wybranym narożniku); pierwszy ruch idzie stamtąd, a na końcu pisak wraca do tego narożnika.
* Przesuw, przyciski pisaka, **Obrys** i pauzy na zmianę pisaka wysyłają prawdziwe ruchy. **Obrys** podnosi pisak i obchodzi prostokąt rysunku: sprawdź, czy podniesiony pisak nie zahaczy o papier i zaciski.
* Obraz (bitmapa) jest rysowany tylko na warstwie, której **Rysowanie** to **Obraz**; na każdej innej jest pomijany z ostrzeżeniem (najpierw zwektoryzuj go na linie albo przełącz warstwę na **Obraz**). Tekst jest rysowany konturami czcionki (każda litera dookoła krawędzi), nie literami jednoliniowymi.
* Linie wypełnienia to równoległe linie pod kątem; bardzo mały odstęp daje ogromną liczbę linii, a po 200 000 linii wypełnienia reszta wypełnienia jest pomijana z ostrzeżeniem.
* Nie ma jeszcze HPGL ani czcionek jednoliniowych.
* W macOS część klawiszy menu (Cmd+E, Cmd+Shift+P, Cmd+L i inne) jeszcze nie uruchamia swoich poleceń; użyj menu. Zobacz [Skróty klawiszowe](Skroty-klawiszowe#w-macos).
* Co jest testowane: edytor i plik projektu (sekcje innych programów są zachowywane), otwieranie plików, urządzenia, warstwy, kolejność pociągnięć, wypełnienie, G-code, menu i ich klawisze oraz tłumaczenia, automatycznymi testami przy każdej zmianie; G-code zadań także na symulowanym kontrolerze GRBL. Ekrany sprawdzono w rendererze bez okna, nie na prawdziwej maszynie.

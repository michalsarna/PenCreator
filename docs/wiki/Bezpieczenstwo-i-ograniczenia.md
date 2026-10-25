🌐 [English](Safety-and-Limitations) · **Polski**

# Bezpieczeństwo i ograniczenia

PenCreator tworzy ścieżki plotowania i zapisuje **G-code dla GRBL**. Może **połączyć się z ploterem GRBL i przesłać zadanie** (karta Device: Connect; karta Control: Start, Stop, Frame), **ale nie było to sprawdzone na prawdziwym ploterze**; plik można też zapisać i wysłać własnym programem. G-code sprawdzają automatyczne testy (pisak jest podniesiony przed każdym ruchem z podniesionym pisakiem, zmiana pisaka zatrzymuje `M0`), ale **nie był wypróbowany na prawdziwym ploterze**.

* **Pisak podniesiony** i **Pisak opuszczony** w urządzeniu ustawiaj ostrożnie: przy osi Z to wysokości w mm (opuszczony 0 to papier), przy serwie to wartości `S`. Złe wartości mogą wcisnąć pisak w maszynę lub stół. Nowe zadanie najpierw sprawdź z podniesionym pisakiem i bądź przy maszynie.
* Pisak zaczyna w zerze maszyny urządzenia (wybranym narożniku); pierwszy ruch idzie stamtąd.
* Obrazów (bitmap) na warstwach nie da się narysować; najpierw zamień je na linie. Tekst jest rysowany konturami czcionki (podwójne linie), nie literami jednoliniowymi.
* Linie wypełnienia to równoległe linie pod kątem; bardzo mały odstęp daje ogromną liczbę linii i część wypełnienia jest pomijana z ostrzeżeniem.
* Nie ma przesuwania ręcznego ani HPGL. Frame podnosi pisak i obchodzi obrys rysunku: sprawdź, czy wysokość podniesionego pisaka jest wolna.
* Co jest testowane: edytor i plik projektu (sekcje innych programów są zachowywane), otwieranie plików, urządzenia, warstwy, kolejność pociągnięć, wypełnienie, G-code i tłumaczenia, automatycznymi testami przy każdej zmianie. Ekrany sprawdzono w rendererze bez okna, nie na prawdziwej maszynie.

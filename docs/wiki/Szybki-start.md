🌐 [English](Quick-Start) · **Polski**

# Szybki start

1. **Wybierz urządzenie.** Ekran startowy pokazuje Twoje plotery. **Dodaj urządzenie…** tworzy nowe (rozmiar arkusza, sposób podnoszenia pisaka, kamery); **Nie mam urządzenia** tworzy zwykłe urządzenie Dummy (arkusz A4, **Poziomo** albo **Pionowo** (Landscape / Portrait): wybierasz pod listą urządzeń, a później na karcie z urządzeniem). Jeśli klikniesz **Nowy** albo **Otwórz…** bez wyboru, użyte będzie Dummy.
2. **Nowy** zaczyna pusty projekt na arkuszu urządzenia. **Otwórz…** wczytuje plik SVG, PDF, Illustratora lub `.cre`; **Plik → Import → Importuj projekt LightBurn…** wczytuje projekt LightBurna, a **Import UGS design…** plik UGS Designera (kształty są dodawane do rysunku; kieszenie stają się wypełnieniami). Upuszczenie pliku na okno otwiera go albo importuje, gdy to plik LightBurna lub UGS.
3. **Rysuj** narzędziami po lewej (zaznaczanie, edycja węzłów, prostokąt, elipsa, trójkąt, gwiazda, wielokąt, serce, spirala, linia, pióro, tekst, przesuwanie, zoom) albo wstaw obrazek z karty **Clipart**.
4. **Pisaki.** W karcie **Warstwy** każda warstwa to pisak: ustaw kolor, grubość i prędkość oraz czy kształty są rysowane konturem, wypełniane liniami, czy jedno i drugie. Zaznacz kształty i kliknij **Umieść zaznaczone kształty na tej warstwie**.
5. **Ploter → Wykres zadania…** (Ctrl+Shift+P) pokazuje, co zrobi pisak; linie przerywane to ruchy z podniesionym pisakiem. **Eksportuj G-code…** (Ctrl+Shift+G) zapisuje plik dla GRBL.
6. **Zapisz** jako `.cre` ([Plik projektu](Plik-projektu)). Karta **Opis** zawiera notatkę o projekcie.

> Program zapisuje G-code i może przesłać go do podłączonego plotera GRBL (karta Device: Connect; karta Control: Start, Stop, Frame, From line…), ale nie było to sprawdzone na prawdziwym ploterze. Zobacz [Bezpieczeństwo i ograniczenia](Bezpieczenstwo-i-ograniczenia).

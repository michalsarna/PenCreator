🌐 [English](Quick-Start) · **Polski**

# Szybki start

Pierwsze zadanie od początku do końca: wybierz ploter, narysuj lub zaimportuj coś, daj kształtom pisaki, obejrzyj wykres i G-code, a potem zapisz G-code (albo wyślij go do maszyny).

> PenCreator jest na wczesnym etapie: nie był jeszcze sprawdzony na prawdziwym ploterze. G-code sprawdzają automatyczne testy i symulator GRBL. Zanim uruchomisz zadanie, przeczytaj [Bezpieczeństwo i ograniczenia](Bezpieczenstwo-i-ograniczenia).

## 1. Wybierz urządzenie

![Ekran startowy](https://raw.githubusercontent.com/michalsarna/PenCreator/main/docs/screenshots/01-start.png)

Ekran startowy pokazuje Twoje plotery pod nagłówkiem **Urządzenie** (każdy z rozmiarem obszaru roboczego). Kliknij jeden, żeby go wybrać.

* **Nie mam urządzenia** dodaje i wybiera zwykłe urządzenie o nazwie **Dummy**: arkusz A4, pisak podnoszony osią Z. Pod listą **Papier** A4 **Poziomo** / **Pionowo** obraca jego arkusz (później to samo jest na karcie **Urządzenie** w edytorze). Gdy nie ma żadnego urządzenia, **Nowy** i **Otwórz…** używają Dummy.
* **Dodaj urządzenie…** otwiera okno **Dodaj urządzenie**. Strony ważne dla pierwszego zadania:
  * **Ogólne**: **Nazwa**; **Start od** (**Zwykły rozmiar arkusza…**, punkt wyjścia dla obszaru roboczego i prędkości); **Jednostki**; **Zero maszyny (0,0)**, narożnik obszaru roboczego, w którym startuje pisak (**Przód po lewej**, **Przód po prawej**, **Tył po lewej**, **Tył po prawej**).
  * **Maszyna**: **Obszar roboczy** (X i Y, arkusz, do którego sięga pisak) i **Prędkość ruchów z podniesionym pisakiem**.
  * **Pisak**: **Pisak podnosi się za pomocą** **Oś Z** (wtedy **Pisak podniesiony** i **Pisak opuszczony** to wysokości w mm; opuszczony 0 to papier) albo **Serwo (M3 S…)** na wyjściu wrzeciona (wtedy to wartości `S`, 0 … 1000), oraz **Czekanie po ruchu pisaka** (ms). Nowe urządzenie zaczyna od 3 mm w górze, 0 w dole i 150 ms.
  * **Połączenie**: potrzebne tylko do wysyłania zadań z programu (zobacz krok 8).

  Przycisk **Dodaj urządzenie** je zapisuje. **Edytuj…**, **Powiel** i **Importuj…** (plik urządzenia) są obok **Dodaj urządzenie…**.

![Dodawanie urządzenia](https://raw.githubusercontent.com/michalsarna/PenCreator/main/docs/screenshots/02-add-device.png)

![Strona pisaka w urządzeniu](https://raw.githubusercontent.com/michalsarna/PenCreator/main/docs/screenshots/03-device-pen.png)

Jeśli program został zamknięty z niezapisaną pracą, ekran startowy najpierw mówi *Znaleziono projekt, który nie został zapisany*: **Odzyskaj** go otwiera, **Odrzuć go** go porzuca.

## 2. Zacznij rysunek

Pod nagłówkiem **Start**:

* **Nowy** zaczyna pusty rysunek na arkuszu urządzenia.
* **Otwórz…** otwiera projekt (`.cre`), plik SVG, PDF albo Illustratora (`.ai`); plik DXF albo projekt UGS (`.ugsd`) jest importowany do nowego rysunku.
* **Pokaż plik G-code…** pokazuje ścieżki pliku G-code, bez rysunku.
* Pod **Otwórz ostatnie** jest pięć ostatnich plików.

Edytor ma pasek menu (zobacz [Menu](Menu)), pod nim pasek przycisków, narzędzia po lewej, pasek rozmieszczania i panel boczny po prawej (zakładki **Właściwości**, **Warstwy**, **Urządzenie**, **Sterowanie**, **Konsola**, **Clipart**, **Opis** oraz **Kamera**, gdy urządzenie ma kamerę), rząd warstw pod obszarem roboczym i pasek stanu na dole (komunikat, powiększenie i wskaźnik, urządzenie projektu, połączenie).

Każdy rysunek to karta nad obszarem roboczym; **Nowy** (Ctrl+N) albo **+** na końcu kart otwiera następny (najwyżej 10). Plik upuszczony na okno jest otwierany: obraz przechodzi przez okno importu, plik LightBurna, UGS lub DXF jest importowany do rysunku, plik G-code jest pokazywany w analizie.

## 3. Rysuj albo importuj

* **Rysuj** narzędziami po lewej: **Zaznaczanie (V)**, **Edycja węzłów (N)**, **Prostokąt (R)**, **Elipsa (E)**, **Trójkąt (Y)**, **Gwiazda (S)**, **Wielokąt (G)**, **Serce (K)**, **Spirala (I)**, **Linia (L)**, **Linia łamana / pióro (P)**, **Tekst (T)**, **Przesuwanie widoku (H)**, **Powiększ (Z)** i **Pomniejsz (X)**. Menu **Wstawianie** ma te same narzędzia rysowania. Karta **Właściwości** zmienia rozmiar, położenie i szczegóły zaznaczonego kształtu.
* **Importuj** przez **Plik → Importuj**: **Import obrazu…**, **Importuj DXF…**, **Importuj projekt LightBurn…**, **Importuj projekt UGS…** albo importery i generatory wtyczek. Kształty są dodawane do rysunku, zaznaczone. Projekt UGS ustawia też puste warstwy dla swoich sposobów cięcia (kieszeń lub wypełnienie dostaje rysowanie **Wypełnienie liniami**, pozostałe **Kontur**).
* **Clipart**: karta **Clipart** ma galerię obrazków do wstawienia, a **Wstawianie → Online clipart…** szuka w internecie.

Tekst jest rysowany konturami czcionki (każda litera dookoła krawędzi), nie pojedynczymi liniami. Edycja, rozmieszczanie i obrazy są opisane na stronie [Narzędzia projektowe](Narzedzia-projektowe).

## 4. Daj kształtom pisaki

Nowe kształty trafiają na **aktywną warstwę**. Warstwa to pisak:

1. Zaznacz kształty i kliknij kolor w rzędzie warstw pod obszarem roboczym: przechodzą na tę warstwę, a ona staje się aktywna. (Albo na karcie **Warstwy** wybierz warstwę i naciśnij **Umieść zaznaczone kształty na tej warstwie**.)
2. Na karcie **Warstwy** ustaw warstwę: **Rysowanie** (**Kontur**, **Wypełnienie liniami**, **Wypełnienie i kontur** albo **Obraz**), **Pisak** (pisak z **Ploter → Biblioteka pisaków…**), **Szerokość pisaka**, **Prędkość**, linie wypełnienia i **Ile razy rysować**.
3. Lista warstw to kolejność zadania (▲ / ▼ przesuwają warstwę). Warstwa bez zaznaczenia nie jest rysowana.

Gdy następna warstwa wymaga innego pisaka, zadanie zatrzymuje się na zmianę. Wszystko o warstwach, pisakach i zmianach pisaka jest na stronie [Warstwy i pisaki](Warstwy-i-pisaki).

![Karta Warstwy](https://raw.githubusercontent.com/michalsarna/PenCreator/main/docs/screenshots/05-layers.png)

## 5. Obejrzyj wykres

**Ploter → Wykres zadania…** (Ctrl+Shift+P) albo przycisk **Podgląd…** na pasku pod menu otwiera obok rysunku kartę **Podgląd: nazwa**:

* arkusz z liniami, które narysują pisaki, w ich tuszu i grubości, oraz ruchy z podniesionym pisakiem jako przerywane niebieskie linie (**Ruchy z podniesionym pisakiem** je wyłącza);
* animacja (**Odtwarzaj** / **Pauza**, **Z powrotem na początek**, **Powtarzaj**, **Szybkość animacji**) z suwakiem postępu; czerwone kółko to pisak;
* czas (**Czas** dotąd / razem), długość linii i ruchów z podniesionym pisakiem, liczba podniesień pisaka i minuty;
* ostrzeżenia na pomarańczowo: obraz, którego nie da się narysować, rysunek częściowo poza obszarem roboczym, wypełnienie ze zbyt wieloma liniami;
* **Resetuj widok** i dwa przyciski dopasowujące obszar roboczy albo wszystkie linie; kółko przybliża, przeciąganie przesuwa, dwuklik wraca do pierwszego widoku.

Podgląd nadąża za zmianami rysunku. Przycisk **Ścieżki** (zielony, gdy włączony) rysuje te same drogi na obszarze roboczym: linie w kolorach ich warstw i ruchy z podniesionym pisakiem na jasnoniebiesko.

![Wykres zadania](https://raw.githubusercontent.com/michalsarna/PenCreator/main/docs/screenshots/09-plot.png)

## 6. Obejrzyj G-code

**Podgląd G-code** (obok **Zapisz G-code** na pasku) robi G-code projektu i pokazuje go jako tekst w karcie **Podgląd G-code: nazwa.nc**; nic nie jest zapisywane na dysk. Słowa są pokolorowane (słowa G i M, osie, prędkości, komentarze), a tekst można edytować.

* **Zapisz G-code** w karcie zapisuje tekst, z Twoimi zmianami, do pliku.
* **Wygeneruj ponownie** robi go od nowa z projektu; gdy był edytowany, przycisk najpierw pyta **Stracić zmiany?**, a drugie kliknięcie go robi.
* **Analizuj** pokazuje ścieżki tekstu w karcie **Analiza G-code**.
* *Zmieniony ręcznie* i kropka na karcie oznaczają zmiany; zamknięcie takiej karty pyta, czy je zapisać.
* Dopóki nie ma zmian, tekst jest robiony od nowa sam, chwilę po zapisaniu projektu, zmianie notatek (karta **Opis**), zmianie ustawienia urządzenia albo włączeniu lub wyłączeniu postprocesora. Ręczne zmiany nigdy nie są przy tym tracone.

Karty można postawić obok siebie: **Widok → Podziel widok: obok siebie** albo przeciągnij kartę do innego panelu (zobacz [Narzędzia projektowe](Narzedzia-projektowe#karty-i-panele)).

## 7. Zapisz projekt

**Plik → Zapisz** (Ctrl+S) zapisuje projekt jako `.cre` (za pierwszym razem pyta o nazwę). Od tej chwili zmiana jest też zapisywana do tego pliku sama, po mniej więcej 20 sekundach. Projekt zawiera rysunek, warstwy, notatki karty **Opis** i notatkę o urządzeniu; zobacz [Plik projektu](Plik-projektu). Notatki trafiają też na początek G-code jako wiersze `(Note: …)`.

## 8. Zapisz G-code albo narysuj

**Zapisz G-code do pliku:** przycisk **Zapisz G-code** albo **Ploter → Eksportuj G-code…** (Ctrl+E). Plik ma nazwę projektu (`nazwa.nc`; `untitled.nc`, zanim projekt zostanie zapisany). Pasek stanu mówi, gdzie go zapisano, i powtarza ostrzeżenia. Włączone postprocesory wtyczek zmieniają plik (zobacz [Wtyczki](Wtyczki)). Wyślij plik własnym programem albo:

**Wyślij go z PenCreatora** do plotera z GRBL:

1. **Maszyna → Ustawienia urządzenia…**, strona **Połączenie**: **Typ połączenia** (Serial port albo Network (TCP, e.g. ser2net) – te dwie nazwy nie mają tłumaczenia), **Port** albo **Host** oraz **Wersja GRBL** płyty. **Zapisz**.
2. Na karcie **Urządzenie** naciśnij **Połącz**. Pasek stanu pokazuje stan maszyny.
3. Na karcie **Sterowanie**: **Podnieś pisak** i **Opuść pisak** sprawdzają pozycje góry i dołu; przyciski ruchu przesuwają pisak; **Obrys** (Frame) obchodzi prostokąt rysunku z podniesionym pisakiem.
4. **Start** wysyła zadanie. Przy zmianie pisaka zadanie się zatrzymuje, a okno mówi, który pisak założyć: **Kontynuuj zadanie** idzie dalej. **STOP** zatrzymuje zadanie.

Sterowanie maszyną jest opisane na stronie [Sterowanie maszyną](Sterowanie-maszyna).

🌐 [English](Design-Tools) · **Polski**

# Narzędzia projektowe

PenCreator edytuje kształty tymi samymi narzędziami co LightCreator i MillCreator (baseCreator). Klawisze narzędzi są na stronie [Skróty klawiszowe](Skroty-klawiszowe), menu na stronie [Menu](Menu).

## Edycja

* **Prawy przycisk myszy** na kształcie otwiera jego menu: **Kopiuj**, **Wklej**, **Powiel**, **Usuń**, **Zaznacz wszystko**, **Grupuj**, **Rozgrupuj**, **Zablokuj**, **Odblokuj**, odbicia i obroty o ćwierć, podmenu **Rozmieść** (kolejność, wyrównanie, wyśrodkowanie, **Edytuj węzły**, **Zamień na ścieżkę**) i podmenu **Operacje na kształtach** (**Suma**, **Część wspólna**, **Odejmij**, **Różnica symetryczna**, **Przesuń obrys…**, **Zaokrąglij rogi…**, **Zamień na krzywe**; są też w menu **Rozmieść**); dla obrazu także **Dostosuj obraz…** i **Wektoryzacja obrazu…**. Pozycje, które nie pasują do zaznaczenia, są szare.
* **Dwa razy Esc** (drugi Esc w ciągu pół sekundy) wraca do narzędzia **Zaznaczanie (V)**.
* **Rząd warstw** pod obszarem roboczym: po jednym kolorze na każdą z 64 warstw (kropka oznacza warstwy z kształtami); kliknięcie przenosi zaznaczone kształty na tę warstwę, dwuklik otwiera jej ustawienia. Zobacz [Warstwy i pisaki](Warstwy-i-pisaki).

## Import

* **DXF:** **Plik → Importuj → Importuj DXF…** (albo **Plik → Otwórz…**, albo przeciągnięcie pliku) wczytuje ASCII `.dxf` (R12 i nowsze): linie, polilinie z łukami, łuki, okręgi, elipsy, splajny i bloki jako kontury, w milimetrach (jednostka pliku jest odczytywana), łuki i okręgi jako krzywe; linie, które się stykają, stają się jedną ścieżką. Warstwy DXF trafiają na warstwy projektu, na których nie ma jeszcze kształtów, jedna po drugiej. To, co nie jest linią, jest pomijane i wymienione w pasku stanu. **Rozmieść → Połącz ścieżki automatycznie…** / **Zamknij ścieżkę z tolerancją…** naprawiają ścieżki, które nie są całkiem zamknięte, a **Plik → Eksport → Eksportuj zaznaczenie jako SVG…** / **Eksportuj zaznaczenie jako DXF…** zapisują kształty z powrotem.
* **LightBurn i UGS Designer:** **Plik → Importuj → Importuj projekt LightBurn…** (`.lbrn2`, `.lbrn`) i **Importuj projekt UGS…** (`.ugsd`) dodają kształty do rysunku. Sposoby cięcia projektu UGS ustawiają warstwy, na których nie ma jeszcze kształtów (kieszeń lub wypełnienie dostaje rysowanie **Wypełnienie liniami**, pozostałe **Kontur**).
* **Obrazy (bitmapy):** **Plik → Importuj → Import obrazu…** (albo upuszczenie pliku PNG, JPG, BMP, GIF lub WebP na okno) otwiera okno importu: obrót, odbicie, jasność, kontrast, gamma, automatyczne poziomy, negatyw i rozmiar. **Dostosuj obraz…** (w sekcji *Obraz* karty **Właściwości** albo **Rozmieść → Dostosuj obraz…**) zmienia obraz na stronie, a **Wektoryzacja obrazu…** (ta sama sekcja albo **Rozmieść → Wektoryzacja obrazu…**) zamienia jego ciemne (lub jasne) obszary na kontury (próg, filtr plamek, uproszczenie, wygładzanie krzywych), które są rysowane jak każdy inny kształt.
* **Obrazy rysowane liniami:** obraz położony na warstwie raz przełącza ją na rysowanie **Obraz** (można to zmienić z powrotem), a wtedy sam obraz jest rysowany liniami, według ustawienia **Sposób rysowania obrazu** warstwy: **Rastrowane wiersze**, **Kropki** albo **Kreskowanie według tonu**. Na warstwie z innym sposobem rysowania obraz jest tylko pokazywany, nie rysowany. Zobacz [Warstwy i pisaki](Warstwy-i-pisaki#sposoby-rysowania).

## Rozmieszczanie, kopiowanie i generowanie

Menu **Wstawianie** zawiera narzędzia lewej palety, które wstawiają obiekty (**Prostokąt (R)**, **Elipsa (E)**, **Trójkąt (Y)**, **Gwiazda (S)**, **Wielokąt (G)**, **Serce (K)**, **Spirala (I)**, **Linia (L)**, **Linia łamana / pióro (P)** i **Tekst (T)**; używane narzędzie ma ptaszek), a potem **Generuj kształt…** (generatory wtyczek) i **Online clipart…** (ta pozycja nie ma tłumaczenia).

Polecenia menu **Rozmieść** (takie same w LightCreatorze, MillCreatorze i PenCreatorze):

* **Połącz ścieżki automatycznie…** łączy otwarte ścieżki, których końce są bliżej niż tolerancja; **Zamknij ścieżkę z tolerancją…** zamyka te, których oba końce są tak blisko. **Narożniki dogbone / T-bone…** dodaje wcięcie w każdym ostrym narożniku, żeby okrągły frez w niego sięgnął (to polecenie jest dla frezowania).
* **Tablica siatkowa…** kopiuje zaznaczenie w rzędach i kolumnach. **Tablica kołowa…** układa kopie wokół środka zaznaczenia na *Kąt całkowity* (360° to pełny okrąg), obrócone razem z miejscem albo tylko przesunięte. Powstaje najwyżej 2000 kopii.
* **Kopiuj wzdłuż ścieżki…**: zaznacz kształty, a **na końcu** ścieżkę; kopie stoją co *Odległość* wzdłuż niej i mogą się z nią obracać. **Tekst na ścieżce…**: zaznacz tekst, a na końcu ścieżkę; kontury tekstu są wyginane wzdłuż ścieżki od *Start* (potem to kontury, nie tekst).
* **Przytnij kształtem…**: zaznacz linie, a **na końcu** kształt zamknięty; zostają części wewnątrz (albo na zewnątrz), przecięte tam, gdzie linie przecinają jego kontur. **Usuń duplikaty** usuwa kształty o tym samym konturze na tej samej warstwie (zaznaczone albo cały rysunek). **Rozdziel** robi z każdego konturu osobny kształt. **Uprość…** usuwa punkty leżące w granicach *Tolerancja* od linii.
* **Tekst seryjny…**: zaznacz tekst; zostaje skopiowany w siatce z odliczanym `{n}` albo, z **Wczytaj plik CSV…**, raz na każdy wiersz, a `{1}`, `{2}`… to kolumny. **Kod kreskowy / QR…** kładzie na aktywnej warstwie kod QR albo kod kreskowy Code 39 jako kontury (zrób je dość duże dla pisaka).
* **Rozmieść kształty (nesting)…** układa zaznaczone kształty w rzędach od rogu strony na podanym obszarze, z odstępem (według prostokątów kształtów, nie ich konturów; można pozwolić na obrót o ćwierć). **Pomiar…** pokazuje rozmiar, położenie, pole i długość zaznaczenia, a dla dwóch kształtów odległości między nimi.

**Plik → Eksport → Eksportuj zaznaczenie jako SVG…** / **Eksportuj zaznaczenie jako DXF…** zapisuje zaznaczone kształty (wszystkie, gdy nic nie jest zaznaczone).

## Obraz w tle (menu Widok) i okno wektoryzacji

**Widok → Obraz w tle…** kładzie zdjęcie, plan albo skan **pod rysunkiem**: **Wczytaj obraz z pliku…** (PNG, JPG, BMP, WebP, GIF). Zaznacz **Przesuwaj narożniki** i przeciągnij cztery narożniki obrazu na narożniki papieru albo obszaru roboczego (obraz podąża za nimi z przekształceniem perspektywicznym); **Krycie**, **Pokaż**, **Dopasuj do obszaru roboczego** i **Usuń** są w tym samym oknie. Obraz nie jest zapisywany w projekcie. W oknie **Wektoryzacja obrazu** obraz można przybliżać kółkiem myszy albo przyciskami **Powiększ**, **Pomniejsz** i **Resetuj widok**, a zielony przycisk **Wektoryzuj** tworzy kontury.

## Panel boczny

* Przeciągnij zakładkę, żeby ustawić ją w innym miejscu (kolejność jest zapamiętywana, także w kopii zapasowej). Mała strzałka po lewej stronie zakładek chowa panel; zostaje wąski pasek z nazwami zakładek, a kliknięcie nazwy otwiera panel na tej zakładce.
* Zakładki: **Właściwości**, **Warstwy**, **Urządzenie**, **Sterowanie**, **Konsola**, **Clipart**, **Opis**, **Kamera** (gdy urządzenie ma włączoną kamerę) i zakładka każdej wtyczki, która ma ustawienia.

## Karty i panele

Każdy otwarty rysunek, **Podgląd**, G-code projektu jako tekst (**Podgląd G-code: nazwa.nc**), **Analiza G-code** pliku i **Widok z kamery** to **karty** nad obszarem roboczym.

* **Widok → Podziel widok: obok siebie** albo **Podziel widok: jeden nad drugim** dzieli obszar roboczy na najwyżej **trzy panele**; **Jeden widok** zbiera wszystkie karty z powrotem razem. Każdy panel ma własny pasek kart, z **+** dla nowego rysunku.
* **Przeciągnij kartę** do innego panelu (albo w obrębie paska w inne miejsce). Albo użyj jej menu (prawy przycisk): **Przenieś na panel N** (inne panele i nowy, gdy jest miejsce, a w panelu zostaje inna karta), **Otwórz w osobnym oknie** (dla podglądu, tekstu G-code i widoku z kamery; okno pamięta swoje miejsce, a **Wróć do głównego okna** albo zamknięcie okna przywraca kartę) i **Zamknij kartę**. Kliknięcie karty środkowym przyciskiem też ją zamyka.
* Przeciągnij linię między dwoma panelami, żeby zmienić ich rozmiary; dwuklik na niej je wyrównuje.
* Zamknięcie ostatniej karty panelu zamyka panel, a pozostałe się zsuwają.
* Nic nie otwiera się dwa razy: otwarcie projektu, który już jest otwarty, pokazuje jego kartę, a projekt ma jeden podgląd, jeden widok z kamery i jeden tekst G-code.
* Jeden rysunek jest **aktywny**: na nim działają narzędzia, menu i klawisze. Rysunek w innym panelu jest pokazany do oglądania, z własnym przybliżeniem; kliknięcie robi z niego aktywny.
* Układ paneli jest zapamiętywany.

## Karta z tekstem G-code

**Podgląd G-code** (przycisk obok **Zapisz G-code**) robi program projektu (z włączonymi postprocesorami) i pokazuje go **jako tekst w karcie**, pokolorowany (słowa G i M, osie, prędkości, komentarze) i otwarty do edycji. **Zapisz G-code** w karcie zapisuje go do pliku; **Wygeneruj ponownie** robi go od nowa z projektu (przy zmianach najpierw pyta **Stracić zmiany?**); **Analizuj** pokazuje jego ścieżki w karcie **Analiza G-code**. Karta ze zmianami pokazuje *Zmieniony ręcznie* i kropkę, a przy zamykaniu pyta, czy je zapisać. Bez zmian tekst jest robiony od nowa sam po zapisaniu projektu, zmianie notatek, zmianie ustawienia urządzenia albo zmianie postprocesorów. **Ustawienia → Opcje widoku… → Podgląd G-code po zapisaniu** (domyślnie wyłączone) otwiera zapisany plik w analizie po każdym **Zapisz G-code**.

## Opcje widoku

**Ustawienia → Opcje widoku…** ma siatkę główną i pomocniczą, kolory obszaru roboczego i przestrzeni wokół niego, grubość linii oraz:

* **Pokaż nazwy warstw** (domyślnie wyłączone): rząd warstw pod obszarem roboczym pisze nazwę warstwy z kształtami obok jej koloru (długie nazwy są skracane, pełna jest w podpowiedzi; rząd przewija się w bok, gdy brakuje miejsca).
* **Podgląd G-code po zapisaniu** (domyślnie wyłączone): zobacz wyżej.
* **Pokaż nazwy zmiennych** (domyślnie wyłączone): obok etykiety każdego pola, które ma zmienną dla własnego G-code, jej nazwa jest pokazana kursywą w nawiasie, np. *(machine_name)* obok nazwy urządzenia albo *(layer_name)* obok nazwy warstwy; kliknięcie kopiuje `{machine_name}` do schowka, gotowe do wklejenia w pole własnego G-code (zobacz [Sterowanie maszyną](Sterowanie-maszyna#uruchamianie-zadania)).

## Presety warstw

**Zapisz preset…** i **Wczytaj preset** nad ustawieniami warstwy otwierają okno **Presety warstw**: ustawienia pisaka warstwy są zachowywane pod nazwą i można je później nałożyć na dowolną warstwę. Zobacz [Warstwy i pisaki](Warstwy-i-pisaki#presety-warstw).

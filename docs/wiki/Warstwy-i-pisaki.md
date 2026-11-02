🌐 [English](Layers-and-Pens) · **Polski**

# Warstwy i pisaki

Rysunek ma **64 warstwy**. Każdy kształt leży na jednej warstwie, a **warstwa to pisak**: jej kolor, szerokość linii, prędkość i sposób, w jaki są rysowane leżące na niej kształty (kontury, linie wypełnienia albo obraz). Warstwy są rysowane jedna po drugiej, a gdy następna warstwa wymaga innego pisaka, zadanie zatrzymuje się, żeby można go było wymienić.

![Karta Warstwy: pisak to warstwa](https://raw.githubusercontent.com/michalsarna/PenCreator/main/docs/screenshots/05-layers.png)

## Warstwy nowego rysunku

| Warstwa | Nazwa | Rysowanie |
|---|---|---|
| 1 | Black pen | **Kontur** |
| 2 | Red pen | **Kontur** |
| 3 | Blue pen | **Kontur** |
| 4 | Green pen | **Kontur** |
| 5 | Black fill | **Wypełnienie liniami** (czarne, jak warstwa 1) |
| 6 … 64 | Layer 6 … Layer 64 | **Kontur** |

Nazwy warstw to dane projektu, nie teksty interfejsu, więc są po angielsku także w polskim interfejsie; można je zmienić. Wszystkie warstwy zaczynają z ustawieniami: **Szerokość pisaka** 0,4 mm, **Prędkość** 3000 mm/min, **Odstęp linii wypełnienia** 0,4 mm pod kątem 45°, jednym przejściem i bez pisaka z biblioteki. Te same warstwy dostaje rysunek otwarty z SVG lub PDF oraz projekt LightCreatora lub MillCreatora.

## Karta Warstwy

Na górze karty jest **lista warstw**. Pokazuje warstwy, na których są kształty, oraz tę, nad którą pracujesz; zaznacz **Pokaż wszystkie 64 warstw** (Show all 64 layers), żeby zobaczyć wszystkie. Każdy wiersz ma:

* **▲ / ▼** (**Wcześniej w zadaniu** / **Później w zadaniu**): lista jest w kolejności zadania, od góry w dół; strzałki przesuwają warstwę o jedno miejsce. Kolejność jest zapisywana w projekcie;
* kolor warstwy;
* pole wyboru **Rysowana: wchodzi do wykresu i do G-code**. Warstwa bez zaznaczenia jest pomijana w podglądzie, ścieżkach, G-code i zadaniu, ale jej kształty zostają na obszarze roboczym;
* oko, **Pokazana w obszarze roboczym**. Ukryta warstwa jest ukryta tylko na obszarze roboczym (a **Zaznacz wszystko** pomija jej kształty); zadanie wciąż ją rysuje, jeśli jest zaznaczona jako rysowana;
* nazwę i sposób rysowania warstwy: kliknięcie robi z niej **aktywną warstwę**, której ustawienia są pokazane niżej i na którą trafiają nowe kształty;
* liczbę kształtów na warstwie.

Pod listą są **Zapisz preset…** i **Wczytaj preset** (zobacz [Presety warstw](#presety-warstw)) oraz ustawienia aktywnej warstwy:

| Ustawienie | Co robi |
|---|---|
| **Nazwa** | Nazwa warstwy. Trafia do G-code jako komentarz `(Layer: nazwa)` i jest zmienną `{layer_name}` w G-code zmiany pisaka. |
| **Kolor** | Kolor kształtów na obszarze roboczym. Jest częścią pisaka: dwie warstwy w różnych kolorach to dwa pisaki (zobacz [Zmiany pisaka](#zmiany-pisaka)). |
| **Rysowanie** | **Kontur**, **Wypełnienie liniami**, **Wypełnienie i kontur** albo **Obraz** (zobacz niżej). |
| **Pisak** | Pisak z [biblioteki pisaków](#biblioteka-pisaków) albo **Nie wybrano pisaka**. Przycisk **…** otwiera bibliotekę. Wybór pisaka ustawia **Szerokość pisaka** na większy wymiar jego śladu. |
| **Szerokość pisaka** | Szerokość linii, którą robi pisak, 0,01 … 20 mm. Bez pisaka z biblioteki to grubość linii w podglądzie. |
| **Prędkość** | Prędkość pisaka na papierze, 1 … 100000 mm/min (`F` ruchów `G1`). |
| **Sposób rysowania obrazu** | Tylko dla **Obraz**: **Rastrowane wiersze**, **Kropki** albo **Kreskowanie według tonu**. |
| **Rastrowanie** | Tylko dla **Rastrowane wiersze** i **Kropki**: jak obraz jest zamieniany na kropki: **Próg**, **Floyd-Steinberg**, **Jarvis**, **Stucki**, **Atkinson**, **Uporządkowane** albo **Skala szarości** (każde ma podpowiedź). |
| **Odstęp linii wypełnienia** | Dla wypełnienia i obrazu: odległość między liniami, 0,02 … 100 mm. |
| **Kąt linii wypełnienia** | Dla wypełnienia i obrazu: kierunek linii, −360 … 360°. |
| **Linie krzyżowe** | Dla wypełnienia: drugi zestaw linii wypełnienia w poprzek pierwszego. |
| **Ile razy rysować** | Ile razy rysowana jest każda linia, 1 … 5: dla pisaka, który jest za blady. Otwarta linia jest rysowana tam i z powrotem. |

Pod ustawieniami jest zdanie objaśniające wybrany sposób rysowania, a **Umieść zaznaczone kształty na tej warstwie** przenosi tam zaznaczone kształty (zablokowane zostają na miejscu).

## Sposoby rysowania

* **Kontur**: pisak idzie po liniach kształtów. Działają też linie otwarte.
* **Wypełnienie liniami**: kształty zamknięte są wypełniane równoległymi liniami; otwory zostają puste.
* **Wypełnienie i kontur**: najpierw wypełnienie, potem kontur na nim, więc krawędź jest czysta.
* **Obraz**: obraz (bitmapa) na warstwie jest rysowany liniami: **Rastrowane wiersze** (każdy wiersz rastrowanego obrazu jest rysowany tam, gdzie jest ciemny), **Kropki** (kropka w każdej ciemnej komórce) albo **Kreskowanie według tonu** (kreskowanie krzyżowe: im ciemniejszy obraz, tym więcej kierunków linii). Pozostałe kształty takiej warstwy są rysowane jako kontury. Obraz położony na warstwie raz przełącza ją na **Obraz**; można to zmienić z powrotem.

Na warstwie z innym sposobem rysowania obraz nie jest rysowany: podgląd i pasek stanu mówią *Obrazów na warstwach (…) nie można narysować: najpierw zamień je na linie* (**Rozmieść → Wektoryzacja obrazu…**).

Wypełnienie jest rysowane przed konturami, a linie warstwy są ustawiane w kolejności, która skraca ruchy z podniesionym pisakiem. Bardzo mały odstęp daje ogromną liczbę linii: zadanie robi najwyżej 200 000 linii wypełnienia, reszta wypełnienia jest pomijana, a ostrzeżenie o tym mówi.

## Rząd warstw pod obszarem roboczym

Po jednym kolorze na każdą z 64 warstw, w rzędzie przewijanym w bok (strzałkami na jego końcach albo kółkiem). Kropka oznacza warstwę z kształtami, przygaszony kolor – ukrytą warstwę, obwódka – aktywną warstwę. Podpowiedź koloru podaje numer i nazwę warstwy.

* **Kliknięcie** przenosi zaznaczone kształty na tę warstwę (zablokowane zostają) i robi z niej aktywną warstwę; gdy nic nie jest zaznaczone, tylko robi ją aktywną.
* **Dwuklik** otwiera kartę **Warstwy** na tej warstwie.
* Z **Ustawienia → Opcje widoku… → Pokaż nazwy warstw** (domyślnie wyłączone) warstwa z kształtami ma obok koloru swoją nazwę (długie nazwy są skracane; pełna jest w podpowiedzi).

## Presety warstw

**Zapisz preset…** i **Wczytaj preset** nad ustawieniami warstwy otwierają okno **Presety warstw**:

* wpisz nazwę i naciśnij **Zapisz preset…**, żeby zachować ustawienia aktywnej warstwy pod tą nazwą (najwyżej 60 presetów);
* pole **Szukaj** zawęża listę; kliknięcie presetu nakłada jego ustawienia na aktywną warstwę; **✎** zmienia jego nazwę, a **✖** go zapomina;
* **Eksportuj…** zapisuje wszystkie presety do pliku, a **Importuj…** dodaje te z pliku.

Preset zachowuje wszystko poza nazwą, kolorem, widocznością, tym, czy warstwa jest rysowana, i jej miejscem w kolejności: sposób rysowania, pisak, szerokość, prędkość, sposób rysowania obrazu i rastrowanie, wypełnienie i liczbę przejść. Preset z pisakiem, którego nie ma już w bibliotece, zostawia warstwę bez pisaka (szerokość zostaje), a pasek stanu mówi *Pisaka z presetu nie ma w bibliotece: warstwa jest bez pisaka*. Presety są w pliku `pen_layer_presets.json` we wspólnym folderze danych (zobacz [Ustawienia i pliki](Ustawienia-i-pliki)).

## Biblioteka pisaków

**Ploter → Biblioteka pisaków…** (albo przycisk **…** obok **Pisak** na karcie Warstwy) otwiera okno **Biblioteka pisaków**. Po lewej są **Gotowe pisaki** i **Moje pisaki**; po prawej próbka wybranego pisaka (linie w czterech kierunkach i kropka, narysowane tak, jak robi je pisak) i jego ustawienia.

### Czym jest pisak

| Ustawienie | Co robi |
|---|---|
| **Nazwa** | Nazwa na liście, w polu **Pisak** warstwy i w komentarzach G-code. |
| **Rodzaj** | **Ołówek**, **Piórko (stalówka)**, **Cienkopis**, **Mazak**, **Zakreślacz** albo **Pisak pędzelkowy**. |
| **Szerokość śladu** | Rozmiar końcówki w poprzek, 0,01 … 50 mm. |
| **Wysokość śladu** | Rozmiar końcówki wzdłuż, 0,01 … 50 mm; przy okrągłej końcówce taki sam jak szerokość. |
| **Obrót końcówki** | Kierunek szerokości ściętej końcówki lub stalówki, w stopniach. Linia wzdłuż szerokości jest tak gruba jak szerokość, linia w poprzek – tak cienka jak wysokość. |
| **Końce linii** | **Zaokrąglone końce** albo **Płaskie końce**. |
| **Kolor tuszu** | Kolor linii w podglądzie. |
| **Krycie** | 5 … 100 %: mniej to przezroczysty tusz (zakreślacz), a tam, gdzie linie się krzyżują, robią się ciemniejsze. |

Pisak warstwy zmienia **Podgląd**: każda linia jest tak gruba, jak robi ją pisak w jej kierunku, z jego końcami, kolorem tuszu i kryciem. Warstwa bez pisaka z biblioteki jest rysowana w swoim kolorze, z zaokrąglonymi końcami, z grubością równą wartości **Szerokość pisaka**.

### Gotowe pisaki

Nazwy gotowych pisaków to dane, nie teksty interfejsu, więc są po angielsku.

| Pisak | Rodzaj | Ślad | Końce | Tusz | Krycie |
|---|---|---|---|---|---|
| Fineliner 0.2 mm | Cienkopis | 0,2 mm | zaokrąglone | czarny | 100 % |
| Fineliner 0.4 mm | Cienkopis | 0,4 mm | zaokrąglone | czarny | 100 % |
| Fineliner 0.8 mm | Cienkopis | 0,8 mm | zaokrąglone | ciemnoniebieski | 100 % |
| Pencil 0.5 mm | Ołówek | 0,5 mm | zaokrąglone | szary | 75 % |
| Fountain pen nib 1.8 x 0.3 mm | Piórko (stalówka) | 1,8 × 0,3 mm, obrót 45° | płaskie | ciemnoniebieski | 95 % |
| Felt-tip marker 1 mm | Mazak | 1 mm | zaokrąglone | czerwony | 90 % |
| Chisel marker 3 x 1 mm | Mazak | 3 × 1 mm, obrót 45° | płaskie | czarny | 95 % |
| Highlighter 4 x 1 mm | Zakreślacz | 4 × 1 mm, obrót 45° | płaskie | żółty | 40 % |
| Brush pen 2 mm | Pisak pędzelkowy | 2 mm | zaokrąglone | czarny | 85 % |

Gotowego pisaka nie da się zmienić (*Gotowego pisaka nie można zmieniać. Zrób kopię, żeby go zmienić.*).

### Własne pisaki

* **Nowy pisak** dodaje pisak do listy **Moje pisaki** (na początek czarny cienkopis 0,4 mm); **Kopiuj** robi kopię wybranego pisaka wśród Twoich; **Usuń** usuwa jeden z Twoich pisaków. Zmiana w polach jest zachowywana od razu.
* **Importuj…** wczytuje pisaki z pliku (`.json`); każdy pisak dostaje własną nazwę (gdy nazwa jest zajęta, dochodzi numer), a pisak, który nie jest poprawny, jest pomijany i wymieniony w komunikacie. **Eksportuj…** zapisuje do pliku wszystkie Twoje pisaki; **Eksportuj ten pisak…** – tylko wybrany (także gotowy).
* Twoje pisaki są przechowywane z ustawieniami programu (najwyżej 200). Projekt niesie te Twoje pisaki, których używają jego warstwy: otwarty na innym komputerze, dołącza je tam do listy **Moje pisaki**.

## Zmiany pisaka

Pisak warstwy to razem jej **kolor**, jej **Szerokość pisaka** i jej **Pisak** z biblioteki. Warstwy są rysowane w kolejności listy; gdy następna warstwa różni się od poprzedniej którąkolwiek z tych trzech rzeczy, pisak jest zmieniany. Dwie warstwy z tym samym pisakiem, kolorem i szerokością są rysowane jedna po drugiej bez zatrzymania; dwie warstwy różniące się tylko kolorem zatrzymują się na zmianę.

* **Bez miejsca na zmianę** (domyślnie): pisak idzie w górę, a zadanie zatrzymuje się tam, gdzie jest, z `M0 (Change the pen to: …)`.
* **Z miejscem** (ustawienia urządzenia, strona **Pisak**, **Zmiana pisaka w stałym miejscu**): pisak idzie w górę, ploter jedzie w to miejsce, wysyła **G-code w tym miejscu** (dla mechanizmu, który zmienia pisak; można tam użyć zmiennych `{layer_name}`, `{layer_index}` i `{total_layers}`) i, gdy zaznaczone jest **Zatrzymaj tam (M0)**, zatrzymuje się z `M0 (Change the pen to: …)`. Bez tego zaznaczenia zapisywany jest tylko komentarz `(Change the pen to: …)` i zadanie idzie dalej. Droga do tego miejsca jest częścią podglądu i czasu.

Tekst po *Change the pen to:* podaje pisak z biblioteki i warstwę (`Fineliner 0.4 mm, Black pen`) albo, bez pisaka z biblioteki, warstwę i jej szerokość (`Red pen, 0.4 mm`). Gdy zadanie jest wysyłane z programu, zatrzymanie otwiera okno **Zadanie jest wstrzymane** z tym tekstem: zmień pisak i naciśnij **Kontynuuj zadanie** (albo **Zatrzymaj zadanie**). Zobacz [Sterowanie maszyną](Sterowanie-maszyna).

Z opcją zadania **Element po elemencie** (karta Sterowanie) element jest rysowany ze wszystkimi swoimi warstwami przed następnym: przy różnych pisakach na tych warstwach pisak jest zmieniany przy każdym przejściu.

## G-code

Program zapisuje G-code dla GRBL. Przy ustawieniach domyślnych (pisak podnoszony osią Z, podniesiony 3 mm, opuszczony 0, czekanie 150 ms, zero maszyny z przodu po lewej) wygląda tak (uwagi po ← nie są w pliku):

```
(PenCreator: nazwa projektu)
(Device: nazwa urządzenia)
(Note: wiersze karty Opis)
G21 G90 G17
G0 Z3            ← pisak w górę (z serwem: M3 S<góra>)
G4 P0.15         ← czekanie po ruchu pisaka, gdy nie jest 0
(Layer: Black pen)
(Pen: Black pen, 0.4 mm)
G0 X… Y…         ← ruch z podniesionym pisakiem
G0 Z0            ← pisak w dół (z serwem: M3 S<dół>)
G4 P0.15
G1 X… Y… F3000   ← linie, z prędkością warstwy
…
(Layer: Red pen)
M0 (Change the pen to: Red pen, 0.4 mm)
…
G0 X0 Y0         ← pisak w górze, powrót do przedniego lewego narożnika obszaru roboczego
M5               ← tylko z serwem
M30
```

* Notatki projektu (karta **Opis**) stają się wierszami `(Note: …)` po nazwie: nawiasy są zamieniane na `<` `>`, `;` i `%` na przecinki, puste wiersze są pomijane, długi wiersz jest cięty na kawałki po 100 znaków i zapisywanych jest najwyżej 60 wierszy.
* Liczby są w milimetrach, we współrzędnych maszyny: narożnik wybrany jako **Zero maszyny (0,0)** urządzenia to X0 Y0.
* **Sam początek pliku**, **Sam koniec pliku** i **Format pliku dla sterowania** urządzenia (strona **G-code**) oraz postprocesory wtyczek są stosowane na końcu. Zobacz [Sterowanie maszyną](Sterowanie-maszyna#uruchamianie-zadania) i [Wtyczki](Wtyczki).

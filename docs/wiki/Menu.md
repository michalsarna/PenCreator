🌐 [English](Menus) · **Polski**

# Menu

Pasek menu ma dziewięć menu, w tej kolejności: **Plik · Edycja · Wstawianie · Rozmieść · Widok · Maszyna · Ploter · Ustawienia · Pomoc** (File · Edit · Insert · Arrange · View · Machine · Plotter · Settings · Help). LightCreator i MillCreator mają ten sam układ (pochodzi z baseCreatora); różni się tylko menu procesu (Laser, Milling, **Ploter**).

* W **Windows i Linuksie** pasek menu jest w oknie, pod paskiem tytułu. Jest widoczny w edytorze, nie na ekranie startowym.
* W **macOS** menu są na systemowym pasku menu u góry ekranu. Ma on dwa menu więcej: menu aplikacji **PenCreator** (**About PenCreator**, zwykłe pozycje systemu, **Zakończ**) i **Window** (minimalizacja i maksymalizacja). Dlatego na Macu **Zakończ** nie stoi na końcu menu **Plik**, a **O programie** na końcu menu **Pomoc**. (Pozycja **About PenCreator** nie ma tłumaczenia, więc jest po angielsku.)
* Pozycja, której teraz nie da się użyć, jest szara (np. **Wektoryzacja obrazu…**, gdy nie zaznaczono obrazu, **Zapisz** na ekranie startowym). Przełącznik ma ptaszek, gdy jest włączony.
* Klawisze są podane obok pozycji; wszystkie są na stronie [Skróty klawiszowe](Skroty-klawiszowe). Działają w każdym systemie; w macOS systemowy pasek menu pokazuje tylko klawisze pozycji **Nowy**, **Otwórz…**, **Zapisz**, **Zapisz jako…** i **Zakończ**.

## Plik

| Pozycja | Klawisz | Co robi |
|---|---|---|
| **Nowy** | Ctrl+N | Nowy, pusty rysunek na arkuszu urządzenia, w osobnej karcie (otwartych może być najwyżej 10 rysunków). |
| **Otwórz…** | Ctrl+O | Otwiera projekt (`.cre`), plik SVG, PDF lub Illustratora (`.ai`) w osobnej karcie; plik DXF lub projekt UGS (`.ugsd`) jest importowany do rysunku. Projekt, który już jest otwarty, jest pokazywany, a nie otwierany drugi raz. Zobacz [Szybki start](Szybki-start) i [Plik projektu](Plik-projektu). |
| **Otwórz ostatnie** | | Ostatnie 10 plików; **Wyczyść listę** opróżnia listę. |
| **Zapisz** | Ctrl+S | Zapisuje projekt jako `.cre`. Rysunek, który nie ma jeszcze pliku projektu (nowy albo otwarty z SVG lub PDF), najpierw pyta o nazwę. |
| **Zapisz jako…** | Ctrl+Shift+S | Zapisuje projekt pod nową nazwą. |
| **Importuj → Importuj obraz…** | | Obraz (PNG, JPG, BMP, GIF, WebP) przez okno importu. Zobacz [Narzędzia projektowe](Narzedzia-projektowe). |
| **Importuj → Importuj SVG…** | Ctrl+I | Ścieżki pliku SVG są dodawane do otwartego rysunku, każda na warstwę o najbliższym kolorze, a jego obrazy na aktywną warstwę. **Otwórz…** zrobiłoby z pliku nową kartę. |
| **Importuj → Importuj Adobe Illustrator / PDF…** | | To samo dla pliku PDF, zgodnego z PDF `.ai` albo `.ai` starego rodzaju w PostScripcie. Plik z warstwami Illustratora pyta, czy układać według koloru, czy według tych warstw. |
| **Importuj → Importuj DXF…** | | Linie, łuki i krzywe rysunku DXF są dodawane do rysunku. |
| **Importuj → Importuj projekt LightBurn / MillMage…** | | Kształty projektu LightBurna (`.lbrn2`, `.lbrn`) albo MillMage'a (`.mage`) są dodawane do rysunku. |
| **Importuj → Importuj projekt UGS…** | | Kształty pliku UGS Designera (`.ugsd`) są dodawane do rysunku. |
| **Importuj → Importuj wtyczką…** | | Plik w formacie, który czyta zainstalowana wtyczka. Zobacz [Wtyczki](Wtyczki). |
| **Importuj → Generuj kształt…** | | Generatory zainstalowanych wtyczek. |
| **Eksportuj → Eksportuj G-code…** | | To samo co **Ploter → Eksportuj G-code…**. |
| **Eksportuj → Eksportuj SVG…** | | Cały rysunek jako SVG, w rozmiarze obszaru roboczego. Krzywe i obrazy zostają. |
| **Eksportuj → Eksportuj PLT / DXF…** | | Obrysy całego rysunku jako HPGL (`.plt`, jeden pisak na warstwę) albo DXF R12, zależnie od wybranego rozszerzenia. To kształty, a nie ruchy maszyny. |
| **Eksportuj → Eksportuj DXF…** | | Cały rysunek jako DXF obecnego rodzaju (AutoCAD 2007); krzywe są spłaszczane do 0,01 mm. |
| **Eksportuj → Eksportuj DXF R12 (starszy)…** | | To samo jako DXF R12, dla starszych programów i ploterów tnących (nazywany też *legacy*). |
| **Eksportuj → Eksportuj Adobe Illustrator…** | | Obrysy jako plik Illustratora w postaci PostScript, z krzywymi, osobna warstwa Illustratora dla każdej warstwy projektu. Obrazy są pomijane. |
| **Eksportuj → Eksportuj projekt LightBurn…** | | Projekt LightBurna (`.lbrn2`, albo `.lbrn`, gdy tak nazwiesz plik): kształty z krzywymi i obrazy, osobna warstwa LightBurna dla każdej warstwy projektu. |
| **Eksportuj → Eksportuj projekt MillMage…** | | To samo jako projekt MillMage (`.mage`), materiał w rozmiarze obszaru roboczego. |
| **Eksportuj → Eksportuj zaznaczenie jako SVG…** / **Eksportuj zaznaczenie jako DXF…** | | Zapisuje zaznaczone kształty (wszystkie, gdy nic nie jest zaznaczone) jako SVG albo DXF. |
| **Pokaż plik G-code…** | | Otwiera plik G-code (`.gcode`, `.nc`, `.gc`, `.gco`, `.ngc`, `.tap`, `.cnc`) w karcie **Analiza G-code**: jego ścieżki, bez zmiany rysunku. |
| **Drukuj…** | Ctrl+P | Drukuje projekt na drukarce papierowej albo zapisuje go jako PDF. |
| **Zakończ** | Ctrl+Q | Zamyka program; o każdy rysunek z niezapisaną pracą najpierw pyta. Tylko Windows i Linux (w macOS jest w menu aplikacji). |

## Edycja

| Pozycja | Klawisz | Co robi |
|---|---|---|
| **Cofnij** | Ctrl+Z | Cofa ostatnią zmianę rysunku (do 100 kroków). |
| **Ponów** | Ctrl+Y | Robi ją znowu (działa też Ctrl+Shift+Z). |
| **Kopiuj** | Ctrl+C | Kopiuje zaznaczone kształty. |
| **Wklej** | Ctrl+V | Wkleja skopiowane kształty. |
| **Powiel** | Ctrl+D | Kopiuje zaznaczone kształty i od razu je wkleja. |
| **Usuń** | Del | Usuwa zaznaczone kształty (z narzędziem **Edycja węzłów (N)** – zaznaczone węzły). |
| **Zaznacz wszystko** | Ctrl+A | Zaznacza każdy kształt na pokazanych warstwach. |

## Wstawianie

Narzędzia lewej palety, które kładą na stronie nowe obiekty: **Prostokąt (R)**, **Elipsa (E)**, **Trójkąt (Y)**, **Gwiazda (S)**, **Wielokąt (G)**, **Serce (K)**, **Spirala (I)**, **Linia (L)**, **Linia łamana / pióro (P)** i **Tekst (T)**. Używane narzędzie ma ptaszek. Dalej:

* **Generuj kształt…**: generatory zainstalowanych wtyczek (to samo co w **Plik → Importuj**).
* **Online clipart…** (ta pozycja nie ma tłumaczenia): otwiera kartę **Clipart** i wyszukiwanie darmowych obrazków w internecie.

Nowe kształty trafiają na aktywną warstwę (zobacz [Warstwy i pisaki](Warstwy-i-pisaki)).

## Rozmieść

Większość pozycji działa na zaznaczonych kształtach; te same polecenia są w menu prawego przycisku kształtu i na pasku po prawej stronie obszaru roboczego. Są opisane na stronie [Narzędzia projektowe](Narzedzia-projektowe).

| Pozycje | Klawisz | Co robią |
|---|---|---|
| **Wyrównaj do lewej**, **Wyśrodkuj w poziomie**, **Wyrównaj do prawej**, **Wyrównaj do góry**, **Wyśrodkuj w pionie**, **Wyrównaj do dołu** | | Ustawiają zaznaczone kształty w linii. |
| **Wyśrodkuj na stole** | | Stawia zaznaczenie na środku obszaru roboczego. |
| **Wyśrodkuj względem siebie** | | Przesuwa zaznaczone kształty (lub grupy) na środek zaznaczenia, jeden na drugi (dwa lub więcej). |
| **Odbij w poziomie**, **Odbij w pionie** | | Odbija zaznaczenie. |
| **Obróć 90° w prawo**, **Obróć 90° w lewo** | | Obraca zaznaczenie o ćwierć obrotu. |
| **Grupuj** | Ctrl+G | Robi jedną grupę z zaznaczonych kształtów. |
| **Rozgrupuj** | Ctrl+Shift+G | Rozbija grupę. |
| **Zablokuj** | Ctrl+L | Zablokowanych kształtów nie da się przesuwać ani zmieniać. |
| **Odblokuj** | Ctrl+Shift+L | Zwalnia je. |
| **Przesuń na wierzch**, **Przesuń na spód** | | Zmienia kolejność kształtów w rysunku. |
| **Edytuj węzły** | N | Narzędzie **Edycja węzłów (N)** na zaznaczonym kształcie. |
| **Zamień na ścieżkę** | | Robi ścieżkę z kształtu (prostokąta, gwiazdy…), żeby można było edytować jego węzły. |
| **Suma**, **Część wspólna**, **Odejmij**, **Różnica symetryczna** | | Łączą, tną i składają kształty zamknięte. |
| **Przesuń obrys…**, **Zaokrąglij rogi…**, **Zamień na krzywe** | | Większy lub mniejszy kontur, zaokrąglone rogi, krzywe zamiast linii. |
| **Połącz ścieżki automatycznie…**, **Zamknij ścieżkę z tolerancją…** | | Łączy otwarte ścieżki, których końce są blisko siebie, zamyka ścieżki, których końce są blisko. |
| **Narożniki dogbone / T-bone…** | | Wcięcia w ostrych narożnikach (do frezowania; to samo we wszystkich trzech programach). |
| **Tablica siatkowa…**, **Tablica kołowa…**, **Kopiuj wzdłuż ścieżki…**, **Tekst na ścieżce…** | | Kopie w siatce, wokół środka, wzdłuż ścieżki; tekst wygięty wzdłuż ścieżki. |
| **Przytnij kształtem…**, **Usuń duplikaty**, **Rozdziel**, **Uprość…** | | Przycina linie kształtem, usuwa podwójne, dzieli kształt na kontury, usuwa zbędne punkty. |
| **Tekst seryjny…**, **Kod kreskowy / QR…**, **Rozmieść kształty (nesting)…**, **Pomiar…** | | Numerowane teksty, kody, upakowanie kształtów na obszarze, pomiar. |
| **Dostosuj obraz…**, **Wektoryzacja obrazu…** | | Zmienia zaznaczony obraz albo zamienia go na kontury. Szare, gdy nie zaznaczono obrazu. |
| **Obrysuj w stylu…** | | Rysuje zaznaczony obraz na nowo liniami w jednym z 19 stylów. Zobacz [Obrysuj w stylu](Obrys-w-stylu). |

## Widok

| Pozycja | Klawisz | Co robi |
|---|---|---|
| **Siatka**, **Przyciągaj do siatki**, **Przyciągaj do obiektów** | | Przełączniki (z ptaszkiem, gdy włączone): siatka na obszarze roboczym, przyciąganie do siatki, przyciąganie do innych kształtów. |
| **Powiększ** | Ctrl++ | Przybliża względem środka widoku. |
| **Pomniejsz** | Ctrl+- | Oddala. |
| **Dopasuj stół do okna** | Ctrl+Shift+0 | Pokazuje cały obszar roboczy. |
| **Dopasuj do wszystkich obiektów** | Ctrl+Shift+9 | Pokazuje wszystkie kształty. |
| **Automatycznie dopasuj obszar roboczy do okna** | | Przełącznik: obszar roboczy zostaje dopasowany, gdy okno zmienia rozmiar. Ręczne przybliżanie lub przesuwanie go wyłącza. |
| **Podziel widok: obok siebie**, **Podziel widok: jeden nad drugim** | | Dzieli obszar roboczy na panele (najwyżej trzy). Przy jednym panelu jego ostatnia karta trafia do nowego panelu (gdy panel ma co najmniej dwie karty). Zobacz [Narzędzia projektowe](Narzedzia-projektowe#karty-i-panele). |
| **Jeden widok** | | Zbiera wszystkie karty znów w jednym panelu. |
| **Pokaż nakładkę z kamery** | | Pokazuje lub ukrywa obraz pod rysunkiem. |
| **Nakładka z…** | | Zdjęcie albo obraz na żywo z kamery pod rysunkiem, ustawiane czterema narożnikami albo punktami odniesienia. Zapisywana w projekcie. |

## Maszyna

| Pozycja | Co robi |
|---|---|
| **Zmień urządzenie…** | Wraca do ekranu startowego, aby wybrać inne urządzenie. |
| **Ustawienia urządzenia…** | Ustawienia urządzenia projektu (strony Ogólne, Maszyna, Pisak, G-code, Połączenie, Kamera, Przesuw ręczny, Makra, Konserwacja, Notatki). Zobacz [Szybki start](Szybki-start) i [Sterowanie maszyną](Sterowanie-maszyna). |
| **Ustawienia firmware ($$)…** | Ustawienia kontrolera GRBL, z kopiami i kopią zapasową. |
| **Kalibracja…** | Kroki na milimetr i prostopadłość osi. |
| **Urządzenia sterujące ▸** | Ustawienia urządzeń sterujących… otwiera okno klawiatur i padów oraz ich klawiszy; Urządzenie sterujące tej maszyny… wybiera urządzenie otwartej maszyny. Zobacz [Urządzenia sterujące](Urzadzenia-sterujace). |
| **Diagnostyka i statystyki…** | Wejścia kontrolera, statystyki i konserwacja maszyny, raport do zapisania. |
| **Udostępnij w sieci (ser2net)…** | Tylko Linux: udostępnia port szeregowy maszyny w sieci. |
| **Obrys** | Obchodzi prostokąt zadania (wymaga podłączonej maszyny). |
| **Rozpocznij zadanie** | Wysyła zadanie (wymaga podłączonej maszyny). |
| **Start od wiersza…** | Otwiera okno, które zaczyna zadanie od wiersza programu (gdy jest co rysować); start wymaga podłączonej maszyny. |
| **Sprawdź program** | Kontroler przechodzi przez program w trybie sprawdzania (`$C`); nic się nie rusza. Wymaga podłączonej maszyny. |

Wszystkie są opisane na stronie [Sterowanie maszyną](Sterowanie-maszyna).

## Ploter

| Pozycja | Klawisz | Co robi |
|---|---|---|
| **Biblioteka pisaków…** | | Gotowe pisaki i Twoje własne. Zobacz [Warstwy i pisaki](Warstwy-i-pisaki#biblioteka-pisaków). |
| **Biblioteka noży…** | | Gotowe noże wleczone i Twoje własne. Zobacz [Narzędzia projektowe](Narzedzia-projektowe#nóż-wleczony). |
| **Wykres zadania…** | Ctrl+Shift+P | Otwiera kartę **Podgląd**: co narysuje pisak, z ruchami z podniesionym pisakiem, czasem i animacją. Zobacz [Szybki start](Szybki-start). |
| **Eksportuj G-code…** | Ctrl+E | Pyta, gdzie zapisać, i zapisuje G-code projektu (to samo robi przycisk **Zapisz G-code**). |

## Ustawienia

| Pozycja | Co robi |
|---|---|
| **Opcje widoku…** | Siatka, kolory obszaru roboczego, grubość linii, **Pokaż nazwy warstw**, **Podgląd G-code po zapisaniu**, **Pokaż nazwy zmiennych**, rozmiar czcionki i kolory **konsoli G-code**. |
| **Opcje programu…** | **Szukaj nowszej wersji przy uruchamianiu programu** (domyślnie włączone). |
| **Wtyczki…** | Instalowanie, włączanie i usuwanie wtyczek. Zobacz [Wtyczki](Wtyczki). |
| **Język** | 30 języków; wybór jest zapamiętywany. |
| **Schemat kolorów** | 18 schematów kolorów (i świąteczny w okolicy niektórych świąt). |

## Pomoc

| Pozycja | Co robi |
|---|---|
| **Strona projektu na GitHubie** | Otwiera stronę PenCreatora w przeglądarce. |
| **Pobierz kopię zapasową…** | Zapisuje ustawienia i wspólną galerię clipartów w pliku zip. Zobacz [Ustawienia i pliki](Ustawienia-i-pliki). |
| **Przywróć z kopii zapasowej…** | Wczytuje taki plik zip po pytaniu. |
| **Krótki przewodnik** (F1) | Przewodnik w programie, z wyszukiwarką. |
| **Eksportuj diagnostykę (zip)…** | Jeden plik z raportem, ustawieniami i konsolą, do dołączenia do zgłoszenia błędu. |
| **Sprawdź aktualizacje…** | Szuka nowszego wydania. |
| **O programie** | Wersja, licencja i podziękowania. Tylko Windows i Linux (w macOS: **About PenCreator** w menu aplikacji). |

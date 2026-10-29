🌐 [English](Machine-Control) · **Polski**

# Sterowanie maszyną

PenCreator steruje maszyną z GRBL z karty **Sterowanie** i z menu **Machine**. Wszystko to pochodzi z baseCreatora, więc MillCreator, PenCreator i LightCreator działają tak samo. **Nic z tego nie było jeszcze próbowane na prawdziwej maszynie**: próbuj z podniesionym narzędziem albo bez pisaka i miej rękę przy zatrzymaniu.

## Odczyt maszyny

Połącz się na karcie **Device**. Program odczytuje kontroler (`$I` – firmware, `$$` – ustawienia, `$G` – stan). W ustawieniach urządzenia, na stronie **Machine**, przycisk **Użyj tych wartości dla urządzenia** kopiuje zakres ruchu, i prędkość ruchów z kontrolera do urządzenia, więc nie musisz ich wpisywać. Nowe urządzenie może też zacząć od **Zwykły rozmiar arkusza…** (tylko punkt wyjścia).

## Karta Sterowanie

* **Pozycja:** pozycja robocza i maszynowa X, Y i Z, dużymi cyframi. **X0 Y0 Z0** (po lewej każdej osi) robią z miejsca, gdzie jest narzędzie, zero pracy (`G10 L20`); kliknij liczbę, aby wpisać pozycję. Pod nimi: **Zero all**, **Zero X and Y**, **Jedź do X0 Y0** (najpierw podnosi na bezpieczną wysokość) i **Z na bezpieczną wysokość**. **Współrzędne** wybierają G54 … G59.
* **Przesuw:** odległość jednego kliknięcia dla XY i dla Z to pole, w którym można pisać; przyciski obok wstawiają tam wartość (XY 0,1 … 100, Z 0,1 … 20; wartości ustawia się w urządzeniu, strona **Jogging**). **Prędkość** to prędkości XY i Z urządzenia (zmiana tutaj trwa do zmiany w urządzeniu).
* **Przyciski ruchu** (duże, pośrodku): strzałki, **Z▲ Z▼**; kwadrat zatrzymuje ruch. Jedno kliknięcie to jeden ruch. Z opcją **Przytrzymaj, aby jechać** narzędzie jedzie, dopóki przycisk jest wciśnięty, i staje po puszczeniu. **Klawiatura** (domyślnie wyłączona, bo strzałki przesuwają też kształty) używa strzałek i Page Up / Page Down o wybrany krok.
* **Podnieś pisak / Opuść pisak** (pod Home, Unlock, Pause i Resume): przesuwają pisak tam, gdzie stoi, osią Z lub serwem urządzenia, do pozycji *góra* albo *dół* ustawionej na stronie **Pen** (i czekają przez ustawione tam opóźnienie). Przydają się do ręcznego ustawienia wysokości pisaka przed zadaniem; podczas zadania są wyłączone.
* **Makra:** przyciski wysyłające napisany przez Ciebie G-code (strona **Macros** ustawień urządzenia). Zmienne w nawiasach kwadratowych są podmieniane przy wysyłaniu: `[posx] [posy] [posz]` (pozycja robocza), `[mposx]` …, `[xmin] [xmax] [ymin] [ymax]` (obrys zadania), `[safez]`, `[prbx] [prby] [prbz]` (ostatnie sondowanie). `$RST` jest w makrze zabronione. Ta sama strona ma G-code uruchamiany **przed** i **po** każdym zadaniu (także po zatrzymaniu).

## Uruchamianie zadania

* **Start** wysyła całe zadanie; **Od wiersza…** (też w menu *Machine*) zaczyna od wybranego wiersza programu: maszyna jest najpierw doprowadzana do stanu, jaki program ma w tym miejscu (jednostki, układ współrzędnych, pisak, posuw, pozycja), na bezpiecznej wysokości, a reszta idzie dalej. Odmawia, gdy nie umie tego obliczyć (np. po `G53` lub `G92`).
* **Sprawdź program** (menu *Machine* albo *Check the program before every run* na stronie **Jogging**): kontroler przechodzi przez program w trybie sprawdzania (`$C`). Nic się nie rusza, a wiersze, których nie rozumie, są zgłaszane z numerem. Przy włączonej opcji właściwe zadanie rusza dopiero, gdy sprawdzanie nic nie znalazło.
* **Opcje zadania** (pod przyciskami Frame i From line…; tylko w tej sesji, nie są zapisywane; można je włączyć razem albo osobno, zmieniają G-code, podgląd i oszacowanie): **One element at a time** (Element po elemencie) rysuje każdy element, czyli grupę (także obejmującą kilka warstw) albo kształt bez grupy, w całości, ze wszystkimi jego warstwami i przejściami, zanim zacznie następny; pierwszy jest element najbliższy narożnikowi startu (wyłączone: warstwa po warstwie). Przeznaczone dla warstw z tym samym pisakiem: przy różnych pisakach przy każdej zmianie następuje wymiana (pauza). **Selection only** (Tylko zaznaczenie) rysuje tylko kształty zaznaczone na obszarze roboczym w chwili zapisu G-code lub startu zadania; bez zaznaczenia nie ma czego rysować.
* **Zmiana pisaka w stałym miejscu:** w ustawieniach urządzenia, na stronie **Pen**, włącz *Change the pen at a fixed place* i podaj miejsce (X i Y, we współrzędnych obszaru roboczego; może leżeć poza nim, np. tam, gdzie stoi uchwyt albo mechanizm). Gdy następna warstwa wymaga innego pisaka, pisak się unosi, ploter jedzie w to miejsce, wysyła *G-code at the place* (do 20 linii, dla mechanizmu, który sam zmienia pisak) i zatrzymuje się tam (`M0`), żeby pisak można było zmienić ręcznie lub mechanizmem; *Stop there (M0)* można wyłączyć dla mechanizmu, który nie wymaga czekania. Droga do tego miejsca jest częścią podglądu i oszacowania. Bez miejsca zadanie zatrzymuje się tam, gdzie jest. Sprawdzone tylko w symulatorze, nie na maszynie ze zmieniaczem pisaków.
* **Podgląd jako karta:** ikona z kartą w linii tytułu okna podglądu (obok przycisku zamknięcia) przypina podgląd jako kartę obszaru roboczego, obok rysunków i widoku z kamery. W karcie ikona z małym oknem zamienia ją z powrotem w okno, a krzyżyk karty zamyka podgląd. Przycisk *Preview…* otwiera go znów tak, jak był ostatnio (okno albo karta).
* **Pauzy:** Gdy program dojdzie do zmiany pisaka, maszyna zatrzymuje się tam, a okno mówi, który pisak założyć. **Kontynuuj zadanie** idzie dalej, **Zatrzymaj zadanie** je kończy. Gdy zadanie czeka, możesz przesuwać maszynę, zerować i sondować na karcie Sterowanie.
* Błędy i alarmy w trakcie zadania są liczone, a pierwszy wiersz każdego jest zapamiętywany.

## Menu Machine

* **Firmware settings ($$):** ustawienia kontrolera z wyszukiwaniem i filtrem pokazującym tylko te, które różnią się od typowej wartości. Zmiana jest zapisywana po pytaniu, a ustawienia są potem czytane ponownie. **Pierwsza zmiana w sesji zapisuje wszystkie ustawienia do pliku** w folderze danych (`firmware-backups`). **Back up to a file…** i **Restore from a file…** robią to ręcznie (plik jest porównywany z kontrolerem przed zapisem i czytany z powrotem po nim); **Factory settings ($RST=$)…** wymaga wpisania słowa RESET.
* **Calibration:** *kroki na milimetr* (zleć ruch, zmierz go, dostaniesz nowe `$100` / `$101` / `$102` i zapiszesz) oraz *prostopadłość osi* (wpisz dwie przekątne prostokąta: pokazany jest kąt i przesunięcie).
* **Diagnostics and statistics:** wejścia kontrolera (wyłączniki krańcowe, sonda, osłona, hold, reset, start), firmware, zadania, czas pracy, alarmy i błędy tej maszyny oraz **przypomnienia o konserwacji** (np. „nasmaruj prowadnice co 50 godzin”; ustawisz je na stronie **Maintenance**). **Export diagnostics (zip)…** zapisuje raport, ustawienia, konsolę i program.

## Kopie ustawień kontrolera

Każde urządzenie przechowuje kopie ustawień swojego kontrolera (`$$`). Pierwsza powstaje przy pierwszym połączeniu z maszyną, a kolejne za każdym razem, gdy ustawienia odczytane przy łączeniu różnią się od najnowszej kopii: pasek stanu i konsola mówią, ile ustawień się zmieniło i które (`$110: 3000 → 5000`), więc zauważysz zmianę zrobioną z innego programu albo przez sam kontroler. Zmiana zrobiona tutaj (zapis w **Firmware settings**) dostaje własną kopię bez alarmu. **Zrób kopię teraz** w oknie ustawień firmware i to samo w ustawieniach urządzenia, na stronie **Machine**, robi kopię ręcznie; **Przywróć…** porównuje kopię z kontrolerem i po pytaniu zapisuje różnice z powrotem; **Usuń** kasuje kopię. Przechowywanych jest osiem najnowszych. Kopie są częścią urządzenia, więc są w **kopii zapasowej programu** i w wyeksportowanym pliku urządzenia.

## Alarmy i błędy

**Wersje GRBL.** Wersję płyty wybierasz w polu *GRBL version* na stronie Połączenie urządzenia: **1.1 lub nowszy** (domyślnie; także grblHAL i FluidNC), **1.0**, **0.9**, **0.8** albo **0.7 lub starszy**. GRBL 0.8, 0.9 i 1.0 podają położenie jako `<Idle,MPos:…,WPos:…>` i nie mają polecenia `$J=`, więc program przesuwa głowicę zwykłymi ruchami `G91` / `G1`; takiego ruchu nie da się przerwać w połowie, kończy się na końcu swojego kroku. GRBL 0.7 nie ma żadnych poleceń czasu rzeczywistego: nie ma raportu stanu (więc i położenia), Pauzy, Wznowienia, bazowania ani odblokowania, a Stop tylko porzuca wiersze, które jeszcze nie poszły, bo płyta dokończy to, co już ma; ten firmware ucina też wiersze dłuższe niż 49 znaków. Serwo podnoszące pisak na wyjściu wrzeciona (`M3 S…`) działa tak samo w każdej wersji.

Gdy GRBL powie `ALARM:n` albo `error:n`, konsola dostaje pod spodem wiersz z wyjaśnieniem, a pasek stanu je pokazuje. Wyjaśnienia są w 30 językach programu.

> ⚠️ Sondowanie, przesuw i pauzy wysyłają prawdziwe ruchy. Miej zatrzymanie w zasięgu ręki.

## Gdy maszyna się zatrzyma

Alarm, błąd w zadaniu albo sterownik, który nie odpowiada, otwiera **okno pośrodku ekranu**: co się stało oraz dwa duże przyciski, **STOP** (czerwony: zatrzymuje zadanie i wrzeciono) i **UNLOCK** (pomarańczowy: kasuje alarm, `$X`). Zobacz, dlaczego się zatrzymało, zanim odblokujesz: narzędzie może tkwić w materiale.

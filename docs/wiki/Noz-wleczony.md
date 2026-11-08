🌐 [English](Drag-Knife) · **Polski**

# Nóż wleczony

Ploter może **ciąć** zamiast rysować: folię, flex, papier, karton, szablony. Narzędziem jest **nóż wleczony**: ostrze, które obraca się swobodnie w uchwycie, a jego czubek wlecze się trochę za osią uchwytu, tak jak kółko krzesła za swoim sworzniem. PenCreator prowadzi oś tak, żeby po Twoich liniach szedł **czubek**.

## Jak program koryguje nóż

* **Przesunięcie czubka:** jak daleko czubek ostrza jest za jego osią (od 0,15 do 0,5 mm przy zwykłych ostrzach; podaje je producent ostrza). Program prowadzi oś o tyle przed czubkiem, więc cięcie leży na linii, a nie obok niej.
* **Obracaj na narożach ostrzejszych niż** (domyślnie 15°): na ostrzejszym narożu oś obchodzi je po małym łuku, ostrze obraca się w miejscu i naroże zostaje ostre. Łagodniejsze naroże jest cięte na wprost.
* **Tnij dalej za początkiem** (domyślnie 0,5 mm): zamknięty obrys jest cięty o tyle dalej za swój początek, żeby się domknął, a część wypadła.
* **Kierunek ostrza** przechodzi z cięcia na cięcie: każde cięcie zaczyna się od obrócenia ostrza z kierunku, w którym zostało.
* **Najpierw otwory:** otwory są cięte przed częściami wokół nich, więc część trzyma się materiału, dopóki nie zostanie wycięta.
* Cięte są tylko **obrysy**: wypełnienia i obrazy są pomijane (podgląd o tym mówi). Obrys mniejszy niż mniej więcej cztery przesunięcia wychodzi zaokrąglony; program o tym ostrzega.

## Biblioteka noży

**Ploter → Biblioteka noży…** (albo przycisk **Biblioteka noży…** obok noża urządzenia) otwiera bibliotekę. Po lewej: **Gotowe noże** i **Moje noże**, po prawej wartości wybranego noża.

| Gotowy nóż | Przesunięcie czubka | Obracaj na narożach ostrzejszych niż | Tnij dalej za początkiem | Do czego |
|---|---|---|---|---|
| Ostrze 30° | 0,15 mm | 15° | 0,4 mm | cienka folia i papier, drobne szczegóły i małe litery |
| Ostrze 45° | 0,25 mm | 15° | 0,5 mm | folia, flex, papier i cienki karton: zwykłe ostrze plotera |
| Ostrze 60° | 0,5 mm | 15° | 0,8 mm | gruba folia, karton, guma, pianka i szablony do piaskowania |
| Nóż wleczony w uchwycie maszyny CNC | 0,5 mm | 10° | 1,0 mm | uchwyt w tulejce wrzeciona (zob. MillCreator) |

* Gotowego noża nie można zmienić. **Kopiuj** wstawia jego kopię do **Moich noży**, gdzie nadajesz jej **Nazwę**, wartości i **Notatki** (ostrze, materiał, nacisk, który się sprawdził). **Usuń** usuwa własny nóż.
* Twoje noże są zapisywane z ustawieniami programu i w jego kopii zapasowej (**Ustawienia → Kopia zapasowa**).
* Urządzenie **kopiuje** wartości noża: późniejsza zmiana w bibliotece nie zmienia urządzenia, które wzięło nóż wcześniej. Gdy wartości urządzenia nie pasują do żadnego noża z biblioteki, lista pokazuje **Własne wartości**.

## Ustawienie plotera tnącego

1. Otwórz okno urządzenia i przejdź na stronę **Pisak**.
2. Zaznacz **Narzędziem jest nóż wleczony**.
3. Wybierz **Nóż** z listy albo ustaw jego wartości ręcznie.
4. **Ploter sam koryguje przesunięcie:** większość ploterów do folii (HPGL) robi to w sterowniku. Zaznacz to, a obrysy będą cięte tak, jak są narysowane (nadal najpierw otwory); zostaw wyłączone dla plotera, który tego nie robi (ploter GRBL z uchwytem noża).
5. Ostrze opuszcza się i podnosi jak pisak (oś Z, serwo albo własny G-code): przy osi Z **Pisak opuszczony** to głębokość cięcia.

**Podgląd** rysuje ścieżkę osi na szaro, a to, co tnie czubek, kolorem warstwy.

## Plotery HPGL: polecenia urządzenia

Plotery i plotery tnące rozumieją HPGL trochę inaczej. Gdy sterownikiem urządzenia jest **Ploter HPGL (nie GRBL)** (strona **Połączenie**), okno urządzenia ma stronę **HPGL**, tylko dla takich urządzeń, na której ustawiasz, co dostaje to urządzenie. Polecenia kończą się `;`, a w miejsce `{speed}` i `{force}` wstawiane są wartości.

| Pole | Domyślnie | Kiedy jest wysyłane |
|---|---|---|
| **Na początku** | `IN;SP1;PU;` | raz, gdy ploter się połączy |
| **Na końcu** | `PU;` | na końcu zadania (`M2`, `M30`), po podniesieniu noża |
| **Opuszczenie pisaka lub noża** | `PD;` | przed każdym cięciem |
| **Podniesienie pisaka lub noża** | `PU;` | po każdym cięciu i przy zatrzymaniu |
| **Prędkość** | `VS{speed};`, **najwyżej** 38 | gdy zmienia się prędkość rysowania, nigdy powyżej prędkości maksymalnej; puste: nigdy (ploter zachowuje własną) |
| **Nacisk** | `FS{force};`, 0 g | raz na początku; 0: nigdy (ploter zachowuje własny) |

**Przywróć zwykłe polecenia** wstawia z powrotem wartości domyślne. Urządzenie utworzone przed tą stroną dostaje wartości domyślne. Polecenia początkowe i nacisk są wysyłane przy połączeniu z ploterem: po ich zmianie połącz się ponownie.

## Pierwsze cięcie

Najpierw wytnij mały test (kwadrat z kołem w środku) na zbędnym kawałku. Zaokrąglone naroże znaczy, że przesunięcie jest za małe, mała pętelka albo haczyk na narożu, że za duże; obrys, który się nie domyka, potrzebuje dłuższego **Tnij dalej za początkiem**. Gdy wszystko jest dobrze, **skopiuj** nóż do **Moich noży**, z wartościami i naciskiem w notatkach.

Zobacz też: [Narzędzia projektowe](Narzedzia-projektowe#nóż-wleczony), [Sterowanie maszyną](Sterowanie-maszyna), [Bezpieczeństwo i ograniczenia](Bezpieczenstwo-i-ograniczenia).

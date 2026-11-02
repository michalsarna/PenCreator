🌐 [English](Keyboard-Shortcuts) · **Polski**

# Skróty klawiszowe

Ctrl to na Macu Cmd. Klawisze działają w edytorze (nie na ekranie startowym) i żaden z nich nie jest przejmowany, gdy pole tekstowe ma klawiaturę: pisanie w polu, w karcie **Opis** albo w karcie z tekstem G-code wpisuje litery. Na Macu systemowy pasek menu nie pokazuje wszystkich klawiszy; zobacz [W macOS](#w-macos).

## Klawisze menu

Są pokazane obok pozycji menu (zobacz [Menu](Menu)). Przy pasku menu w oknie działają także wtedy, gdy menu jest zamknięte; pozycja, która w tej chwili jest szara, nic nie robi.

| Klawisz | Polecenie |
|---|---|
| Ctrl+N | **Plik → Nowy** |
| Ctrl+O | **Plik → Otwórz…** |
| Ctrl+S | **Plik → Zapisz** |
| Ctrl+Shift+S | **Plik → Zapisz jako…** |
| Ctrl+Q | **Plik → Zakończ** |
| Ctrl+Z | **Edycja → Cofnij** |
| Ctrl+Y albo Ctrl+Shift+Z | **Edycja → Ponów** |
| Ctrl+C | **Edycja → Kopiuj** |
| Ctrl+V | **Edycja → Wklej** |
| Ctrl+D | **Edycja → Powiel** |
| Delete albo Backspace | **Edycja → Usuń** (z narzędziem **Edycja węzłów (N)** i zaznaczonymi węzłami: usuwa węzły) |
| Ctrl+A | **Edycja → Zaznacz wszystko** |
| Ctrl+G | **Rozmieść → Grupuj** |
| Ctrl+Shift+G | **Rozmieść → Rozgrupuj** |
| Ctrl+L | **Rozmieść → Zablokuj** |
| Ctrl+Shift+L | **Rozmieść → Odblokuj** |
| N | **Rozmieść → Edytuj węzły** (narzędzie **Edycja węzłów (N)**) |
| Ctrl++ (Ctrl+=) | **Widok → Powiększ** |
| Ctrl+- | **Widok → Pomniejsz** |
| Ctrl+Shift+0 | **Widok → Dopasuj stół do okna** |
| Ctrl+Shift+9 | **Widok → Dopasuj do wszystkich obiektów** |
| Ctrl+Shift+P | **Ploter → Wykres zadania…** |
| Ctrl+E | **Ploter → Eksportuj G-code…** |

Żadne dwie pozycje menu nie mają tego samego klawisza; sprawdza to test programu.

## Karty rysunków

| Klawisz | Co robi |
|---|---|
| Ctrl+1 … Ctrl+9, Ctrl+0 | Pokazuje rysunek 1. … 9. karty, Ctrl+0 – 10. (otwartych może być najwyżej 10 rysunków). |
| Ctrl+W | Zamyka pokazany rysunek; gdy ma niezapisaną pracę, najpierw pyta (**Zapisz**, **Odrzuć**, **Anuluj**). |

## Narzędzia

Litera bez Ctrl wybiera narzędzie z lewej palety (litera jest w podpowiedzi jego przycisku):

| Klawisz | Narzędzie | Klawisz | Narzędzie |
|---|---|---|---|
| V | **Zaznaczanie (V)** | L | **Linia (L)** |
| N | **Edycja węzłów (N)** | P | **Linia łamana / pióro (P)** |
| R | **Prostokąt (R)** | T | **Tekst (T)** |
| E | **Elipsa (E)** | H | **Przesuwanie widoku (H)** |
| Y | **Trójkąt (Y)** | Z | **Powiększ (Z)** |
| S | **Gwiazda (S)** | X | **Pomniejsz (X)** |
| G | **Wielokąt (G)** | | |
| K | **Serce (K)** | | |
| I | **Spirala (I)** | | |

| Klawisz | Co robi |
|---|---|
| Esc | Czyści zaznaczenie (i rysowaną linię łamaną). |
| Esc dwa razy (w ciągu pół sekundy) | Wraca do narzędzia **Zaznaczanie (V)**. |
| Enter | Kończy linię łamaną rysowaną narzędziem **Linia łamana / pióro (P)**. |

## Mysz z klawiszami

| Czynność | Co robi |
|---|---|
| Kółko (albo gest szczypania) na obszarze roboczym | Przybliża i oddala względem wskaźnika. |
| Przeciąganie środkowym przyciskiem | Przesuwa widok (z narzędziem **Przesuwanie widoku (H)** robi to lewy przycisk). |
| Shift+kliknięcie albo Ctrl+kliknięcie | Dodaje kształt do zaznaczenia albo go z niego wyjmuje. |
| Shift podczas rysowania prostokąta, elipsy, trójkąta, gwiazdy, wielokąta, serca albo spirali | Szerokość i wysokość zostają równe. |
| Przeciąganie uchwytu w rogu | Zachowuje proporcje; z Shift szerokość i wysokość zmieniają się niezależnie. |
| Shift podczas obracania | Obraca co 15°. |
| Shift z **Powiększ (Z)** / **Pomniejsz (X)** | Działa odwrotnie. |
| Prawy przycisk albo dwuklik z **Linia łamana / pióro (P)** | Kończy linię łamaną; kliknięcie blisko pierwszego punktu ją zamyka. |
| Prawy przycisk na kształcie | Menu kształtu (zobacz [Narzędzia projektowe](Narzedzia-projektowe)). |
| Kliknięcie środkowym przyciskiem na karcie panelu | Zamyka kartę. |
| Dwuklik na linii między panelami | Wyrównuje rozmiary paneli. |
| Kółko nad rzędem warstw pod obszarem roboczym | Przewija rząd w bok. |

W karcie **Podgląd** kółko przybliża, przeciąganie przesuwa obraz, a dwuklik wraca do pierwszego widoku.

## Karta Sterowanie

Gdy na karcie **Sterowanie** zaznaczysz **Klawiatura** (domyślnie wyłączone), strzałki przesuwają X i Y, a Page Up / Page Down – Z, o wybrany tam krok. Zobacz [Sterowanie maszyną](Sterowanie-maszyna).

## W macOS

Menu są na systemowym pasku menu macOS. Wszystkie klawisze z tej strony działają także tam:

* **Cmd+N**, **Cmd+O**, **Cmd+S**, **Cmd+Shift+S** i **Cmd+Q** (w menu aplikacji **PenCreator**) należą do systemowego paska menu, który pokazuje je obok pozycji.
* Pozostałe klawisze menu (**Cmd+E**, **Cmd+Shift+P**, **Cmd+L**, **Cmd+Shift+L**, **Cmd+=** / **Cmd+-**, **Cmd+Shift+0** / **Cmd+Shift+9**, klawisze edycji) przejmuje sam program w swoim oknie. Działają tak samo jak w Windows i Linuksie, ale systemowy pasek menu nie pokazuje ich obok pozycji; wymienia je tabela wyżej.

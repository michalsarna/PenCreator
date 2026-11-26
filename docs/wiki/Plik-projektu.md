🌐 [English](Project-File) · **Polski** · [简体中文](Project-File-ZH) · [한국어](Project-File-KO)

# Plik projektu

PenCreator zapisuje **`.cre`**, plik projektu baseCreatora, który zapisują też LightCreator i MillCreator: jeden plik JSON z rysunkiem, opisem i **jedną sekcją na program** (`laser`, `mill`, `pen`). Każdy program zapisuje swoją sekcję i zostawia nietknięte sekcje pozostałych, więc projekt przechodzi między programami bez strat.

Sekcja `pen` zawiera 64 warstwy (nazwa, kolor, pokazana i rysowana, sposób rysowania, pisak z biblioteki, szerokość pisaka, prędkość, odstęp i kąt linii wypełnienia, linie krzyżowe, ile razy rysować, sposób rysowania obrazu i rastrowanie, miejsce w kolejności zadania), **Twoje własne pisaki, których używają warstwy** (dzięki temu projekt otwarty na innym komputerze przynosi je ze sobą; dołączają tam do listy **Moje pisaki**) oraz notatkę o ploterze, dla którego zrobiono projekt (nazwa i rozmiar arkusza); gdy otworzysz projekt dla urządzenia o innym arkuszu, pasek stanu o tym mówi. Warstwa z liczbą poza zakresem jest zastępowana warstwą domyślną.

Opis (karta **Opis**) jest częścią projektu; PenCreator zapisuje go też do G-code jako wiersze `(Note: …)`. Projekt, który ma plik, jest do niego zapisywany sam mniej więcej 20 sekund po zmianie (zobacz [Ustawienia i pliki](Ustawienia-i-pliki)).

Projekt zrobiony w LightCreatorze lub MillCreatorze otwiera się tu z rysunkiem, opisem i domyślnymi pisakami; przy zapisie ich sekcje wracają do pliku bez zmian. Plik z nowszym formatem kontenera (lub nowszą sekcją `pen`), niż zna program, jest odrzucany z komunikatem.

## Ochrona hasłem

Projekt można chronić hasłami: w **Ustawieniach projektu…** LightCreatora, MillCreatora i PenCreatora oraz w **Plik → Ustawienia strony…** BaseCreatora, w części **Ochrona**.

* **Hasło właściciela**: wszystko (edycja, kopiowanie, eksport, zmiana lub usunięcie ochrony).
* **Hasło otwarcia** (opcjonalne): projekt można otworzyć, oglądać, zamienić w G-code i uruchomić na maszynie, można też zmienić materiał i ustawienia warstw (prędkość, moc, przejścia, głębokość, frez, pisak), ale rysunku nie da się edytować, kopiować, eksportować jako rysunku (SVG, DXF, Illustrator, LightBurn …) ani drukować. Wbudowana strona WWW nie pokazuje wtedy rysunku.
* Gdy jest tylko hasło właściciela, nikt inny nie otworzy pliku. Bez hasła pliku nie da się w ogóle odczytać, a **zapomnianego hasła nie da się odzyskać**.
* Projekt otwarty hasłem otwarcia można odblokować hasłem właściciela bez ponownego otwierania: w części **Ochrona** ustawień (**Hasło właściciela**, **Odblokuj**) albo poleceniem **Odblokuj hasłem właściciela…** z menu jego karty (prawy przycisk). To menu ma też **Informacje o projekcie…** i ustawienia projektu.

Plik pozostaje tekstem JSON z kluczem `protected_cre`; projekt w nim jest zaszyfrowany **XChaCha20-Poly1305** losowym kluczem 256-bitowym, zapisanym raz dla każdego hasła i zaszyfrowanym kluczem zrobionym z hasła przez **Argon2id** (64 MiB, 3 przebiegi, własna sól). Każdy zapis ma nowy nonce; zmieniony plik się nie otworzy. Hasła muszą mieć co najmniej 8 znaków, a hasło otwarcia musi być inne niż hasło właściciela.

To, czego zabrania hasło otwarcia, egzekwują programy, tak jak hasło właściciela w PDF: kto je zna, może w zasadzie odszyfrować projekt innymi narzędziami. Hasło właściciela zachowaj dla siebie.

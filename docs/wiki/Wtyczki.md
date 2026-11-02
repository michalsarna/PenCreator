🌐 [English](Plugins) · **Polski**

# Wtyczki

Wtyczki dodają PenCreatorowi rzeczy bez zmieniania programu: **clipart, presety maszyn, tłumaczenia, importery innych formatów plików, generatory kształtów i postprocesory G-code.** Są wspólne dla LightCreatora, MillCreatora i PenCreatora: wtyczka leży w folderze `plugins` we wspólnym folderze danych (`baseCreator`), a każdy program używa tych części, które do niego pasują.

**Ustawienia → Wtyczki…** otwiera okno: instalacja wtyczki z pliku `.zip`, włączanie i wyłączanie, usuwanie, otwarcie folderu oraz ustawianie wtyczek z opcjami: każda wtyczka, w której jest coś do wyboru (**postprocesor**), ma własną zakładkę w prawym panelu, gdzie wybiera się ją dla G-code, który powstanie następny.

* **Importery:** **Plik → Importuj → Importuj wtyczką…**, przeciągnięcie pliku albo **Plik → Otwórz…** na pliku, którego rozszerzenie czyta jakaś wtyczka. Kształty są dodawane do rysunku (zaznaczone, jedno cofnięcie).
* **Generatory:** **Plik → Importuj → Generuj kształt…** (także **Wstawianie → Generuj kształt…**) pokazuje formularz generatora (liczby, przełączniki, teksty, listy) i dodaje kształty, które tworzy.
* **Postprocesory:** gdy jeden jest wybrany w zakładce jego wtyczki, **Eksportuj G-code…** (i przycisk **Zapisz G-code**) oraz **Podgląd G-code** przepuszczają program przez niego. Może zmieniać tylko wiersze G-code; wiersze będące poleceniami kontrolera (`$…`, `~`, `!`, `?`) albo ze znakami sterującymi są odrzucane i nic się nie zapisuje. Zadanie wysyłane prosto do maszyny nie przechodzi przez postprocesor.
* PenCreator bierze też presety maszyn klasy `pen` (na liście **Start od** przy dodawaniu urządzenia).

**Jak używać G-code tools (i każdego postprocesora), krok po kroku.** 1. **Ustawienia → Wtyczki…** → **Oficjalne wtyczki** → zainstaluj **G-code tools** (albo **Zainstaluj wtyczkę z pliku zip…**). 2. W prawym panelu, po pozostałych zakładkach, pojawia się zakładka **G-code tools** (gdy panel jest zwinięty: na wąskim pasku). 3. Otwórz ją i wybierz **Number the lines (N10, N20, …)** albo **Remove comments** (to teksty wtyczki, nie programu); **Brak** wyłącza postprocesor. 4. **Ploter → Eksportuj G-code…** (albo przycisk **Zapisz G-code**) zapisuje teraz przetworzony plik; pasek stanu mówi, którego postprocesora użyto. **Podgląd G-code** pokazuje przetworzony tekst. Zadanie uruchomione przyciskiem **Start** jest wysyłane bez zmian.

**Bezpieczeństwo.** Skrypty wtyczek (Rhai) działają w piaskownicy bez dostępu do plików, sieci, systemu i maszyny, z limitem czasu i pamięci. Mimo to wtyczka to czyjś kod, który decyduje, co usłyszy Twoja maszyna: instaluj tylko wtyczki, którym ufasz, i obejrzyj G-code nowego postprocesora, zanim go uruchomisz na maszynie.

## Oficjalne wtyczki

**Ustawienia → Wtyczki… → Oficjalne wtyczki** pokazuje oficjalną listę wtyczek, prowadzoną w repozytorium [baseCreator-plugins](https://github.com/michalsarna/baseCreator-plugins). Naciśnij **Sprawdź aktualizacje**, żeby ją wczytać: widać na niej *wszystkie* wtyczki, także te dla innego programu (oznaczone *Dla: …*). **Zainstaluj** pobiera wtyczkę z oficjalnej strony (sprawdzana jest suma kontrolna) do folderu wtyczek wspólnego dla wszystkich programów; **Zaktualizuj** pojawia się, gdy lista ma nowszą wersję (opis zmian po najechaniu). Wtyczka wymagająca nowszego programu niż Twój jest pokazana, ale nie do instalacji. Sprawdzanie aktualizacji przy starcie programu jest **włączone** (odznacz **Sprawdzaj aktualizacje wtyczek przy starcie**, żeby je wyłączyć): gdy jakieś wtyczki są nieaktualne, okno je wymienia i daje **Zaktualizuj wszystkie**, **Wtyczki** i **Później**. Nic nie instaluje się bez Twojego kliknięcia.

Własną wtyczkę napiszesz i wyślesz na listę według [przewodnika dla autorów](https://github.com/michalsarna/baseCreator-plugins/blob/main/docs/GUIDE.md) (po angielsku) z szablonami w repozytorium wtyczek; błędy i pomysły zgłaszaj w issues tamtego repozytorium.

Jak napisać wtyczkę (manifest, formularz, funkcje skryptu, przykłady): [PLUGINS.md w baseCreator](https://github.com/michalsarna/baseCreator/blob/main/docs/PLUGINS.md) (po angielsku).

Kilka postprocesorów może być włączonych naraz: zaznacz je w zakładkach wtyczek; działają **w kolejności zaznaczania** (wiersze jednego trafiają do następnego), a pasek stanu wymienia je połączone znakiem `+`. **Brak** wyłącza wszystkie.

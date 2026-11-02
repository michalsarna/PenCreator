🌐 [English](Home) · **Polski**

<p align="center"><img src="https://raw.githubusercontent.com/michalsarna/PenCreator/main/assets/icons/icon-256.png" alt="PenCreator" width="128"></p>

# PenCreator

Otwartoźródłowy program dla ploterów piszących, napisany w Rust. **Wczesna wersja:** nie był jeszcze sprawdzony na prawdziwym ploterze. Urządzenia, pisaki jako warstwy, wypełnianie liniami, podgląd wykresu i G-code dla GRBL; zapisuje G-code i może przesłać go do plotera GRBL. To rodzeństwo [LightCreatora](https://github.com/michalsarna/LightCreator) (laser) i [MillCreatora](https://github.com/michalsarna/MillCreator) (frezarka CNC): to samo okno, ten sam edytor i te same pliki dzięki [baseCreatorowi](https://github.com/michalsarna/baseCreator).

![Edytor](https://raw.githubusercontent.com/michalsarna/PenCreator/main/docs/screenshots/05-layers.png)

## Od czego zacząć

| Chcę… | Przejdź do |
|---|---|
| Zainstalować program | [Instalacja](Instalacja) |
| Zrobić pierwsze zadanie krok po kroku | [Szybki start](Szybki-start) |
| Ustawić warstwy, wybrać pisaki, zrozumieć zmiany pisaka | [Warstwy i pisaki](Warstwy-i-pisaki) |
| Znaleźć polecenie w menu | [Menu](Menu) |
| Poznać klawisze | [Skróty klawiszowe](Skroty-klawiszowe) |
| Dzielić projekt z LightCreatorem i MillCreatorem | [Plik projektu](Plik-projektu) |
| Edytować kształty, importować obrazy, karty i panele | [Narzędzia projektowe](Narzedzia-projektowe) |
| Sterować maszyną: przesuw, zerowanie, firmware, kalibracja | [Sterowanie maszyną](Sterowanie-maszyna) |
| Zobaczyć, jak to wygląda | [Zrzuty ekranu](Zrzuty-ekranu) |
| Znaleźć plik ustawień | [Ustawienia i pliki](Ustawienia-i-pliki) |
| Poczytać o kodzie | [Architektura](Architektura) |
| Wiedzieć, co jest, a czego nie ma | [Bezpieczeństwo i ograniczenia](Bezpieczenstwo-i-ograniczenia) |

## W skrócie

* **Edytor** z baseCreatora: kształty, edycja węzłów, tekst, grupowanie, wyrównanie, cofanie, siatka i przyciąganie, na obszarze roboczym urządzenia (A4 dla Dummy). Rysunki, podgląd, tekst G-code i kamera to karty, w najwyżej trzech panelach.
* **Import:** SVG, PDF / Illustrator, DXF, projekty LightBurna (`.lbrn2`), pliki UGS Designera (`.ugsd`); 310 clipartów z wyszukiwaniem w sieci.
* **Plik projektu:** `.cre`, wspólny z LightCreatorem i MillCreatorem; sekcje innych programów są zachowywane przy zapisie.
* **Interfejs:** 30 języków (także własne teksty PenCreatora) i 14 schematów kolorów.
* **Urządzenia i pisaki:** lista ploterów na ekranie startowym (**Nie mam urządzenia** tworzy Dummy), 64 warstwy, które są pisakami (kolor, szerokość, prędkość, kontur, wypełnienie liniami albo obraz rysowany liniami), biblioteka pisaków (**Ploter → Biblioteka pisaków…**), wykres zadania, G-code jako tekst w karcie i G-code dla GRBL.
* **Sterowanie maszyną:** przesuw, zerowanie, makra, ustawienia firmware, kalibracja, statystyki (zobacz [Sterowanie maszyną](Sterowanie-maszyna)).
* **Planowane:** HPGL, czcionki jednoliniowe.

Najnowsza wersja: **v0.5.3** · [Wydania](https://github.com/michalsarna/PenCreator/releases) · [Zgłoszenia](https://github.com/michalsarna/PenCreator/issues) · Licencja: [MIT](https://github.com/michalsarna/PenCreator/blob/main/LICENSE) lub [Apache-2.0](https://github.com/michalsarna/PenCreator/blob/main/LICENSE-APACHE), do wyboru

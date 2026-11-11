//! The quick guide of PenCreator: what is its own. The rest is the shared guide (`base_ui::guide`).
use base_ui::dialogs::Section;
use base_ui::guide::{Own, Picture};
use base_ui::i18n::{lang, Lang};

const EN: &[Section] = &[
    Section {
        title: "Getting started",
        lines: &[
            "The start screen lists your plotters. Click one, or press I have no device for a stand-in called Dummy (an A4 sheet). Add device… makes a new one: the work area, how the pen is lifted (the Z axis or a servo) and the connection. Then press New or Open….",
            "A first job: draw or import something, give the shapes their pens on the Layers tab, look at the plot, then save the G-code or send it to the plotter.",
        ],
    },
    Section {
        title: "Layers and pens",
        lines: &[
            "A layer is one pen. Select shapes and click a colour in the row under the work area to put them on that layer. On the Layers tab set how it is drawn (outline, fill with lines, fill and outline, picture), the pen, its width and the speed. The list is the order of the job, and a layer that is not ticked is not drawn.",
            "Plotter > Pen library… keeps your pens. When the next layer needs another pen, the job stops and a window says which one to put in.",
        ],
    },
    Section {
        title: "Plot and G-code",
        lines: &[
            "Plotter > Plot of the job… opens the preview: the lines in their ink, the moves with the pen up, an animation, the time and the warnings. Preview G-code shows the program as text in a tab, without saving anything. Plotter > Export G-code… writes the file.",
        ],
    },
    Section {
        title: "Running a job",
        lines: &[
            "Machine > Frame goes round the box of the job with the pen up. Machine > Start job sends the job; a change of pen stops it and asks for the pen.",
            "To draw round a cup or a bottle, switch on the Rotary page of Machine > Device settings…: the Y motor turns the work, and the work area shows a line at every way round it.",
        ],
    },
];

const PL: &[Section] = &[
    Section {
        title: "Pierwsze kroki",
        lines: &[
            "Ekran startowy pokazuje Twoje plotery. Kliknij jeden albo naciśnij Nie mam urządzenia, żeby użyć zastępczego urządzenia Dummy (arkusz A4). Dodaj urządzenie… tworzy nowe: obszar roboczy, sposób podnoszenia pisaka (oś Z albo serwo) i połączenie. Potem naciśnij Nowy albo Otwórz….",
            "Pierwsze zadanie: narysuj albo zaimportuj coś, daj kształtom pisaki na karcie Warstwy, obejrzyj wykres, a potem zapisz G-code albo wyślij go do plotera.",
        ],
    },
    Section {
        title: "Warstwy i pisaki",
        lines: &[
            "Warstwa to jeden pisak. Zaznacz kształty i kliknij kolor w rzędzie pod obszarem roboczym, żeby przenieść je na tę warstwę. Na karcie Warstwy ustaw sposób rysowania (obrys, wypełnienie liniami, wypełnienie i obrys, obraz), pisak, jego szerokość i prędkość. Lista to kolejność zadania, a warstwa bez zaznaczenia nie jest rysowana.",
            "Ploter > Biblioteka pisaków… przechowuje Twoje pisaki. Gdy następna warstwa potrzebuje innego pisaka, zadanie się zatrzymuje, a okno mówi, który włożyć.",
        ],
    },
    Section {
        title: "Wykres i G-code",
        lines: &[
            "Ploter > Wykres zadania… otwiera podgląd: linie w kolorach tuszu, ruchy z podniesionym pisakiem, animację, czas i ostrzeżenia. Podgląd G-code pokazuje program jako tekst na karcie, niczego nie zapisując. Ploter > Eksportuj G-code… zapisuje plik.",
        ],
    },
    Section {
        title: "Uruchamianie zadania",
        lines: &[
            "Maszyna > Obrys obchodzi prostokąt zadania z podniesionym pisakiem. Maszyna > Rozpocznij zadanie wysyła zadanie; zmiana pisaka je zatrzymuje i prosi o pisak.",
            "Aby rysować na kubku albo butelce, włącz stronę Obrotowy w Maszyna > Ustawienia urządzenia…: silnik Y obraca przedmiot, a obszar roboczy pokazuje linię co jeden obwód.",
        ],
    },
];

/// The guide in English or Polish.
pub fn sections_in(polish: bool) -> Vec<Section> {
    let first = if polish { PL } else { EN };
    base_ui::guide::sections(Own { first, last: &[], machine: true, picture: Picture::Camera }, polish)
}

/// The guide in the language of the interface.
pub fn sections() -> Vec<Section> {
    sections_in(lang() == Lang::Pl)
}

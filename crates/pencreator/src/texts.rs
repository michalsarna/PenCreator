//! The texts of PenCreator that the shared interface does not have: English, six languages in the table, and the other 23
//! in the files of `assets/lang`. The shared texts are in baseCreator; the application's own are asked first.
use base_ui::i18n::{self, Texts};
use std::sync::Once;

/// The texts of the other languages, in the order of `Lang::ALL` after the first seven. One line each:
/// `id|translation`, where the id is the line of the English text in `assets/lang/keys.txt`.
const EXTRA_FILES: [&str; 23] = [
    include_str!("../../../assets/lang/bg.txt"),
    include_str!("../../../assets/lang/ca.txt"),
    include_str!("../../../assets/lang/hr.txt"),
    include_str!("../../../assets/lang/cs.txt"),
    include_str!("../../../assets/lang/da.txt"),
    include_str!("../../../assets/lang/nl.txt"),
    include_str!("../../../assets/lang/et.txt"),
    include_str!("../../../assets/lang/fr.txt"),
    include_str!("../../../assets/lang/el.txt"),
    include_str!("../../../assets/lang/hu.txt"),
    include_str!("../../../assets/lang/is.txt"),
    include_str!("../../../assets/lang/ga.txt"),
    include_str!("../../../assets/lang/lv.txt"),
    include_str!("../../../assets/lang/lt.txt"),
    include_str!("../../../assets/lang/mt.txt"),
    include_str!("../../../assets/lang/nb.txt"),
    include_str!("../../../assets/lang/pt.txt"),
    include_str!("../../../assets/lang/ro.txt"),
    include_str!("../../../assets/lang/sk.txt"),
    include_str!("../../../assets/lang/sl.txt"),
    include_str!("../../../assets/lang/es.txt"),
    include_str!("../../../assets/lang/sv.txt"),
    include_str!("../../../assets/lang/uk.txt"),
];
const KEYS: &str = include_str!("../../../assets/lang/keys.txt");

static TEXTS: Texts = Texts::new(TABLE, KEYS, EXTRA_FILES);

/// Make the texts known to the translation layer (once).
pub fn register() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| i18n::register(&TEXTS));
}

// Columns: English, Polski, Deutsch, Italiano, Suomi, 中文, हिन्दी
#[rustfmt::skip]
const TABLE: &[[&str; 7]] = &[
    ["A layer is a pen: when the next layer has another pen, the G-code stops so that you can change it.", "Warstwa to pisak: gdy następna warstwa ma inny pisak, G-code zatrzymuje się, żeby go wymienić.", "Eine Ebene ist ein Stift: Hat die nächste Ebene einen anderen Stift, hält der G-Code an, damit du ihn wechseln kannst.", "Un livello è una penna: se il livello successivo ha un'altra penna, il G-code si ferma per farti cambiare penna.", "Taso on kynä: kun seuraavalla tasolla on toinen kynä, G-koodi pysähtyy, jotta voit vaihtaa sen.", "图层就是一支笔：下一图层使用另一支笔时，G-code 会暂停以便你换笔。", "लेयर एक पेन है: जब अगली लेयर में दूसरा पेन हो, तो G-code रुक जाता है ताकि आप उसे बदल सकें।"],
    ["A port name, or the USB vendor and product id (VID:PID, hexadecimal): the port that has these ids is used.", "Nazwa portu albo identyfikator USB producenta i produktu (VID:PID, szesnastkowo): używany jest port z tymi identyfikatorami.", "Ein Portname oder die USB-Hersteller- und Produkt-ID (VID:PID, hexadezimal): Es wird der Port mit diesen IDs verwendet.", "Il nome di una porta, oppure l'ID USB di produttore e prodotto (VID:PID, esadecimale): si usa la porta con questi ID.", "Portin nimi tai USB-valmistajan ja tuotteen tunnus (VID:PID, heksadesimaalina): käytetään porttia, jolla on nämä tunnukset.", "端口名称，或 USB 厂商与产品 ID（VID:PID，十六进制）：将使用具有这些 ID 的端口。", "पोर्ट का नाम, या USB विक्रेता और उत्पाद आईडी (VID:PID, हेक्साडेसिमल): इन आईडी वाला पोर्ट उपयोग होता है।"],
    ["A second set of fill lines across the first", "Drugi zestaw linii wypełnienia w poprzek pierwszego", "Ein zweiter Satz Füllinien quer zum ersten", "Un secondo gruppo di linee di riempimento incrociato al primo", "Toinen täyteviivojen joukko ensimmäisen poikki", "与第一组交叉的第二组填充线", "पहले के आर-पार भरने की रेखाओं का दूसरा सेट"],
    ["A usual sheet size…", "Zwykły rozmiar arkusza…", "Eine übliche Blattgröße…", "Una dimensione di foglio comune…", "Tavallinen arkin koko…", "常见的纸张尺寸…", "एक सामान्य कागज़ आकार…"],
    ["Abort the running job", "Przerwij wykonywaną pracę", "Laufenden Auftrag abbrechen", "Interrompi il lavoro in corso", "Keskeytä käynnissä oleva työ", "中止正在运行的作业", "चल रहे जॉब को रोकें"],
    ["About", "O programie", "Über", "Informazioni", "Tietoja", "关于", "के बारे में"],
    ["Add a camera", "Dodaj kamerę", "Kamera hinzufügen", "Aggiungi una telecamera", "Lisää kamera", "添加摄像头", "कैमरा जोड़ें"],
    ["Add a device", "Dodaj urządzenie", "Gerät hinzufügen", "Aggiungi un dispositivo", "Lisää laite", "添加设备", "डिवाइस जोड़ें"],
    ["Add device", "Dodaj urządzenie", "Gerät hinzufügen", "Aggiungi dispositivo", "Lisää laite", "添加设备", "डिवाइस जोड़ें"],
    ["Add device…", "Dodaj urządzenie…", "Gerät hinzufügen…", "Aggiungi dispositivo…", "Lisää laite…", "添加设备…", "डिवाइस जोड़ें…"],
    ["Address", "Adres", "Adresse", "Indirizzo", "Osoite", "地址", "पता"],
    ["After connecting", "Po połączeniu", "Nach dem Verbinden", "Dopo la connessione", "Yhdistämisen jälkeen", "连接后", "कनेक्ट होने के बाद"],
    ["Alarm {}", "Alarm {}", "Alarm {}", "Allarme {}", "Hälytys {}", "报警 {}", "अलार्म {}"],
    ["Arrange", "Rozmieść", "Anordnen", "Disponi", "Järjestä", "排列", "व्यवस्थित करें"],
    ["Back left", "Tył po lewej", "Hinten links", "Dietro a sinistra", "Takana vasemmalla", "左后", "पीछे बाएँ"],
    ["Back right", "Tył po prawej", "Hinten rechts", "Dietro a destra", "Takana oikealla", "右后", "पीछे दाएँ"],
    ["Back to the first view. Double click on the picture does the same.", "Powrót do pierwszego widoku. Podwójne kliknięcie obrazu robi to samo.", "Zurück zur ersten Ansicht. Ein Doppelklick auf das Bild tut dasselbe.", "Torna alla prima vista. Il doppio clic sull'immagine fa lo stesso.", "Takaisin ensimmäiseen näkymään. Kuvan kaksoisnapsautus tekee saman.", "回到最初的视图。双击图片效果相同。", "पहले दृश्य पर वापस। चित्र पर डबल क्लिक भी यही करता है।"],
    ["Baud rate", "Szybkość transmisji", "Baudrate", "Baud rate", "Siirtonopeus (baud)", "波特率", "बॉड दर"],
    ["Bind to this port", "Powiąż z tym portem", "An diesen Port binden", "Associa a questa porta", "Sido tähän porttiin", "绑定到此端口", "इस पोर्ट से बाँधें"],
    ["Bound USB device", "Powiązane urządzenie USB", "Gebundenes USB-Gerät", "Dispositivo USB associato", "Sidottu USB-laite", "已绑定的 USB 设备", "बँधा USB डिवाइस"],
    ["Camera", "Kamera", "Kamera", "Fotocamera", "Kamera", "摄像头", "कैमरा"],
    ["Camera name", "Nazwa kamery", "Kameraname", "Nome della telecamera", "Kameran nimi", "摄像头名称", "कैमरे का नाम"],
    ["Camera number", "Numer kamery", "Kameranummer", "Numero della telecamera", "Kameran numero", "摄像头编号", "कैमरा संख्या"],
    ["Camera of this computer", "Kamera tego komputera", "Kamera dieses Computers", "Telecamera di questo computer", "Tämän tietokoneen kamera", "本机摄像头", "इस कंप्यूटर का कैमरा"],
    ["Camera rotation", "Obrót kamery", "Kameradrehung", "Rotazione fotocamera", "Kameran kierto", "摄像头旋转", "कैमरा रोटेशन"],
    ["Camera view", "Widok z kamery", "Kameraansicht", "Vista fotocamera", "Kameran näkymä", "摄像头画面", "कैमरा दृश्य"],
    ["Cameras", "Kamery", "Kameras", "Telecamere", "Kamerat", "摄像头", "कैमरे"],
    ["Check the program", "Sprawdź program", "Programm prüfen", "Controlla il programma", "Tarkista ohjelma", "检查程序", "प्रोग्राम जाँचें"],
    ["Checking the program first…", "Najpierw sprawdzam program…", "Programm wird zuerst geprüft…", "Prima controllo il programma…", "Tarkistetaan ohjelma ensin…", "先检查程序…", "पहले प्रोग्राम जाँच रहे हैं…"],
    ["Checking the program…", "Sprawdzam program…", "Programm wird geprüft…", "Controllo del programma…", "Tarkistetaan ohjelmaa…", "正在检查程序…", "प्रोग्राम जाँच रहे हैं…"],
    ["Choose a port or a host in the device settings first.", "Najpierw wybierz port lub host w ustawieniach urządzenia.", "Wähle zuerst in den Geräteeinstellungen einen Port oder Host.", "Scegli prima una porta o un host nelle impostazioni del dispositivo.", "Valitse ensin portti tai isäntä laitteen asetuksissa.", "请先在设备设置中选择端口或主机。", "पहले डिवाइस सेटिंग्स में पोर्ट या होस्ट चुनें।"],
    ["Choose the machine you will work with.", "Wybierz maszynę, na której będziesz pracować.", "Wähle die Maschine, mit der du arbeiten willst.", "Scegli la macchina con cui lavorare.", "Valitse kone, jolla työskentelet.", "选择你要使用的机器。", "वह मशीन चुनें जिस पर आप काम करेंगे।"],
    ["Clear an alarm", "Wyczyść alarm", "Alarm löschen", "Cancella un allarme", "Kuittaa hälytys", "清除报警", "अलार्म साफ़ करें"],
    ["Clearing the alarm skips homing: use it only when the head is known to stand in a safe place.", "Wyczyszczenie alarmu pomija bazowanie: używaj tylko, gdy wiadomo, że głowica stoi w bezpiecznym miejscu.", "Das Löschen des Alarms überspringt das Referenzieren: nur verwenden, wenn der Kopf sicher an einem sicheren Ort steht.", "Cancellare l'allarme salta l'azzeramento: usalo solo quando la testa si trova di sicuro in un punto sicuro.", "Hälytyksen kuittaus ohittaa referoinnin: käytä vain, kun pään tiedetään olevan turvallisessa paikassa.", "清除报警会跳过回零：仅在确知机头位于安全位置时使用。", "अलार्म साफ़ करना होमिंग छोड़ देता है: केवल तब उपयोग करें जब हेड सुरक्षित जगह पर हो।"],
    ["Closed shapes are filled with parallel lines; holes stay empty.", "Zamknięte kształty są wypełniane równoległymi liniami; otwory zostają puste.", "Geschlossene Formen werden mit parallelen Linien gefüllt; Löcher bleiben leer.", "Le forme chiuse vengono riempite con linee parallele; i fori restano vuoti.", "Suljetut muodot täytetään yhdensuuntaisilla viivoilla; reiät jäävät tyhjiksi.", "闭合图形用平行线填充；孔保持空白。", "बंद आकार समानांतर रेखाओं से भरे जाते हैं; छेद खाली रहते हैं।"],
    ["Confirm delete", "Potwierdź usunięcie", "Löschen bestätigen", "Conferma eliminazione", "Vahvista poisto", "确认删除", "हटाने की पुष्टि करें"],
    ["Connect", "Połącz", "Verbinden", "Connetti", "Yhdistä", "连接", "कनेक्ट"],
    ["Connect again automatically", "Łącz ponownie automatycznie", "Automatisch erneut verbinden", "Riconnetti automaticamente", "Yhdistä uudelleen automaattisesti", "自动重新连接", "अपने आप दोबारा कनेक्ट करें"],
    ["Connected", "Połączono", "Verbunden", "Collegato", "Yhdistetty", "已连接", "कनेक्टेड"],
    ["Connection", "Połączenie", "Verbindung", "Connessione", "Yhteys", "连接", "कनेक्शन"],
    ["Connection type", "Typ połączenia", "Verbindungsart", "Tipo di connessione", "Yhteyden tyyppi", "连接类型", "कनेक्शन प्रकार"],
    ["Console", "Konsola", "Konsole", "Console", "Konsoli", "控制台", "कंसोल"],
    ["Control", "Sterowanie", "Steuerung", "Controllo", "Ohjaus", "控制", "नियंत्रण"],
    ["Copied to {}", "Skopiowano do {}", "Kopiert nach {}", "Copiato in {}", "Kopioitu: {}", "已复制为 {}", "{} में कॉपी किया"],
    ["Could not save: {}", "Nie udało się zapisać: {}", "Konnte nicht speichern: {}", "Impossibile salvare: {}", "Tallennus ei onnistunut: {}", "无法保存：{}", "सहेजा नहीं जा सका: {}"],
    ["Counting fills the controller's buffer and is fast. Simple sends one line and waits for its ok: slow, but for firmware that answers wrongly.", "Zliczanie wypełnia bufor sterownika i jest szybkie. Prosty wysyła jedną linię i czeka na ok: wolny, ale dla firmware odpowiadającego błędnie.", "Zählen füllt den Puffer der Steuerung und ist schnell. Einfach sendet eine Zeile und wartet auf ihr ok: langsam, aber für Firmware, die falsch antwortet.", "Il conteggio riempie il buffer del controller ed è veloce. Semplice invia una riga e attende il suo ok: lento, ma per firmware che risponde male.", "Laskenta täyttää ohjaimen puskurin ja on nopea. Yksinkertainen lähettää yhden rivin ja odottaa sen ok:n: hidas, mutta väärin vastaavalle laiteohjelmistolle.", "计数会填满控制器缓冲区，速度快。简单模式发送一行并等待其 ok：较慢，但适用于回答错误的固件。", "गिनती कंट्रोलर का बफ़र भरती है और तेज़ है। सरल एक पंक्ति भेजकर उसके ok की प्रतीक्षा करता है: धीमा, पर गलत जवाब देने वाले फ़र्मवेयर के लिए।"],
    ["Crossed lines", "Linie krzyżowe", "Gekreuzte Linien", "Linee incrociate", "Ristiviivat", "交叉线", "आड़ी-तिरछी रेखाएँ"],
    ["Delete this camera", "Usuń tę kamerę", "Diese Kamera löschen", "Elimina questa telecamera", "Poista tämä kamera", "删除此摄像头", "इस कैमरे को हटाएँ"],
    ["Device", "Urządzenie", "Gerät", "Dispositivo", "Laite", "设备", "डिवाइस"],
    ["Device settings", "Ustawienia urządzenia", "Geräteeinstellungen", "Impostazioni dispositivo", "Laitteen asetukset", "设备设置", "डिवाइस सेटिंग्स"],
    ["Device settings…", "Ustawienia urządzenia…", "Geräteeinstellungen…", "Impostazioni dispositivo…", "Laitteen asetukset…", "设备设置…", "डिवाइस सेटिंग्स…"],
    ["Diagnostics and statistics…", "Diagnostyka i statystyki…", "Diagnose und Statistik…", "Diagnostica e statistiche…", "Diagnostiikka ja tilastot…", "诊断和统计…", "निदान और आँकड़े…"],
    ["Disconnect", "Rozłącz", "Trennen", "Disconnetti", "Katkaise yhteys", "断开连接", "डिस्कनेक्ट"],
    ["Drawing", "Rysowanie", "Zeichnen", "Disegno", "Piirtäminen", "绘制", "ड्रॉइंग"],
    ["Drawn times", "Ile razy rysować", "Wie oft gezeichnet", "Quante volte disegnare", "Piirtokertoja", "绘制次数", "कितनी बार खींचा जाए"],
    ["Drawn: taken into the plot and the G-code", "Rysowana: wchodzi do wykresu i do G-code", "Gezeichnet: wird in den Plot und den G-Code übernommen", "Disegnato: incluso nel tracciato e nel G-code", "Piirretään: mukaan piirtoon ja G-koodiin", "绘制：计入绘图和 G-code", "खींचा जाता है: प्लॉट और G-code में शामिल"],
    ["Edit", "Edycja", "Bearbeiten", "Modifica", "Muokkaa", "编辑", "संपादन"],
    ["Edit…", "Edytuj…", "Bearbeiten…", "Modifica…", "Muokkaa…", "编辑…", "संपादित करें…"],
    ["Error {} in line {}: {}", "Błąd {} w wierszu {}: {}", "Fehler {} in Zeile {}: {}", "Errore {} alla riga {}: {}", "Virhe {} rivillä {}: {}", "第 {} 行出现错误 {}：{}", "पंक्ति {} में त्रुटि {}: {}"],
    ["Error {}: {}", "Błąd {}: {}", "Fehler {}: {}", "Errore {}: {}", "Virhe {}: {}", "错误 {}：{}", "त्रुटि {}: {}"],
    ["Export G-code…", "Eksportuj G-code…", "G-Code exportieren…", "Esporta G-code…", "Vie G-koodi…", "导出 G-code…", "G-code निर्यात करें…"],
    ["Export…", "Eksportuj…", "Exportieren…", "Esporta…", "Vie…", "导出…", "निर्यात…"],
    ["Fill and outline", "Wypełnienie i kontur", "Füllung und Kontur", "Riempimento e contorno", "Täyttö ja ääriviiva", "填充和轮廓", "भराव और रूपरेखा"],
    ["Fill line angle", "Kąt linii wypełnienia", "Winkel der Füllinien", "Angolo delle linee di riempimento", "Täyteviivojen kulma", "填充线角度", "भराव रेखाओं का कोण"],
    ["Fill line gap", "Odstęp linii wypełnienia", "Abstand der Füllinien", "Distanza delle linee di riempimento", "Täyteviivojen väli", "填充线间距", "भराव रेखाओं के बीच की दूरी"],
    ["Fill with lines", "Wypełnienie liniami", "Mit Linien füllen", "Riempi con linee", "Täytä viivoilla", "用线填充", "रेखाओं से भरें"],
    ["Firmware settings ($$)…", "Ustawienia firmware ($$)…", "Firmware-Einstellungen ($$)…", "Impostazioni del firmware ($$)…", "Laiteohjelmiston asetukset ($$)…", "固件设置（$$）…", "फ़र्मवेयर सेटिंग ($$)…"],
    ["Frame", "Obrys", "Rahmen", "Cornice", "Kehys", "走边框", "फ़्रेम"],
    ["Framing the work area", "Obrys obszaru pracy", "Arbeitsbereich abfahren", "Contorno dell'area di lavoro", "Työalueen kehystys", "给工作区域画框", "कार्य क्षेत्र का फ़्रेम"],
    ["From line…", "Od wiersza…", "Ab Zeile…", "Dalla riga…", "Riviltä…", "从行…", "पंक्ति से…"],
    ["Front left", "Przód po lewej", "Vorne links", "Davanti a sinistra", "Edessä vasemmalla", "左前", "आगे बाएँ"],
    ["Front right", "Przód po prawej", "Vorne rechts", "Davanti a destra", "Edessä oikealla", "右前", "आगे दाएँ"],
    ["General", "Ogólne", "Allgemein", "Generale", "Yleiset", "常规", "सामान्य"],
    ["Go round the box where the work will be done", "Obejdź obrys miejsca, gdzie będzie wykonywana praca", "Fahre den Rahmen ab, in dem gearbeitet wird", "Percorri il riquadro dove verrà eseguito il lavoro", "Kierrä laatikko, jossa työ tehdään", "沿着将要加工的区域外框走一圈", "उस बॉक्स का चक्कर लगाएँ जहाँ काम होगा"],
    ["Help", "Pomoc", "Hilfe", "Aiuto", "Ohje", "帮助", "सहायता"],
    ["Home", "Bazuj", "Referenzfahrt", "Home", "Kotiutus", "回零", "होम"],
    ["Home the machine", "Bazuj maszynę", "Maschine referenzieren", "Azzera la macchina", "Referoi kone", "机器回零", "मशीन होम करें"],
    ["Host", "Host", "Host", "Host", "Isäntä", "主机", "होस्ट"],
    ["How many times every line is drawn: for a pen that is too light", "Ile razy rysowana jest każda linia: dla zbyt jasnego pisaka", "Wie oft jede Linie gezeichnet wird: für einen zu hellen Stift", "Quante volte viene disegnata ogni linea: per una penna troppo chiara", "Kuinka monta kertaa jokainen viiva piirretään: liian vaaleaa kynää varten", "每条线绘制的次数：用于颜色太浅的笔", "हर रेखा कितनी बार खींची जाए: बहुत हल्के पेन के लिए"],
    ["I have no device", "Nie mam urządzenia", "Ich habe kein Gerät", "Non ho un dispositivo", "Minulla ei ole laitetta", "我没有设备", "मेरे पास कोई डिवाइस नहीं है"],
    ["Import LightBurn project…", "Importuj projekt LightBurn…", "LightBurn-Projekt importieren…", "Importa progetto LightBurn…", "Tuo LightBurn-projekti…", "导入 LightBurn 项目…", "LightBurn प्रोजेक्ट आयात करें…"],
    ["Imported the device {}", "Zaimportowano urządzenie {}", "Gerät {} importiert", "Dispositivo {} importato", "Laite {} tuotu", "已导入设备 {}", "डिवाइस {} आयात किया गया"],
    ["Import…", "Importuj…", "Importieren…", "Importa…", "Tuo…", "导入…", "आयात…"],
    ["Landscape", "Poziomo", "Querformat", "Orizzontale", "Vaaka", "横向", "लैंडस्केप"],
    ["Layers", "Warstwy", "Ebenen", "Livelli", "Tasot", "图层", "लेयर"],
    ["Lines: {} m, moves with the pen up: {} m, pen lifts: {}, about {} min", "Linie: {} m, ruchy z podniesionym pisakiem: {} m, podniesień pisaka: {}, około {} min", "Linien: {} m, Bewegungen mit angehobenem Stift: {} m, Stifthübe: {}, etwa {} min", "Linee: {} m, movimenti con la penna alzata: {} m, sollevamenti della penna: {}, circa {} min", "Viivat: {} m, liikkeet kynä ylhäällä: {} m, kynän nostot: {}, noin {} min", "线条：{} 米，抬笔移动：{} 米，抬笔次数：{}，约 {} 分钟", "रेखाएँ: {} मी, पेन उठाकर चालें: {} मी, पेन उठाना: {}, लगभग {} मिनट"],
    ["Machine zero (0,0)", "Zero maszyny (0,0)", "Maschinennullpunkt (0,0)", "Zero macchina (0,0)", "Koneen nollapiste (0,0)", "机器原点 (0,0)", "मशीन शून्य (0,0)"],
    ["Moves with the pen up", "Ruchy z podniesionym pisakiem", "Bewegungen mit angehobenem Stift", "Movimenti con la penna alzata", "Liikkeet kynä ylhäällä", "抬笔移动", "पेन उठाकर चालें"],
    ["New", "Nowy", "Neu", "Nuovo", "Uusi", "新建", "नया"],
    ["No device yet. Add one, or work without a machine.", "Nie ma jeszcze urządzenia. Dodaj je albo pracuj bez maszyny.", "Noch kein Gerät. Füge eines hinzu oder arbeite ohne Maschine.", "Ancora nessun dispositivo. Aggiungine uno o lavora senza macchina.", "Ei vielä laitetta. Lisää sellainen tai työskentele ilman konetta.", "还没有设备。添加一个，或不用机器工作。", "अभी कोई डिवाइस नहीं। एक जोड़ें, या बिना मशीन के काम करें।"],
    ["No port", "Brak portu", "Kein Port", "Nessuna porta", "Ei porttia", "无端口", "कोई पोर्ट नहीं"],
    ["Not an http or https address.", "To nie jest adres http ani https.", "Keine http- oder https-Adresse.", "Non è un indirizzo http o https.", "Ei ole http- tai https-osoite.", "不是 http 或 https 地址。", "यह http या https पता नहीं है।"],
    ["Notes", "Notatki", "Notizen", "Note", "Muistiinpanot", "笔记", "नोट"],
    ["Notes about this machine", "Notatki o tej maszynie", "Notizen zu dieser Maschine", "Note su questa macchina", "Muistiinpanoja tästä koneesta", "关于这台机器的备注", "इस मशीन के बारे में नोट्स"],
    ["Nothing selected", "Nic nie zaznaczono", "Nichts ausgewählt", "Nessuna selezione", "Ei valintaa", "未选择任何内容", "कुछ चयनित नहीं"],
    ["Open a drawing or start an empty design.", "Otwórz rysunek albo zacznij pusty projekt.", "Öffne eine Zeichnung oder beginne mit einem leeren Entwurf.", "Apri un disegno o inizia un progetto vuoto.", "Avaa piirros tai aloita tyhjä suunnitelma.", "打开一幅图，或开始一个空白设计。", "कोई ड्रॉइंग खोलें या खाली डिज़ाइन शुरू करें।"],
    ["Open the link to the controller set in the device settings", "Otwórz łącze do sterownika ustawionego w ustawieniach urządzenia", "Die Verbindung zur Steuerung aus den Geräteeinstellungen öffnen", "Apri il collegamento al controller impostato nelle impostazioni del dispositivo", "Avaa yhteys laitteen asetuksissa määritettyyn ohjaimeen", "打开设备设置中所设控制器的连接", "डिवाइस सेटिंग्स में तय कंट्रोलर से लिंक खोलें"],
    ["Open the preview window", "Otwórz okno podglądu", "Vorschaufenster öffnen", "Apri la finestra di anteprima", "Avaa esikatseluikkuna", "打开预览窗口", "पूर्वावलोकन विंडो खोलें"],
    ["Open…", "Otwórz…", "Öffnen…", "Apri…", "Avaa…", "打开…", "खोलें…"],
    ["Outline", "Kontur", "Kontur", "Contorno", "Ääriviiva", "轮廓", "रूपरेखा"],
    ["Paper", "Papier", "Papier", "Carta", "Paperi", "纸", "कागज़"],
    ["Pen", "Pisak", "Stift", "Penna", "Kynä", "笔", "पेन"],
    ["Pen down", "Pisak opuszczony", "Stift unten", "Penna abbassata", "Kynä alhaalla", "落笔", "पेन नीचे"],
    ["Pen up", "Pisak podniesiony", "Stift oben", "Penna alzata", "Kynä ylhäällä", "抬笔", "पेन ऊपर"],
    ["Pen width", "Szerokość pisaka", "Stiftbreite", "Larghezza della penna", "Kynän leveys", "笔宽", "पेन की चौड़ाई"],
    ["Plot of the job…", "Wykres zadania…", "Plot des Auftrags…", "Tracciato del lavoro…", "Työn piirtokuva…", "作业绘图…", "कार्य का प्लॉट…"],
    ["Plotter", "Ploter", "Plotter", "Plotter", "Piirturi", "绘图仪", "प्लॉटर"],
    ["Port", "Port", "Port", "Porta", "Portti", "端口", "पोर्ट"],
    ["Port or USB id", "Port lub identyfikator USB", "Port oder USB-ID", "Porta o ID USB", "Portti tai USB-tunnus", "端口或 USB ID", "पोर्ट या USB आईडी"],
    ["Portrait", "Pionowo", "Hochformat", "Verticale", "Pysty", "纵向", "पोर्ट्रेट"],
    ["Preview", "Podgląd", "Vorschau", "Anteprima", "Esikatselu", "预览", "पूर्वावलोकन"],
    ["Preview…", "Podgląd…", "Vorschau…", "Anteprima…", "Esikatselu…", "预览…", "पूर्वावलोकन…"],
    ["Properties", "Właściwości", "Eigenschaften", "Proprietà", "Ominaisuudet", "属性", "गुण"],
    ["Put the selected shapes on this layer", "Umieść zaznaczone kształty na tej warstwie", "Die ausgewählten Formen auf diese Ebene legen", "Metti le forme selezionate su questo livello", "Siirrä valitut muodot tälle tasolle", "将所选图形放到此图层", "चुने हुए आकार इस लेयर पर रखें"],
    ["Read a device from a file", "Wczytaj urządzenie z pliku", "Ein Gerät aus einer Datei lesen", "Leggi un dispositivo da un file", "Lue laite tiedostosta", "从文件读取设备", "फ़ाइल से डिवाइस पढ़ें"],
    ["Ready", "Gotowe", "Bereit", "Pronto", "Valmis", "就绪", "तैयार"],
    ["Reconnect", "Ponowne łączenie", "Neu verbinden", "Riconnessione", "Uudelleenyhdistys", "重新连接", "दोबारा कनेक्ट"],
    ["Refresh", "Odśwież", "Aktualisieren", "Aggiorna", "Päivitä", "刷新", "रीफ़्रेश"],
    ["Remember the USB ids of the chosen port: the port is found again by them when its name changes.", "Zapamiętaj identyfikatory USB wybranego portu: gdy zmieni się jego nazwa, port zostanie znaleziony po nich.", "Die USB-IDs des gewählten Ports merken: Ändert sich sein Name, wird er über sie wiedergefunden.", "Ricorda gli ID USB della porta scelta: se il suo nome cambia, la porta viene ritrovata tramite essi.", "Muista valitun portin USB-tunnukset: portti löytyy niillä, kun sen nimi muuttuu.", "记住所选端口的 USB ID：端口名称改变时可凭它们重新找到。", "चुने पोर्ट की USB आईडी याद रखें: नाम बदलने पर पोर्ट उनसे फिर मिल जाता है।"],
    ["Reset view", "Resetuj widok", "Ansicht zurücksetzen", "Reimposta la vista", "Palauta näkymä", "重置视图", "दृश्य रीसेट करें"],
    ["Resume", "Wznów", "Fortsetzen", "Riprendi", "Jatka", "继续", "फिर शुरू करें"],
    ["STOP", "STOP", "STOPP", "STOP", "PYSÄYTÄ", "急停", "रुकें"],
    ["Save", "Zapisz", "Speichern", "Salva", "Tallenna", "保存", "सहेजें"],
    ["Save G-code", "Zapisz G-code", "G-Code speichern", "Salva G-code", "Tallenna G-koodi", "保存 G-code", "G-code सहेजें"],
    ["Save as…", "Zapisz jako…", "Speichern unter…", "Salva come…", "Tallenna nimellä…", "另存为…", "इस रूप में सहेजें…"],
    ["Save this device to a file", "Zapisz to urządzenie do pliku", "Dieses Gerät in einer Datei speichern", "Salva questo dispositivo in un file", "Tallenna tämä laite tiedostoon", "将此设备保存到文件", "इस डिवाइस को फ़ाइल में सहेजें"],
    ["Saved the device to {}", "Zapisano urządzenie do {}", "Gerät in {} gespeichert", "Dispositivo salvato in {}", "Laite tallennettu: {}", "设备已保存到 {}", "डिवाइस {} में सहेजा गया"],
    ["Saved {}", "Zapisano {}", "{} gespeichert", "Salvato {}", "Tallennettu {}", "已保存 {}", "{} सहेजा गया"],
    ["Servo (M3 S…)", "Serwo (M3 S…)", "Servo (M3 S…)", "Servo (M3 S…)", "Servo (M3 S…)", "舵机 (M3 S…)", "सर्वो (M3 S…)"],
    ["Settings", "Ustawienia", "Einstellungen", "Impostazioni", "Asetukset", "设置", "सेटिंग्स"],
    ["Shapes on this layer", "Kształty na tej warstwie", "Formen auf dieser Ebene", "Forme su questo livello", "Muodot tällä tasolla", "此图层上的图形", "इस लेयर पर आकार"],
    ["Show the live camera picture of this machine", "Pokaż obraz na żywo z kamery tej maszyny", "Live-Kamerabild dieser Maschine anzeigen", "Mostra l'immagine dal vivo della fotocamera di questa macchina", "Näytä tämän koneen kameran suora kuva", "显示此机器的实时摄像头画面", "इस मशीन का लाइव कैमरा चित्र दिखाएँ"],
    ["Show the toolpaths on the work area", "Pokaż ścieżki na obszarze roboczym", "Werkzeugwege im Arbeitsbereich anzeigen", "Mostra i percorsi sull'area di lavoro", "Näytä työstöradat työalueella", "在工作区域显示刀路", "कार्य क्षेत्र पर टूलपाथ दिखाएँ"],
    ["Shown in the Camera tab", "Pokazywana w zakładce Kamera", "Im Reiter Kamera gezeigt", "Mostrata nella scheda Telecamera", "Näytetään Kamera-välilehdellä", "显示在“摄像头”标签页", "कैमरा टैब में दिखाया गया"],
    ["Shown in the work area", "Pokazana w obszarze roboczym", "Im Arbeitsbereich angezeigt", "Mostrato nell'area di lavoro", "Näytetään työalueella", "显示在工作区域", "कार्य क्षेत्र में दिखाया गया"],
    ["Speed", "Prędkość", "Geschwindigkeit", "Velocità", "Nopeus", "速度", "गति"],
    ["Speed of moves with the pen up", "Prędkość ruchów z podniesionym pisakiem", "Geschwindigkeit der Bewegungen mit angehobenem Stift", "Velocità dei movimenti con la penna alzata", "Liikkeiden nopeus kynä ylhäällä", "抬笔移动的速度", "पेन उठाकर चालों की गति"],
    ["Start", "Start", "Start", "Avvia", "Käynnistä", "开始", "शुरू"],
    ["Start from", "Start od", "Start von", "Parti da", "Aloituspiste", "起点", "यहाँ से शुरू करें"],
    ["Start the job at a line of the program", "Zacznij zadanie od wiersza programu", "Den Auftrag bei einer Zeile des Programms starten", "Avvia il lavoro da una riga del programma", "Aloita työ ohjelman riviltä", "从程序的某一行开始作业", "प्रोग्राम की एक पंक्ति से कार्य शुरू करें"],
    ["Starting job: {} lines, ~{}", "Start zadania: {} linii, ok. {}", "Auftrag startet: {} Zeilen, ca. {}", "Avvio lavoro: {} righe, ~{}", "Työn aloitus: {} riviä, ~{}", "开始任务：{} 行，约 {}", "जॉब शुरू: {} पंक्तियाँ, ~{}"],
    ["Stream the job to the connected machine", "Wyślij zadanie do podłączonej maszyny", "Den Auftrag an die verbundene Maschine senden", "Invia il lavoro alla macchina collegata", "Lähetä työ yhdistettyyn koneeseen", "把作业发送到已连接的机器", "जॉब को कनेक्टेड मशीन पर भेजें"],
    ["Streaming", "Strumieniowanie", "Streaming", "Streaming", "Suoratoisto", "串流", "स्ट्रीमिंग"],
    ["Switched on", "Włączona", "Eingeschaltet", "Attiva", "Päällä", "已开启", "चालू"],
    ["TCP port", "Port TCP", "TCP-Port", "Porta TCP", "TCP-portti", "TCP 端口", "TCP पोर्ट"],
    ["The G-code around the job: {}", "G-code wokół zadania: {}", "Der G-Code rund um den Auftrag: {}", "Il G-code attorno al lavoro: {}", "Työn ympärillä oleva G-koodi: {}", "作业前后的 G 代码：{}", "कार्य के आसपास का G-कोड: {}"],
    ["The check found nothing wrong.", "Sprawdzanie nie znalazło nic złego.", "Die Prüfung hat nichts Falsches gefunden.", "Il controllo non ha trovato nulla di sbagliato.", "Tarkistus ei löytänyt mitään virheellistä.", "检查没有发现问题。", "जाँच में कुछ ग़लत नहीं मिला।"],
    ["The check found nothing. The job is running.", "Sprawdzanie nic nie znalazło. Zadanie trwa.", "Die Prüfung hat nichts gefunden. Der Auftrag läuft.", "Il controllo non ha trovato nulla. Il lavoro è in esecuzione.", "Tarkistus ei löytänyt mitään. Työ on käynnissä.", "检查没有发现问题。作业正在运行。", "जाँच में कुछ नहीं मिला। कार्य चल रहा है।"],
    ["The check found {} problems, the first in line {}. The job was not started.", "Sprawdzanie znalazło {} problemów, pierwszy w wierszu {}. Zadania nie uruchomiono.", "Die Prüfung hat {} Probleme gefunden, das erste in Zeile {}. Der Auftrag wurde nicht gestartet.", "Il controllo ha trovato {} problemi, il primo alla riga {}. Il lavoro non è stato avviato.", "Tarkistus löysi {} ongelmaa, ensimmäisen riviltä {}. Työtä ei aloitettu.", "检查发现 {} 个问题，第一个在第 {} 行。作业未启动。", "जाँच में {} समस्याएँ मिलीं, पहली पंक्ति {} में। कार्य शुरू नहीं हुआ।"],
    ["The device of this project", "Urządzenie tego projektu", "Das Gerät dieses Projekts", "Il dispositivo di questo progetto", "Tämän projektin laite", "此项目的设备", "इस प्रोजेक्ट का डिवाइस"],
    ["The device was added.", "Dodano urządzenie.", "Das Gerät wurde hinzugefügt.", "Il dispositivo è stato aggiunto.", "Laite lisättiin.", "设备已添加。", "डिवाइस जोड़ा गया।"],
    ["The device was saved.", "Zapisano urządzenie.", "Das Gerät wurde gespeichert.", "Il dispositivo è stato salvato.", "Laite tallennettiin.", "设备已保存。", "डिवाइस सहेजा गया।"],
    ["The fill first, then the outline over it, so the edge is clean.", "Najpierw wypełnienie, potem kontur na wierzchu, żeby krawędź była czysta.", "Erst die Füllung, dann die Kontur darüber, damit die Kante sauber ist.", "Prima il riempimento, poi il contorno sopra, così il bordo è pulito.", "Ensin täyttö, sitten ääriviiva päälle, jotta reuna on siisti.", "先填充，再在上面画轮廓，使边缘干净。", "पहले भराव, फिर उसके ऊपर रूपरेखा, ताकि किनारा साफ़ रहे।"],
    ["The job is finished, with {} problems (see the Console).", "Zadanie zakończone, z {} problemami (patrz Konsola).", "Der Auftrag ist beendet, mit {} Problemen (siehe Konsole).", "Il lavoro è terminato, con {} problemi (vedi la Console).", "Työ on valmis, {} ongelmalla (katso Konsoli).", "作业已完成，有 {} 个问题（见控制台）。", "कार्य पूरा हुआ, {} समस्याओं के साथ (कंसोल देखें)।"],
    ["The job is finished.", "Zadanie zakończone.", "Der Auftrag ist beendet.", "Il lavoro è terminato.", "Työ on valmis.", "作业已完成。", "जॉब पूरा हुआ।"],
    ["The job is paused: {}", "Zadanie wstrzymane: {}", "Der Auftrag ist angehalten: {}", "Il lavoro è in pausa: {}", "Työ on keskeytetty: {}", "作业已暂停：{}", "कार्य रुका हुआ है: {}"],
    ["The job is running.", "Zadanie trwa.", "Der Auftrag läuft.", "Il lavoro è in corso.", "Työ on käynnissä.", "作业正在运行。", "जॉब चल रहा है।"],
    ["The pen follows the lines of the shapes. Open lines work too.", "Pisak podąża za liniami kształtów. Otwarte linie też działają.", "Der Stift folgt den Linien der Formen. Offene Linien funktionieren auch.", "La penna segue le linee delle forme. Funzionano anche le linee aperte.", "Kynä seuraa muotojen viivoja. Avoimet viivat toimivat myös.", "笔沿图形的线走。开放线也可以。", "पेन आकारों की रेखाओं का अनुसरण करता है। खुली रेखाएँ भी काम करती हैं।"],
    ["The pen is lifted by", "Pisak podnosi się za pomocą", "Der Stift wird angehoben durch", "La penna viene alzata da", "Kynää nostaa", "抬笔方式", "पेन उठाया जाता है"],
    ["The work area is the sheet the pen can reach. Machine zero is the corner where the pen starts.", "Obszar roboczy to arkusz, do którego sięga pisak. Zero maszyny to narożnik, w którym pisak zaczyna.", "Der Arbeitsbereich ist das Blatt, das der Stift erreicht. Der Maschinennullpunkt ist die Ecke, in der der Stift startet.", "L'area di lavoro è il foglio che la penna raggiunge. Lo zero macchina è l'angolo da cui parte la penna.", "Työalue on arkki, jonka kynä tavoittaa. Koneen nollapiste on kulma, josta kynä aloittaa.", "工作区域是笔能够到达的纸张。机器零点是笔开始的角。", "कार्य क्षेत्र वह कागज़ है जहाँ तक पेन पहुँचता है। मशीन का शून्य वह कोना है जहाँ से पेन शुरू होता है।"],
    ["There is nothing to draw: put shapes on layers that are switched on.", "Nie ma czego narysować: połóż kształty na włączonych warstwach.", "Es gibt nichts zu zeichnen: Lege Formen auf eingeschaltete Ebenen.", "Non c'è nulla da disegnare: metti le forme su livelli attivi.", "Ei ole mitään piirrettävää: laita muodot päällä oleville tasoille.", "没有可绘制的内容：请把图形放到已开启的图层上。", "खींचने के लिए कुछ नहीं है: आकार चालू लेयर पर रखें।"],
    ["There is nothing to frame: put shapes on layers that are switched on.", "Nie ma czego obrysować: połóż kształty na włączonych warstwach.", "Nichts zum Abfahren: Lege Formen auf eingeschaltete Ebenen.", "Non c'è nulla da contornare: metti le forme su livelli attivi.", "Ei ole mitään kehystettävää: laita muodot päällä oleville tasoille.", "没有可描边的内容：请把图形放到已开启的图层上。", "फ़्रेम करने के लिए कुछ नहीं: आकार चालू लेयर पर रखें।"],
    ["There is nothing to show yet: draw shapes and put them on layers that are switched on.", "Nie ma jeszcze czego pokazać: narysuj kształty i połóż je na włączonych warstwach.", "Es gibt noch nichts zu zeigen: Zeichne Formen und lege sie auf eingeschaltete Ebenen.", "Non c'è ancora nulla da mostrare: disegna delle forme e mettile su livelli attivi.", "Ei ole vielä mitään näytettävää: piirrä muotoja ja laita ne päällä oleville tasoille.", "还没有可显示的内容：请绘制图形并放到已开启的图层上。", "अभी दिखाने के लिए कुछ नहीं है: आकार बनाएँ और उन्हें चालू लेयर पर रखें।"],
    ["This build cannot use the cameras of the computer.", "Ta wersja programu nie może używać kamer komputera.", "Diese Version kann die Kameras des Computers nicht verwenden.", "Questa versione non può usare le telecamere del computer.", "Tämä versio ei voi käyttää tietokoneen kameroita.", "此版本不能使用电脑的摄像头。", "यह संस्करण कंप्यूटर के कैमरों का उपयोग नहीं कर सकता।"],
    ["This device has no camera that is switched on. Add one in the device settings.", "To urządzenie nie ma włączonej kamery. Dodaj ją w ustawieniach urządzenia.", "Dieses Gerät hat keine eingeschaltete Kamera. Füge in den Geräteeinstellungen eine hinzu.", "Questo dispositivo non ha una telecamera attiva. Aggiungila nelle impostazioni del dispositivo.", "Tällä laitteella ei ole päällä olevaa kameraa. Lisää sellainen laitteen asetuksissa.", "此设备没有开启的摄像头。请在设备设置中添加。", "इस डिवाइस में कोई चालू कैमरा नहीं है। डिवाइस सेटिंग्स में एक जोड़ें।"],
    ["This device is not in the list: it came with the project", "Tego urządzenia nie ma na liście: przyszło z projektem", "Dieses Gerät ist nicht in der Liste: Es kam mit dem Projekt", "Questo dispositivo non è nell'elenco: è arrivato con il progetto", "Tämä laite ei ole luettelossa: se tuli projektin mukana", "此设备不在列表中：它随项目而来", "यह डिवाइस सूची में नहीं है: यह प्रोजेक्ट के साथ आया"],
    ["This is the stand-in device (Dummy): the default work area and speeds.", "To urządzenie zastępcze (Dummy): domyślny obszar roboczy i prędkości.", "Das ist das Ersatzgerät (Dummy): Standard-Arbeitsbereich und -Geschwindigkeiten.", "Questo è il dispositivo sostitutivo (Dummy): area di lavoro e velocità predefinite.", "Tämä on varalaite (Dummy): oletustyöalue ja -nopeudet.", "这是替代设备（Dummy）：默认的工作区域和速度。", "यह स्थानापन्न डिवाइस (Dummy) है: डिफ़ॉल्ट कार्य क्षेत्र और गतियाँ।"],
    ["Time {} / {}", "Czas {} / {}", "Zeit {} / {}", "Tempo {} / {}", "Aika {} / {}", "时间 {} / {}", "समय {} / {}"],
    ["Too many devices", "Zbyt wiele urządzeń", "Zu viele Geräte", "Troppi dispositivi", "Liian monta laitetta", "设备太多", "बहुत अधिक डिवाइस"],
    ["Toolpaths", "Ścieżki", "Werkzeugwege", "Percorsi", "Työstöradat", "刀路", "टूलपाथ"],
    ["Unbind", "Rozłącz powiązanie", "Bindung lösen", "Dissocia", "Pura sidonta", "解除绑定", "बंधन हटाएँ"],
    ["Units", "Jednostki", "Einheiten", "Unità", "Yksiköt", "单位", "इकाइयाँ"],
    ["View", "Widok", "Ansicht", "Vista", "Näytä", "视图", "दृश्य"],
    ["View options…", "Opcje widoku…", "Ansichtsoptionen…", "Opzioni di visualizzazione…", "Näkymän asetukset…", "视图选项…", "दृश्य विकल्प…"],
    ["Wait after the pen moves", "Czekanie po ruchu pisaka", "Wartezeit nach der Stiftbewegung", "Attesa dopo il movimento della penna", "Odotus kynän liikkeen jälkeen", "笔移动后的等待时间", "पेन चलने के बाद प्रतीक्षा"],
    ["Waiting for the picture…", "Oczekiwanie na obraz…", "Warte auf das Bild…", "In attesa dell'immagine…", "Odotetaan kuvaa…", "正在等待画面…", "चित्र की प्रतीक्षा…"],
    ["When the link drops by itself, try again every few seconds until you press Disconnect.", "Gdy połączenie samo się zerwie, próbuj co kilka sekund, aż naciśniesz Rozłącz.", "Reißt die Verbindung von selbst ab, alle paar Sekunden erneut versuchen, bis du auf Trennen drückst.", "Se il collegamento cade da solo, riprova ogni pochi secondi finché non premi Disconnetti.", "Kun yhteys katkeaa itsestään, yritä uudelleen muutaman sekunnin välein, kunnes painat Katkaise.", "连接自行断开时，每隔几秒重试，直到你点击“断开”。", "लिंक अपने आप टूटे तो हर कुछ सेकंड में फिर कोशिश करें, जब तक आप डिस्कनेक्ट न दबाएँ।"],
    ["With the Z axis the pen goes to the heights above; with a servo the G-code switches the spindle output on with the S value (M3 S…).", "Przy osi Z pisak jedzie na podane wysokości; przy serwie G-code włącza wyjście wrzeciona z wartością S (M3 S…).", "Bei der Z-Achse fährt der Stift auf die obigen Höhen; bei einem Servo schaltet der G-Code den Spindelausgang mit dem S-Wert ein (M3 S…).", "Con l'asse Z la penna va alle altezze indicate; con un servo il G-code attiva l'uscita del mandrino con il valore S (M3 S…).", "Z-akselilla kynä menee yllä oleville korkeuksille; servolla G-koodi kytkee karan lähdön päälle S-arvolla (M3 S…).", "使用 Z 轴时，笔会移到上面的高度；使用舵机时，G-code 会用 S 值打开主轴输出 (M3 S…)。", "Z अक्ष से पेन ऊपर दी गई ऊँचाइयों पर जाता है; सर्वो के साथ G-code स्पिंडल आउटपुट को S मान से चालू करता है (M3 S…)।"],
    ["Work on a project without a machine: a plain device called Dummy, with the default work area and speeds", "Pracuj nad projektem bez maszyny: zwykłe urządzenie o nazwie Dummy z domyślnym polem roboczym i prędkościami", "An einem Projekt ohne Maschine arbeiten: ein einfaches Gerät namens Dummy mit der Standard-Arbeitsfläche und den Standardgeschwindigkeiten", "Lavora a un progetto senza macchina: un dispositivo semplice chiamato Dummy, con l'area di lavoro e le velocità predefinite", "Työskentele projektin parissa ilman konetta: tavallinen laite nimeltä Dummy oletustyöalueella ja oletusnopeuksilla", "无需机器即可处理项目：一台名为 Dummy 的普通设备，使用默认工作区域和速度", "मशीन के बिना प्रोजेक्ट पर काम करें: Dummy नाम का एक सामान्य डिवाइस, डिफ़ॉल्ट कार्य क्षेत्र और गति के साथ"],
    ["Z axis", "Oś Z", "Z-Achse", "Asse Z", "Z-akseli", "Z 轴", "Z अक्ष"],
    ["e.g. My pen plotter", "np. Mój ploter", "z. B. Mein Plotter", "es. Il mio plotter", "esim. Oma piirturi", "例如：我的绘图仪", "उदा. मेरा प्लॉटर"],
    ["none", "brak", "keins", "nessuno", "ei mitään", "无", "कोई नहीं"],
    ["part of the drawing lies outside the work area", "część rysunku leży poza obszarem roboczym", "ein Teil der Zeichnung liegt außerhalb des Arbeitsbereichs", "una parte del disegno è fuori dall'area di lavoro", "osa piirroksesta on työalueen ulkopuolella", "部分图形超出了工作区域", "ड्रॉइंग का कुछ हिस्सा कार्य क्षेत्र से बाहर है"],
    ["the fill has too many lines: the gap between them is too small, so part of the fill was left out", "wypełnienie ma zbyt wiele linii: odstęp między nimi jest za mały, więc część wypełnienia pominięto", "die Füllung hat zu viele Linien: Der Abstand ist zu klein, deshalb wurde ein Teil der Füllung ausgelassen", "il riempimento ha troppe linee: la distanza tra loro è troppo piccola, quindi una parte del riempimento è stata esclusa", "täytössä on liian monta viivaa: väli on liian pieni, joten osa täytöstä jätettiin pois", "填充的线太多：线间距太小，所以部分填充被省略", "भराव में बहुत अधिक रेखाएँ हैं: उनके बीच की दूरी बहुत कम है, इसलिए भराव का कुछ हिस्सा छोड़ दिया गया"],
    ["this project was made for a different machine: {}", "ten projekt powstał dla innej maszyny: {}", "dieses Projekt wurde für eine andere Maschine erstellt: {}", "questo progetto è stato creato per un'altra macchina: {}", "tämä projekti on tehty toiselle koneelle: {}", "此项目是为另一台机器制作的：{}", "यह प्रोजेक्ट किसी दूसरी मशीन के लिए बनाया गया था: {}"],
    ["{} picture(s) on the layers cannot be drawn: trace them to lines first", "Obrazów na warstwach ({}) nie można narysować: najpierw zamień je na linie", "{} Bild(er) auf den Ebenen können nicht gezeichnet werden: Wandle sie zuerst in Linien um", "{} immagine/i sui livelli non si possono disegnare: vettorializzale prima", "{} kuvaa tasoilla ei voi piirtää: jäljitä ne ensin viivoiksi", "图层上有 {} 张图片无法绘制：请先将它们描摹成线条", "लेयर पर {} चित्र नहीं खींचे जा सकते: पहले उन्हें रेखाओं में बदलें"],
    ["Import UGS design…", "Importuj projekt UGS…", "UGS-Entwurf importieren…", "Importa progetto UGS…", "Tuo UGS-suunnitelma…", "导入 UGS 设计…", "UGS डिज़ाइन आयात करें…"],
    ["Picture", "Obraz", "Bild", "Immagine", "Kuva", "图片", "चित्र"],
    ["A picture on the layer is drawn with lines: dithered rows, dots, or crossed hatching by tone.", "Obraz na warstwie jest rysowany liniami: rastrowane wiersze, kropki albo kreskowanie krzyżowe według tonu.", "Ein Bild auf der Ebene wird mit Linien gezeichnet: gerasterte Zeilen, Punkte oder Kreuzschraffur nach Tonwert.", "Un'immagine sul livello viene disegnata con linee: righe retinate, punti o tratteggio incrociato in base al tono.", "Tason kuva piirretään viivoilla: rasteroidut rivit, pisteet tai ristikkäisviivoitus sävyn mukaan.", "图层上的图片用线条绘制：抖动行、点，或按色调的交叉排线。", "लेयर पर चित्र रेखाओं से बनाया जाता है: डिथर की गई पंक्तियाँ, बिंदु, या रंग-स्वर के अनुसार क्रॉस हैचिंग।"],
    ["Dithered rows", "Rastrowane wiersze", "Gerasterte Zeilen", "Righe retinate", "Rasteroidut rivit", "抖动行", "डिथर की गई पंक्तियाँ"],
    ["Dots", "Kropki", "Punkte", "Punti", "Pisteet", "点", "बिंदु"],
    ["Hatching by tone", "Kreskowanie według tonu", "Schraffur nach Tonwert", "Tratteggio per tono", "Viivoitus sävyn mukaan", "按色调排线", "स्वर के अनुसार हैचिंग"],
    ["Each row is drawn as lines where the dithered picture is dark. The dithering method is chosen below.", "Każdy wiersz jest rysowany liniami tam, gdzie rastrowany obraz jest ciemny. Metodę rastrowania wybierasz niżej.", "Jede Zeile wird dort mit Linien gezeichnet, wo das gerasterte Bild dunkel ist. Das Rasterverfahren wählst du unten.", "Ogni riga viene disegnata con linee dove l'immagine retinata è scura. Il metodo di retinatura si sceglie sotto.", "Jokainen rivi piirretään viivoilla siellä, missä rasteroitu kuva on tumma. Rasterointitavan valitset alla.", "抖动后的图片较暗处，每一行用线条绘制。抖动方法在下方选择。", "हर पंक्ति वहाँ रेखाओं से बनती है जहाँ डिथर किया चित्र गहरा है। डिथरिंग विधि नीचे चुनें।"],
    ["A dot for every dark cell of the dithered picture.", "Kropka dla każdej ciemnej komórki rastrowanego obrazu.", "Ein Punkt für jede dunkle Zelle des gerasterten Bildes.", "Un punto per ogni cella scura dell'immagine retinata.", "Piste jokaiselle rasteroidun kuvan tummalle solulle.", "抖动后图片的每个暗单元画一个点。", "डिथर किए चित्र के हर गहरे सेल के लिए एक बिंदु।"],
    ["Crossed hatching: the darker the picture, the more directions of lines.", "Kreskowanie krzyżowe: im ciemniejszy obraz, tym więcej kierunków linii.", "Kreuzschraffur: je dunkler das Bild, desto mehr Linienrichtungen.", "Tratteggio incrociato: più l'immagine è scura, più direzioni di linee.", "Ristikkäisviivoitus: mitä tummempi kuva, sitä useampia viivasuuntia.", "交叉排线：图片越暗，线条方向越多。", "क्रॉस हैचिंग: चित्र जितना गहरा, रेखाओं की उतनी अधिक दिशाएँ।"],
    ["Picture method", "Sposób rysowania obrazu", "Bildmethode", "Metodo dell'immagine", "Kuvan piirtotapa", "图片方式", "चित्र की विधि"],
    ["Dithering", "Rastrowanie", "Rasterung", "Retinatura", "Rasterointi", "抖动", "डिथरिंग"],
    ["Pen library", "Biblioteka pisaków", "Stiftbibliothek", "Libreria di penne", "Kynäkirjasto", "笔库", "पेन लाइब्रेरी"],
    ["Pen library…", "Biblioteka pisaków…", "Stiftbibliothek…", "Libreria di penne…", "Kynäkirjasto…", "笔库…", "पेन लाइब्रेरी…"],
    ["Ready-made pens", "Gotowe pisaki", "Fertige Stifte", "Penne pronte", "Valmiit kynät", "预设笔", "तैयार पेन"],
    ["My pens", "Moje pisaki", "Meine Stifte", "Le mie penne", "Omat kynät", "我的笔", "मेरे पेन"],
    ["Nothing here yet", "Na razie nic tu nie ma", "Noch nichts vorhanden", "Ancora niente qui", "Täällä ei ole vielä mitään", "这里还没有内容", "यहाँ अभी कुछ नहीं है"],
    ["New pen", "Nowy pisak", "Neuer Stift", "Nuova penna", "Uusi kynä", "新建笔", "नया पेन"],
    ["A copy among your own pens", "Kopia wśród twoich pisaków", "Eine Kopie unter deinen eigenen Stiften", "Una copia tra le tue penne", "Kopio omiin kyniisi", "复制到你自己的笔中", "आपके अपने पेनों में एक प्रति"],
    ["Read pens from a file", "Wczytaj pisaki z pliku", "Stifte aus einer Datei lesen", "Leggi le penne da un file", "Lue kynät tiedostosta", "从文件读取笔", "फ़ाइल से पेन पढ़ें"],
    ["The file is too large", "Plik jest zbyt duży", "Die Datei ist zu groß", "Il file è troppo grande", "Tiedosto on liian suuri", "文件太大", "फ़ाइल बहुत बड़ी है"],
    ["pens added", "dodanych pisaków", "Stifte hinzugefügt", "penne aggiunte", "kynää lisätty", "支笔已添加", "पेन जोड़े गए"],
    ["Left out", "Pominięte", "Ausgelassen", "Escluse", "Jätetty pois", "已略过", "छोड़े गए"],
    ["Cannot use the file", "Nie można użyć pliku", "Die Datei kann nicht verwendet werden", "Impossibile usare il file", "Tiedostoa ei voi käyttää", "无法使用该文件", "फ़ाइल का उपयोग नहीं हो सकता"],
    ["Save your pens to a file", "Zapisz swoje pisaki do pliku", "Deine Stifte in einer Datei speichern", "Salva le tue penne in un file", "Tallenna omat kynäsi tiedostoon", "将你的笔保存到文件", "अपने पेन फ़ाइल में सहेजें"],
    ["Saved", "Zapisano", "Gespeichert", "Salvato", "Tallennettu", "已保存", "सहेजा गया"],
    ["Could not save", "Nie udało się zapisać", "Speichern nicht möglich", "Impossibile salvare", "Tallennus ei onnistunut", "无法保存", "सहेजा नहीं जा सका"],
    ["Export this pen…", "Eksportuj ten pisak…", "Diesen Stift exportieren…", "Esporta questa penna…", "Vie tämä kynä…", "导出此笔…", "इस पेन को निर्यात करें…"],
    ["Choose a pen", "Wybierz pisak", "Wähle einen Stift", "Scegli una penna", "Valitse kynä", "选择一支笔", "एक पेन चुनें"],
    ["A ready-made pen cannot be changed. Make a copy to change it.", "Gotowego pisaka nie można zmieniać. Zrób kopię, żeby go zmienić.", "Ein fertiger Stift kann nicht geändert werden. Erstelle eine Kopie, um ihn zu ändern.", "Una penna pronta non si può modificare. Fai una copia per modificarla.", "Valmista kynää ei voi muuttaa. Tee siitä kopio, jos haluat muuttaa.", "预设笔无法修改。请先复制再修改。", "तैयार पेन बदला नहीं जा सकता। बदलने के लिए उसकी प्रति बनाएँ।"],
    ["Type", "Rodzaj", "Art", "Tipo", "Tyyppi", "类型", "प्रकार"],
    ["Width of the mark", "Szerokość śladu", "Breite der Spur", "Larghezza del segno", "Jäljen leveys", "笔迹宽度", "निशान की चौड़ाई"],
    ["Height of the mark", "Wysokość śladu", "Höhe der Spur", "Altezza del segno", "Jäljen korkeus", "笔迹高度", "निशान की ऊँचाई"],
    ["Turn of the tip", "Obrót końcówki", "Drehung der Spitze", "Rotazione della punta", "Kärjen kierto", "笔尖旋转", "नोक का घुमाव"],
    ["Ends of a line", "Końce linii", "Linienenden", "Estremità della linea", "Viivan päät", "线条末端", "रेखा के सिरे"],
    ["Round ends", "Zaokrąglone końce", "Runde Enden", "Estremità arrotondate", "Pyöreät päät", "圆头", "गोल सिरे"],
    ["Flat ends", "Płaskie końce", "Flache Enden", "Estremità piatte", "Suorat päät", "平头", "सपाट सिरे"],
    ["Ink colour", "Kolor tuszu", "Tintenfarbe", "Colore dell'inchiostro", "Musteen väri", "墨水颜色", "स्याही का रंग"],
    ["Opacity", "Krycie", "Deckkraft", "Opacità", "Peittävyys", "不透明度", "अपारदर्शिता"],
    ["No pen chosen", "Nie wybrano pisaka", "Kein Stift gewählt", "Nessuna penna scelta", "Kynää ei valittu", "未选择笔", "कोई पेन नहीं चुना"],
    ["Pencil", "Ołówek", "Bleistift", "Matita", "Lyijykynä", "铅笔", "पेंसिल"],
    ["Fountain pen (nib)", "Piórko (stalówka)", "Füller (Feder)", "Penna stilografica (pennino)", "Täytekynä (kärki)", "钢笔（笔尖）", "फ़ाउंटेन पेन (निब)"],
    ["Fineliner", "Cienkopis", "Fineliner", "Penna a punta fine", "Fineliner-kynä", "针管笔", "फ़ाइनलाइनर"],
    ["Felt-tip marker", "Mazak", "Filzstift", "Pennarello", "Huopakynä", "马克笔", "फ़ेल्ट-टिप मार्कर"],
    ["Highlighter", "Zakreślacz", "Textmarker", "Evidenziatore", "Korostuskynä", "荧光笔", "हाइलाइटर"],
    ["Brush pen", "Pisak pędzelkowy", "Pinselstift", "Pennarello a pennello", "Sivellinkynä", "毛笔", "ब्रश पेन"],
    ["Fit to the work area", "Dopasuj do obszaru roboczego", "An den Arbeitsbereich anpassen", "Adatta all'area di lavoro", "Sovita työalueeseen", "适应工作区域", "कार्य क्षेत्र में फ़िट करें"],
    ["Fit to the elements", "Dopasuj do elementów", "An die Elemente anpassen", "Adatta agli elementi", "Sovita elementteihin", "适应元素", "तत्वों में फ़िट करें"],
    ["Zoom so that the whole sheet is seen", "Powiększ tak, by widać było cały arkusz", "So zoomen, dass das ganze Blatt zu sehen ist", "Ingrandisci in modo da vedere tutto il foglio", "Zoomaa niin, että koko arkki näkyy", "缩放到能看到整张纸", "इस तरह ज़ूम करें कि पूरी शीट दिखे"],
    ["Zoom so that all the lines fill the window", "Powiększ tak, by wszystkie linie wypełniły okno", "So zoomen, dass alle Linien das Fenster füllen", "Ingrandisci in modo che tutte le linee riempiano la finestra", "Zoomaa niin, että kaikki viivat täyttävät ikkunan", "缩放到所有线条填满窗口", "इस तरह ज़ूम करें कि सभी रेखाएँ विंडो भर दें"],
    ["Raise the pen", "Podnieś pisak", "Stift anheben", "Alza la penna", "Nosta kynä", "抬起笔", "पेन उठाएँ"],
    ["Lower the pen", "Opuść pisak", "Stift absenken", "Abbassa la penna", "Laske kynä", "放下笔", "पेन नीचे करें"],
    ["One element at a time", "Element po elemencie", "Ein Element nach dem anderen", "Un elemento alla volta", "Yksi elementti kerrallaan", "逐个元素进行", "एक-एक तत्व"],
    ["Each element (a group, or a shape that is in none) is drawn completely, all its layers and all their passes, before the next one; the nearest element goes first. Off: layer after layer, pass after pass.", "Każdy element (grupa albo kształt bez grupy) jest rysowany w całości, ze wszystkimi warstwami i przejściami, zanim zacznie się następny; pierwszy jest najbliższy. Wyłączone: warstwa po warstwie, przejście po przejściu.", "Jedes Element (eine Gruppe oder eine Form ohne Gruppe) wird ganz gezeichnet, mit allen Ebenen und allen Durchgängen, bevor das nächste kommt; das nächstgelegene zuerst. Aus: Ebene für Ebene, Durchgang für Durchgang.", "Ogni elemento (un gruppo o una forma senza gruppo) viene disegnato per intero, con tutti i livelli e tutti i passaggi, prima del successivo; il più vicino per primo. Disattivato: livello dopo livello, passaggio dopo passaggio.", "Jokainen elementti (ryhmä tai muoto ilman ryhmää) piirretään kokonaan kaikkine tasoineen ja kierroksineen ennen seuraavaa; lähin ensin. Pois: taso kerrallaan, kierros kerrallaan.", "每个元素（一个组，或不在任何组中的图形）先完整绘制其所有图层和所有道次，再进行下一个；最近的元素优先。关闭时：逐图层、逐道次进行。", "हर तत्व (एक समूह, या बिना समूह वाला आकार) अपनी सभी परतों और पासों के साथ पूरा खींचा जाता है, फिर अगला; सबसे नज़दीकी पहले। बंद: परत-दर-परत, पास-दर-पास।"],
    ["Selection only", "Tylko zaznaczenie", "Nur Auswahl", "Solo selezione", "Vain valinta", "仅选中项", "केवल चयन"],
    ["Only the shapes that are selected on the work area when the G-code is saved or the job is started are drawn.", "Rysowane są tylko kształty zaznaczone na obszarze roboczym w chwili zapisu G-code lub startu zadania.", "Nur die Formen, die beim Speichern des G-Codes oder beim Start des Auftrags im Arbeitsbereich ausgewählt sind, werden gezeichnet.", "Vengono disegnate solo le forme selezionate nell'area di lavoro quando si salva il G-code o si avvia il lavoro.", "Piirretään vain työalueella valitut muodot, kun G-koodi tallennetaan tai työ aloitetaan.", "仅绘制在保存 G 代码或开始任务时于工作区中被选中的图形。", "केवल वे आकार खींचे जाते हैं जो G-कोड सहेजते या कार्य शुरू करते समय कार्य क्षेत्र में चुने हों।"],
    ["There is nothing to draw: nothing is selected", "Nie ma czego narysować: nic nie jest zaznaczone", "Nichts zu zeichnen: nichts ist ausgewählt", "Niente da disegnare: nulla è selezionato", "Ei piirrettävää: mitään ei ole valittu", "无可绘制内容：未选中任何内容", "खींचने के लिए कुछ नहीं: कुछ भी चयनित नहीं है"],
    ["Change the pen at a fixed place", "Zmiana pisaka w stałym miejscu", "Stift an einer festen Stelle wechseln", "Cambia la penna in un punto fisso", "Vaihda kynä kiinteässä paikassa", "在固定位置更换笔", "निश्चित स्थान पर पेन बदलें"],
    ["When the next layer needs another pen, the pen goes up, the plotter drives to this place (in the coordinates of the work area, which it may lie outside of), runs the G-code below and stops there (M0) so that the pen can be changed, by hand or by a mechanism. Off: the job stops where it is.", "Gdy następna warstwa wymaga innego pisaka, pisak się unosi, ploter jedzie w to miejsce (we współrzędnych obszaru roboczego, może leżeć poza nim), wykonuje poniższy G-code i zatrzymuje się tam (M0), żeby można było zmienić pisak ręcznie lub mechanizmem. Wyłączone: zadanie zatrzymuje się tam, gdzie jest.", "Wenn die nächste Ebene einen anderen Stift braucht, hebt sich der Stift, der Plotter fährt an diese Stelle (in Koordinaten des Arbeitsbereichs, sie darf außerhalb liegen), führt den G-Code unten aus und hält dort an (M0), damit der Stift von Hand oder durch einen Mechanismus gewechselt werden kann. Aus: Der Auftrag hält dort an, wo er ist.", "Quando il livello successivo richiede un'altra penna, la penna si alza, il plotter va in questo punto (nelle coordinate dell'area di lavoro, può trovarsi fuori), esegue il G-code qui sotto e si ferma lì (M0) perché la penna possa essere cambiata, a mano o da un meccanismo. Disattivato: il lavoro si ferma dove si trova.", "Kun seuraava taso tarvitsee toisen kynän, kynä nousee, piirturi ajaa tähän paikkaan (työalueen koordinaateissa, paikka voi olla sen ulkopuolella), suorittaa alla olevan G-koodin ja pysähtyy siihen (M0), jotta kynä voidaan vaihtaa käsin tai mekanismilla. Pois: työ pysähtyy siihen, missä se on.", "当下一图层需要另一支笔时，笔抬起，绘图仪移动到此位置（使用工作区坐标，可以在工作区之外），运行下面的 G 代码并在此停止（M0），以便手动或由机构更换笔。关闭时：任务就地停止。", "जब अगली परत को दूसरा पेन चाहिए, पेन ऊपर उठता है, प्लॉटर इस स्थान पर जाता है (कार्य क्षेत्र के निर्देशांक में, यह उसके बाहर हो सकता है), नीचे का G-कोड चलाता है और वहाँ रुकता है (M0) ताकि पेन हाथ से या तंत्र से बदला जा सके। बंद: कार्य वहीं रुकता है जहाँ वह है।"],
    ["Stop there (M0)", "Zatrzymaj tam (M0)", "Dort anhalten (M0)", "Fermati lì (M0)", "Pysähdy siihen (M0)", "在此停止（M0）", "वहाँ रुकें (M0)"],
    ["G-code at the place", "G-code w tym miejscu", "G-Code an der Stelle", "G-code nel punto", "G-koodi paikassa", "此位置的 G 代码", "उस स्थान पर G-कोड"],
    ["Lines sent when the plotter has arrived, before the stop: for a mechanism that changes the pen by itself. With the stop switched off the job goes on right after them.", "Linie wysyłane, gdy ploter dojedzie, przed zatrzymaniem: dla mechanizmu, który sam zmienia pisak. Przy wyłączonym zatrzymaniu zadanie idzie dalej zaraz po nich.", "Zeilen, die gesendet werden, wenn der Plotter angekommen ist, vor dem Anhalten: für einen Mechanismus, der den Stift selbst wechselt. Bei ausgeschaltetem Anhalten geht der Auftrag direkt danach weiter.", "Righe inviate quando il plotter è arrivato, prima dell'arresto: per un meccanismo che cambia la penna da solo. Con l'arresto disattivato il lavoro prosegue subito dopo.", "Rivit, jotka lähetetään piirturin saavuttua perille ennen pysähdystä: mekanismille, joka vaihtaa kynän itse. Kun pysähdys on pois, työ jatkuu heti niiden jälkeen.", "绘图仪到达后、停止前发送的行：用于自行更换笔的机构。关闭停止时，任务紧接着继续。", "प्लॉटर के पहुँचने पर, रुकने से पहले भेजी जाने वाली पंक्तियाँ: उस तंत्र के लिए जो पेन खुद बदलता है। रुकना बंद होने पर कार्य उनके तुरंत बाद आगे बढ़ता है।"],
    ["Import DXF…", "Importuj DXF…", "DXF importieren…", "Importa DXF…", "Tuo DXF…", "导入 DXF…", "DXF आयात करें…"],
    ["Insert", "Wstawianie", "Einfügen", "Inserisci", "Lisää", "插入", "सम्मिलित करें"],
    ["The view has three panes at most", "Widok ma najwyżej trzy panele", "Die Ansicht hat höchstens drei Bereiche", "La vista ha al massimo tre riquadri", "Näkymässä on enintään kolme paneelia", "视图最多有三个窗格", "दृश्य में अधिकतम तीन पेन हो सकते हैं"],
    ["Show the preview here", "Pokaż tu podgląd", "Vorschau hier anzeigen", "Mostra qui l'anteprima", "Näytä esikatselu tässä", "在此显示预览", "यहाँ पूर्वावलोकन दिखाएं"],
    ["No G-code file is open: File, View G-code file…", "Żaden plik G-code nie jest otwarty: File, View G-code file…", "Keine G-Code-Datei geöffnet: File, View G-code file…", "Nessun file G-code aperto: File, View G-code file…", "G-koodi-tiedostoa ei ole avattu: File, View G-code file…", "没有打开 G 代码文件：File, View G-code file…", "कोई G-कोड फ़ाइल खुली नहीं है: File, View G-code file…"],
    ["Preview G-code", "Podgląd G-code", "G-Code-Vorschau", "Anteprima G-code", "G-koodin esikatselu", "预览 G-code", "G-code पूर्वावलोकन"],
    ["Show the G-code without saving it", "Pokaż G-code bez zapisywania", "G-Code anzeigen, ohne zu speichern", "Mostra il G-code senza salvarlo", "Näytä G-koodi tallentamatta", "显示 G-code 但不保存", "G-code को सहेजे बिना दिखाएं"],
    ["preview", "podgląd", "Vorschau", "anteprima", "esikatselu", "预览", "पूर्वावलोकन"],
    ["The G-code is shown, nothing is saved", "G-code jest pokazany, nic nie zostało zapisane", "Der G-Code wird angezeigt, nichts wird gespeichert", "Il G-code è mostrato, nulla viene salvato", "G-koodi näytetään, mitään ei tallenneta", "已显示 G-code，未保存任何内容", "G-code दिखाया गया है, कुछ भी सहेजा नहीं गया"],
    ["The own G-code of the device: {}", "Własny G-code urządzenia: {}", "Eigener G-Code des Geräts: {}", "G-code personalizzato del dispositivo: {}", "Laitteen oma G-koodi: {}", "设备的自定义 G-code：{}", "डिवाइस का अपना G-code: {}"],
    ["In the G-code of the pen change also:", "W G-code zmiany pióra także:", "Im G-Code des Stiftwechsels außerdem:", "Nel G-code del cambio penna anche:", "Kynän vaihdon G-koodissa myös:", "在换笔的 G-code 中还可用：", "पेन बदलने के G-code में ये भी:"],
    ["Own G-code is added to every program, one command per line (at most 20 lines).", "Własny G-code jest dodawany do każdego programu, jedno polecenie w wierszu (najwyżej 20 wierszy).", "Eigener G-Code wird jedem Programm hinzugefügt, ein Befehl pro Zeile (höchstens 20 Zeilen).", "Il G-code personale viene aggiunto a ogni programma, un comando per riga (al massimo 20 righe).", "Oma G-koodi lisätään jokaiseen ohjelmaan, yksi komento riviä kohti (enintään 20 riviä).", "自定义 G 代码会添加到每个程序中，每行一条命令（最多 20 行）。", "अपना G-कोड हर प्रोग्राम में जोड़ा जाता है, एक पंक्ति में एक कमांड (अधिकतम 20 पंक्तियाँ)।"],
];

#[cfg(test)]
mod tests {
    use super::*;
    use base_ui::i18n::{Lang, TABLE_LANGS};

    #[test]
    fn every_row_is_complete_and_keeps_placeholders() {
        let mut seen = std::collections::HashSet::new();
        for row in TABLE {
            assert!(seen.insert(row[0]), "duplicate key {:?}", row[0]);
            let n = row[0].matches("{}").count();
            for (i, t) in row.iter().enumerate() {
                assert!(!t.is_empty(), "empty translation {i} for {:?}", row[0]);
                assert_eq!(t.matches("{}").count(), n, "placeholder mismatch in {:?} (column {i})", row[0]);
            }
        }
    }

    /// `assets/lang/keys.txt` lists the English texts of `TABLE` in order; the other languages refer to it by id.
    #[test]
    fn keys_file_matches_the_table() {
        let keys: Vec<&str> = KEYS.lines().filter_map(|l| l.split_once('\t')).map(|(_, t)| t).collect();
        assert_eq!(keys.len(), TABLE.len());
        for (i, (k, row)) in keys.iter().zip(TABLE).enumerate() {
            assert_eq!(*k, row[0], "line {i} of keys.txt");
        }
    }

    #[test]
    fn every_language_file_is_complete_and_keeps_placeholders() {
        let english: Vec<&str> = KEYS.lines().filter_map(|l| l.split_once('\t')).map(|(_, t)| t).collect();
        for (n, file) in EXTRA_FILES.iter().enumerate() {
            let code = Lang::ALL[TABLE_LANGS + n].code();
            let mut seen = std::collections::HashSet::new();
            for line in file.lines().filter(|l| !l.is_empty()) {
                let (id, text) = line.split_once('|').unwrap_or_else(|| panic!("{code}: no separator in {line:?}"));
                let id: usize = id.parse().unwrap_or_else(|_| panic!("{code}: bad id in {line:?}"));
                assert!(seen.insert(id), "{code}: duplicate id {id}");
                let key = english.get(id).unwrap_or_else(|| panic!("{code}: unknown id {id}"));
                assert!(!text.trim().is_empty(), "{code}: empty text for {key:?}");
                assert_eq!(text.matches("{}").count(), key.matches("{}").count(), "{code}: placeholders in {key:?}");
            }
            assert_eq!(seen.len(), english.len(), "{code}: every text needs a translation");
        }
    }

    #[test]
    fn the_texts_translate() {
        let (pl, fr) = (Lang::Pl.index() as usize, Lang::Fr.index() as usize);
        let first = TABLE[0][0];
        assert_ne!(TEXTS.lookup(pl, first), None);
        assert_ne!(TEXTS.lookup(fr, first), None);
        assert_eq!(TEXTS.lookup(0, first), None, "English needs no lookup");
    }

    /// Every text that the code of this application passes to `tr`, `trf` or `Warn::new` is in the table or in the
    /// shared texts, so no language is left with an English text by mistake.
    #[test]
    fn every_text_in_the_code_has_a_translation() {
        let own: std::collections::HashSet<&str> = TABLE.iter().map(|r| r[0]).collect();
        let mut missing = vec![];
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().is_none_or(|e| e != "rs") || path.ends_with("texts.rs") {
                continue;
            }
            let code = std::fs::read_to_string(&path).unwrap();
            let code = code.split("#[cfg(test)]\nmod tests").next().unwrap();
            for call in ["tr(\"", "trf(\"", "Warn::new(\""] {
                for (at, _) in code.match_indices(call) {
                    if code[..at].chars().next_back().is_some_and(|c| c.is_alphanumeric() || c == '_') && !call.starts_with("Warn") {
                        continue;
                    }
                    let part = &code[at + call.len()..];
                    let end = part.find('"').unwrap_or(0);
                    let text = &part[..end];
                    if part[..end].ends_with('\\') || text.is_empty() {
                        continue;
                    }
                    let shared = base_ui::texts::TEXTS.lookup(Lang::Pl.index() as usize, Box::leak(text.to_string().into_boxed_str())).is_some();
                    if !own.contains(text) && !shared && text != "{}" {
                        missing.push(format!("{}: {text}", path.file_name().unwrap().to_string_lossy()));
                    }
                }
            }
        }
        assert!(missing.is_empty(), "texts without a translation: {missing:#?}");
    }
}

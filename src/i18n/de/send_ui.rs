//! German texts for send_ui.

pub const T: &[(&str, &str)] = &[
    (
        r#"Text lines: $0D = byte as hex, #13 = byte as decimal number,
\r \n \t = control characters, $$ and ## = the character itself.
HEX lines: bytes as hex, e.g. 01 03 0A FF.
Enter in the field sends the line. The ASCII table (above) inserts characters
into the line you clicked last."#,
        r#"Text-Zeilen: $0D = Byte als Hex, #13 = Byte als Dezimalzahl,
\r \n \t = Steuerzeichen, $$ und ## = das Zeichen selbst.
HEX-Zeilen: Bytes als Hex, z. B. 01 03 0A FF.
Enter im Feld sendet die Zeile. Die ASCII-Tabelle (oben) fügt Zeichen
in die zuletzt angeklickte Zeile ein."#,
    ),
    ("Send", "Senden"),
    ("✉ Lines", "✉ Zeilen"),
    ("Send single lines by click or timer", "Einzelne Zeilen per Klick oder Timer senden"),
    ("🔁 Loop", "🔁 Schleife"),
    ("Send several lines one after another with pauses, as often as you like", "Mehrere Zeilen nacheinander mit Pausen senden, beliebig oft"),
    ("📜 Script", "📜 Skript"),
    ("A script for more complex sequences and protocols", "Ein Skript für komplexere Abläufe und Protokolle"),
    ("Appended automatically to text lines", "Wird an Text-Zeilen automatisch angehängt"),
    ("ℹ Help", "ℹ Hilfe"),
    ("➕ Line", "➕ Zeile"),
    ("Received: {} bytes   Sent: {} bytes", "Empfangen: {} Byte   Gesendet: {} Byte"),
    ("e.g. 01 02 0A", "z. B. 01 02 0A"),
    ("Text, e.g. Hello$0D$0A", "Text, z. B. Hallo$0D$0A"),
    ("Input as hex bytes", "Eingabe als Hex-Bytes"),
    ("every", "alle"),
    ("Send repeatedly and automatically", "Automatisch wiederholt senden"),
    ("Remove line", "Zeile entfernen"),
    ("This line is part of the loop", "Diese Zeile gehört zur Schleife"),
    ("then", "dann"),
    ("Pause after this line", "Pause nach dieser Zeile"),
    ("Runs", "Durchläufe"),
    ("0 = endless, until Stop", "0 = endlos, bis Stopp"),
    ("(endless)", "(endlos)"),
    ("⏹ Stop", "⏹ Stopp"),
    ("▶ Run loop", "▶ Schleife starten"),
    ("Loop stopped", "Schleife gestoppt"),
    ("Loop started", "Schleife gestartet"),
    ("Run {} / {}", "Durchlauf {} / {}"),
    ("connect first", "erst verbinden"),
    ("▶ Run script", "▶ Skript starten"),
    ("Load example …", "Beispiel laden …"),
    ("📂 Open", "📂 Öffnen"),
    ("Script not readable: {}", "Skript nicht lesbar: {}"),
    ("💾 Save", "💾 Speichern"),
    ("script.lua", "skript.lua"),
    ("Script not saved: {}", "Skript nicht gespeichert: {}"),
    ("ℹ Commands", "ℹ Befehle"),
    ("running …", "läuft …"),
];

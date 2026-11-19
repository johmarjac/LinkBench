//! German texts for app.

pub const T: &[(&str, &str)] = &[
    ("🔌 Serial", "🔌 Seriell"),
    ("🔗 TCP client", "🔗 TCP-Client"),
    ("🗄 TCP server", "🗄 TCP-Server"),
    ("🏭 Modbus client", "🏭 Modbus-Client"),
    ("🏭 Modbus server", "🏭 Modbus-Server"),
    ("🔧 Tools", "🔧 Werkzeuge"),
    ("ℹ About", "ℹ Info"),
    ("Project loaded: {}", "Projekt geladen: {}"),
    ("{} project", "{}-Projekt"),
    ("project", "projekt"),
    ("Saved: {}", "Gespeichert: {}"),
    ("New project", "Neues Projekt"),
    ("📁 Project", "📁 Projekt"),
    ("🗋 New", "🗋 Neu"),
    ("📂 Open …", "📂 Öffnen …"),
    ("Ctrl+O", "Strg+O"),
    ("💾 Save", "💾 Speichern"),
    ("Ctrl+S", "Strg+S"),
    ("💾 Save as …", "💾 Speichern unter …"),
    (
        "A project holds all sessions, send lines,\nscripts and the Modbus, MQTT and REST settings.",
        "Ein Projekt enthält alle Sitzungen, Sendezeilen,\nSkripte, Modbus-, MQTT- und REST-Einstellungen.",
    ),
    ("📎 Open .{} files with {}", "📎 .{}-Dateien mit {} öffnen"),
    (
        "Register the file extension for your user account (for the portable version; the installer does this itself)",
        "Dateiendung für Ihr Benutzerkonto registrieren (für die portable Version; der Installer erledigt das selbst)",
    ),
    ("Like the computer", "Wie der Computer"),
    ("Use the language of the operating system", "Die Sprache des Betriebssystems verwenden"),
    ("Language", "Sprache"),
    (
        "A tool for developers: serial ports, TCP, UDP, Modbus, MQTT and REST in one window.",
        "Werkzeug für Entwickler: serielle Schnittstellen, TCP, UDP, Modbus, MQTT und REST in einem Fenster.",
    ),
    ("What it can do", "Was es kann"),
    (
        "Several sessions per connection type as tabs: open, rename (double-click), duplicate, close – all run at the same time",
        "Mehrere Sitzungen je Verbindungsart als Reiter: öffnen, benennen (Doppelklick), duplizieren, schließen – alle laufen gleichzeitig",
    ),
    (
        "Open serial ports (baud rate, data bits, parity, stop bits, handshake), switch DTR/RTS/BREAK, watch CTS/DSR/RI/CD",
        "Serielle Ports öffnen (Baudrate, Datenbits, Parität, Stoppbits, Handshake), DTR/RTS/BREAK schalten, CTS/DSR/RI/CD beobachten",
    ),
    ("Connect to devices or services as a TCP client", "Als TCP-Client mit Geräten oder Diensten verbinden"),
    (
        "Accept several clients as a TCP server, send to all or to one, disconnect clients",
        "Als TCP-Server mehrere Clients annehmen, an alle oder einzelne senden, Clients trennen",
    ),
    (
        "Send and receive UDP packets, also broadcast and reply to the last sender",
        "UDP-Pakete senden und empfangen, auch Broadcast und Antwort an den letzten Absender",
    ),
    (
        "Modbus client RTU and TCP: read and write coils, inputs and registers, poll cyclically, values as 16/32 bit, float or text, traffic as hex",
        "Modbus-Client RTU und TCP: Coils, Eingänge und Register lesen und schreiben, zyklisch abfragen, Werte als 16/32 Bit, Float oder Text, Datenverkehr als Hex",
    ),
    (
        "Modbus TCP server (device simulator): edit values in a table, several clients at once, count values up or change them randomly",
        "Modbus-TCP-Server (Geräte-Simulator): Werte in einer Tabelle bearbeiten, mehrere Clients gleichzeitig, Werte automatisch hochzählen oder zufällig ändern",
    ),
    (
        "MQTT client (3.1.1, also TLS): several brokers at once, subscriptions with + and #, message list as text/hex/JSON, topic overview, publish with QoS/retain and repeat",
        "MQTT-Client (3.1.1, auch TLS): mehrere Broker gleichzeitig, Abos mit + und #, Nachrichtenliste als Text/Hex/JSON, Themenübersicht, Veröffentlichen mit QoS/Retain und Wiederholung",
    ),
    (
        "REST client: GET/POST/PUT/PATCH/DELETE, parameters, headers, JSON/text/form body, authentication, variables like {{baseUrl}}, history, cURL export",
        "REST-Client: GET/POST/PUT/PATCH/DELETE, Parameter, Header, JSON/Text/Formular-Body, Anmeldung, Variablen wie {{baseUrl}}, Verlauf, cURL-Export",
    ),
    (
        "Project files (.linkbench): save all sessions and settings and pass them on to colleagues – a double-click opens LinkBench",
        "Projektdateien (.linkbench): alle Sitzungen und Einstellungen speichern und an Kollegen weitergeben – Doppelklick öffnet LinkBench",
    ),
    (
        "Display as text, hex, hex + text or decimal, with time of day and control characters",
        "Anzeige als Text, Hex, Hex + Text oder Dezimal, mit Uhrzeit und Steuerzeichen",
    ),
    (
        "Any number of send lines, as text with $XX/#DDD escapes or as hex, with automatic repeat",
        "Beliebig viele Sendezeilen, als Text mit $XX-/#DDD-Escapes oder als Hex, mit automatischer Wiederholung",
    ),
    (
        "Loop: send several lines one after another with their own pauses, x times or endlessly",
        "Schleife: mehrere Zeilen nacheinander mit eigenen Pausen senden, x-mal oder endlos",
    ),
    (
        "Scripts (Lua) for protocols: send, wait, react to answers, checksums, HTTP requests",
        "Skripte (Lua) für Protokolle: senden, warten, auf Antworten reagieren, Prüfsummen, HTTP-Anfragen",
    ),
    (
        "ASCII table (decimal, hex, binary, character) – one click inserts the character into the send line",
        "ASCII-Tabelle (Dezimal, Hex, Binär, Zeichen) – ein Klick fügt das Zeichen in die Sendezeile ein",
    ),
    (
        "Echo mode, pause, save the window content and a running log to a file",
        "Echo-Modus, Pause, Fensterinhalt speichern und laufende Mitschrift in eine Datei",
    ),
    (
        "Tools: converter text/hex/decimal/binary, checksums (sum, XOR, CRC-8, CRC-16 Modbus/CCITT, CRC-32), number converter",
        "Werkzeuge: Umrechner Text/Hex/Dezimal/Binär, Prüfsummen (Summe, XOR, CRC-8, CRC-16 Modbus/CCITT, CRC-32), Zahlenumrechner",
    ),
    ("Interface in English and German (🌐 in the top bar)", "Oberfläche auf Englisch und Deutsch (🌐 oben rechts)"),
    ("Escape characters in text send lines", "Escape-Zeichen in Text-Sendezeilen"),
    ("one byte as hex (here 13 = CR)", "ein Byte als Hex (hier 13 = CR)"),
    ("one byte as a decimal number", "ein Byte als Dezimalzahl"),
    ("CR, LF, tab, null", "CR, LF, Tab, Null"),
    ("the character $ or # itself", "das Zeichen $ bzw. # selbst"),
    ("Light / dark", "Hell / Dunkel"),
    (
        "Show the ASCII table with decimal, hex and character values",
        "ASCII-Tabelle mit Dezimal-, Hex- und Zeichenwerten einblenden",
    ),
    ("Serial", "Seriell"),
    ("TCP client", "TCP-Client"),
    ("TCP server", "TCP-Server"),
    ("Modbus client", "Modbus-Client"),
    ("Modbus server", "Modbus-Server"),
    ("Client", "Client"),
    ("Server", "Server"),
];

# LinkBench

A workbench for developers that brings serial ports, TCP client, TCP server, UDP, Modbus, MQTT and REST together in one window. Written in Rust with [egui](https://github.com/emilk/egui). The interface is in **English and German**; switch it with 🌐 in the top bar.

*Deutsch: siehe [unten](#deutsch).*

## Features

- **Several sessions:** every connection type can have any number of sessions as tabs. You can open, rename (double-click), duplicate and close them, and they all run at the same time.
- **Serial:** set port, baud rate, data bits, parity, stop bits and handshake. Switch DTR, RTS and BREAK, and watch CTS, DSR, RI and CD.
- **TCP client:** connect to devices or services.
- **TCP server:** accept several clients, send to all of them or to one, disconnect clients.
- **UDP:** send and receive, including broadcast and replying to the last sender.
- **Modbus client RTU and TCP:** read coils, discrete inputs, holding and input registers (functions 01–04) and write them (05, 06, 15, 16). Cyclic polling, display as 16 or 32 bit, float or text in both word orders, traffic as hex, and plain-language messages for exceptions.
- **Modbus TCP server (device simulator):** edit coils, discrete inputs, holding and input registers in a table, serve several clients at once, optionally answer only one unit ID, and count values up or change them randomly. Every request is logged in plain words and as hex. The values are saved with the project.
- **MQTT client (3.1.1):** several brokers at once, optionally with TLS (also your own CA file or without certificate checks), user and password, keep-alive, clean session and automatic reconnect. Subscriptions with `+` and `#` and QoS 0–2, a message list as text, hex or JSON with filter, pause, save and log, a topic overview with the latest value, and publishing with QoS, retain and automatic repeat.
- **REST client:** requests with GET, POST, PUT, PATCH, DELETE, HEAD and OPTIONS, with parameters, headers, body (JSON, text, form) and authentication (bearer token, user + password, API key). Plus saved requests, variables like `{{baseUrl}}`, history, formatted JSON answers and export as a cURL command.
- **Display:** text, hex, hex + text or decimal, with time of day and control characters. Plus echo, pause, save and a running log to a file.
- **Sending:**
  - *Lines:* any number of send lines, as text with `$0D`/`#13` escapes or as hex, with automatic repeat.
  - *Loop:* several lines one after another, each with its own pause, x times or endlessly.
  - *Script:* Lua 5.4 scripts for protocols: send, wait, react to answers and calculate checksums. Examples are included.
- **ASCII table:** decimal, hex, binary and character. One click inserts the character into the send line.
- **Tools:** converter for text, hex, decimal and binary, checksums (sum, XOR, CRC-8, CRC-16 Modbus/CCITT, CRC-32) and a number converter.
- **Project files (`.linkbench`):** all sessions, send lines, scripts and the Modbus, MQTT and REST settings can be saved in one file and passed on. A double-click on the file opens LinkBench with the project: the Windows installer and the Linux package register the extension, and the portable version can do it itself via *Project → Open .linkbench files with LinkBench*. You can also drag project files into the window. Project files from the time the app was called PortWerk (`.portwerk`) still open.
- **Languages:** English and German. LinkBench starts in the computer's language and remembers your choice. Start option: `--lang en` or `--lang de`.
- All settings are saved when you quit.

## Script commands (Lua)

| Command | Meaning |
|---|---|
| `send("Text\r\n")` | send text (`$XX` and `#DDD` are allowed) |
| `send_hex("01 02 FF")` | send bytes as hex |
| `send_bytes(data)` | send raw bytes (Lua string) |
| `wait(ms)` | wait |
| `recv(ms)` / `recv_hex(ms)` | get what was received |
| `wait_for("OK", ms)` | `true` if "OK" arrived in time |
| `clear_rx()` | empty the receive buffer |
| `log(...)` / `print(...)` | write a message into the window |
| `mqtt_publish(topic, payload [, qos, retain, session])` | publish through a connected MQTT session |
| `http_get(url)`, `http_post(url, body)`, `http_request(method, url, body, type)` | HTTP requests, return status and answer |
| `from_hex`, `to_hex`, `crc16_modbus`, `crc16_ccitt`, `crc32`, `crc8`, `sum8`, `xor8` | helpers |

## Adding a language

All texts are written in English in the code as `tr("…")`. A language is a table that maps each English text to its translation; German is in `src/i18n/de/`. To add one, add it to `Lang` in `src/i18n.rs` and give it tables like the German ones. `cargo test` lists every text that has no German translation yet.

## Releases (downloads for every system)

To publish a new version:

1. On GitHub, open the **Actions** tab and pick **Release** on the left.
2. Click **Run workflow** on the right and enter a version, for example `0.2.0`.
3. After about 10–15 minutes the new version with all files is under **Releases**.

Instead of steps 1 and 2 you can also push a tag like `v0.2.0`.

If the version already exists, the run stops right away with a note. To replace it on purpose, tick **Replace existing version**. The old release and its tag are then deleted just before publishing.

What's new is in [CHANGELOG.md](CHANGELOG.md). The section for each version appears on the release page automatically.

| System | File |
|---|---|
| Windows | `…-windows-x64.msi` (installer) and `…-windows-x64-portable.exe` (no installation) |
| macOS (Intel + Apple Silicon) | `…-macos-universal.dmg` and `…-macos-universal.zip` |
| Linux x64 / ARM64 | `…-linux-x64.deb`, `…-linux-arm64.deb` and `.tar.gz` |

Every push to `main` builds and tests the program automatically on Windows, macOS and Linux (workflow **CI**).

## Building

```
cargo build --release
```

The finished program is then in `target/release/`. Tests run with `cargo test`.

## Deutsch

LinkBench ist eine Werkbank für Entwickler: serielle Schnittstellen, TCP-Client, TCP-Server, UDP, Modbus (Client und Server-Simulator), MQTT und REST in einem Fenster, dazu Sendezeilen, Schleifen, Lua-Skripte, ASCII-Tabelle und Prüfsummen-Werkzeuge. Die Oberfläche gibt es auf Deutsch und Englisch; umgeschaltet wird mit 🌐 oben rechts. Projektdateien heißen jetzt `.linkbench`; ältere `.portwerk`-Dateien lassen sich weiter öffnen. Früher hieß das Programm PortWerk.

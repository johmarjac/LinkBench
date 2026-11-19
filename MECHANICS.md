# LinkBench – values and rules

The original program (hercules_3-2-8.exe) was looked at only from outside in the first rounds.
On 2026-11-19 a decompile was attempted, but the file is no longer at
`C:\Users\JohannesJacobs\Downloads\hercules_3-2-8.exe` (opening it fails with "file not found"),
and a search of the user folder found no other copy. So no function addresses exist yet, and
nothing here was read from its code. Every value is the default LinkBench uses, chosen from
common practice ("estimated"). The same list is in `mechanics.json`.

To replace the estimates: put the original back, open it, search its strings (e.g. `9600`,
`Baud`, `$`, `#`, `CRC`), follow the references to the functions, read them, and write each real
value here with its function address.

| Value | Default | Where in LinkBench |
|---|---|---|
| Serial port | 9600 baud, 8 data bits, no parity, 1 stop bit, no handshake | `src/link.rs` `SerialConfig::default` |
| TCP client | 127.0.0.1, port 23 | `src/sessions.rs` |
| TCP server | 0.0.0.0, port 2000 (next session +1) | `src/sessions.rs`, `src/app.rs` |
| UDP | local and remote port 2001 | `src/sessions.rs` |
| Send line repeat | every 1000 ms | `src/terminal/send_ui.rs` |
| Escapes in text lines | `$XX` hex byte, `#DDD` decimal byte, `\r \n \t \0`, `$$`/`##` literal | `src/codec.rs` |
| Terminal history | 4000 entries | `src/terminal.rs` |
| Modbus client | TCP 127.0.0.1:502, unit 1, timeout 1000 ms, poll 1000 ms; RTU 19200 baud, even parity | `src/modbus.rs` |
| Modbus server | port 502 (next session +1), 65536 values per area | `src/modbus_server.rs` |
| MQTT | localhost:1883, keep alive 30 s, up to 5000 messages and topics | `src/mqtt.rs` |
| Languages | English and German; starts in the computer's language (else English), the choice is remembered; `--lang en/de` | `src/i18n.rs`, `src/app.rs` |
| Project files | `.linkbench`; `.portwerk` files from before the rename still open | `src/project.rs` |

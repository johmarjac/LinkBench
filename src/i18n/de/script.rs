//! German texts for script.

pub const T: &[(&str, &str)] = &[
    ("Script could not be prepared: {}", "Skript konnte nicht vorbereitet werden: {}"),
    ("Script started", "Skript gestartet"),
    ("Script", "Skript"),
    ("Script finished", "Skript beendet"),
    ("Script stopped", "Skript gestoppt"),
    ("Script error: {}", "Skriptfehler: {}"),
    // ---- examples
    ("Loop: 10 messages", "Schleife: 10 Nachrichten"),
    (
        r#"-- Sends 10 numbered messages 500 ms apart
for i = 1, 10 do
    send("Message " .. i .. "\r\n")
    wait(500)
end
"#,
        r#"-- Schickt 10 nummerierte Nachrichten im Abstand von 500 ms
for i = 1, 10 do
    send("Nachricht " .. i .. "\r\n")
    wait(500)
end
"#,
    ),
    ("Question / answer (AT command)", "Frage / Antwort (AT-Befehl)"),
    (
        r#"-- Sends a command and waits up to 2 s for "OK"
clear_rx()
send("AT\r")
if wait_for("OK", 2000) then
    log("Device answered with OK")
else
    log("No answer within 2 s")
end
"#,
        r#"-- Sendet einen Befehl und wartet bis zu 2 s auf "OK"
clear_rx()
send("AT\r")
if wait_for("OK", 2000) then
    log("Gerät hat mit OK geantwortet")
else
    log("Keine Antwort innerhalb von 2 s")
end
"#,
    ),
    ("Modbus RTU with CRC", "Modbus RTU mit CRC"),
    (
        r#"-- Read holding registers: device 1, starting at address 0, 10 registers
local req = from_hex("01 03 00 00 00 0A")
local crc = crc16_modbus(req)
req = req .. string.char(crc & 0xFF, (crc >> 8) & 0xFF)  -- CRC low byte first
send_bytes(req)
local answer = recv(1000)
if #answer == 0 then
    log("No answer")
else
    log("Answer: " .. to_hex(answer))
end
"#,
        r#"-- Holding-Register lesen: Gerät 1, ab Adresse 0, 10 Register
local req = from_hex("01 03 00 00 00 0A")
local crc = crc16_modbus(req)
req = req .. string.char(crc & 0xFF, (crc >> 8) & 0xFF)  -- CRC low byte zuerst
send_bytes(req)
local antwort = recv(1000)
if #antwort == 0 then
    log("Keine Antwort")
else
    log("Antwort: " .. to_hex(antwort))
end
"#,
    ),
    ("Automatic responder", "Automatischer Antworter"),
    (
        r#"-- Answers every "PING" with "PONG" – runs until Stop
while true do
    if wait_for("PING", 1000) then
        send("PONG\r\n")
        log("PING answered")
    end
end
"#,
        r#"-- Antwortet auf jedes "PING" mit "PONG" – läuft bis Stopp
while true do
    if wait_for("PING", 1000) then
        send("PONG\r\n")
        log("PING beantwortet")
    end
end
"#,
    ),
    ("Forward measurement to REST API", "Messwert an REST-API weiterleiten"),
    (
        r#"-- Reads replies from the device and sends each line as JSON to an API
local url = "https://httpbin.org/post"
while true do
    local line = recv(2000)
    if #line > 0 then
        local json = '{"value": "' .. line:gsub("[\r\n\"]", "") .. '"}'
        local status, answer = http_post(url, json)
        log("API response: " .. status)
    end
end
"#,
        r#"-- Liest Antworten vom Gerät und schickt jede Zeile als JSON an eine API
local url = "https://httpbin.org/post"
while true do
    local zeile = recv(2000)
    if #zeile > 0 then
        local json = '{"wert": "' .. zeile:gsub("[\r\n\"]", "") .. '"}'
        local status, antwort = http_post(url, json)
        log("API-Antwort: " .. status)
    end
end
"#,
    ),
    ("Forward measurement to MQTT", "Messwert an MQTT weiterleiten"),
    (
        r#"-- Forwards every received line to MQTT as JSON
-- (connect a session in the MQTT tab first)
while true do
    local line = recv(2000)
    if #line > 0 then
        local value = line:gsub("[\r\n\"]", "")
        mqtt_publish("linkbench/measurement", '{"value": "' .. value .. '"}')
        log("sent to MQTT: " .. value)
    end
end
"#,
        r#"-- Leitet jede empfangene Zeile als JSON an MQTT weiter
-- (vorher im Reiter MQTT eine Sitzung verbinden)
while true do
    local zeile = recv(2000)
    if #zeile > 0 then
        local wert = zeile:gsub("[\r\n\"]", "")
        mqtt_publish("linkbench/messwert", '{"wert": "' .. wert .. '"}')
        log("an MQTT gesendet: " .. wert)
    end
end
"#,
    ),
    ("Endless loop with counter", "Endlos-Schleife mit Zähler"),
    (
        r#"-- Sends AA + a counter byte every second – runs until Stop
local n = 0
while true do
    send_bytes(string.char(0xAA, n % 256))
    n = n + 1
    wait(1000)
end
"#,
        r#"-- Sendet jede Sekunde AA + Zähler-Byte – läuft bis Stopp
local n = 0
while true do
    send_bytes(string.char(0xAA, n % 256))
    n = n + 1
    wait(1000)
end
"#,
    ),
    // ---- help
    (
        r#"Language: Lua 5.4  (comment: -- ..., join text: ..)
Commands:
  send("text\r\n")       send text ($XX and #DDD allowed)
  send_hex("01 02 FF")   send bytes given as hex
  send_bytes(data)       send raw bytes (Lua string)
  wait(ms)               wait
  recv(ms)               fetch received data (waits at most ms)
  recv_hex(ms)           ... as hex text
  wait_for("OK", ms)     true if "OK" arrived in time
  clear_rx()             clear the receive buffer
  log(...) / print(...)  write a message to the window
  now_ms()               milliseconds since script start
MQTT (through a connected MQTT session):
  mqtt_publish("topic", "payload" [, qos, retain, "session name"])
HTTP / REST (returns status and response):
  local st, txt = http_get("https://...")
  local st, txt = http_post("https://...", '{"value":1}')   (JSON)
  http_request("PUT", url, body, "text/plain")
Helpers:
  from_hex("01 02"), to_hex(data), string.char(1, 2), #data
  crc16_modbus(d), crc16_ccitt(d), crc32(d), crc8(d), sum8(d), xor8(d)"#,
        r#"Sprache: Lua 5.4  (Kommentar: -- …, Text verbinden: ..)
Befehle:
  send("Text\r\n")       Text senden ($XX und #DDD erlaubt)
  send_hex("01 02 FF")   Bytes als Hex senden
  send_bytes(daten)      Bytes roh senden (Lua-String)
  wait(ms)               warten
  recv(ms)               Empfangenes holen (wartet max. ms)
  recv_hex(ms)           … als Hex-Text
  wait_for("OK", ms)     true, wenn "OK" rechtzeitig kam
  clear_rx()             Empfangspuffer leeren
  log(…) / print(…)      Meldung ins Fenster schreiben
  now_ms()               Millisekunden seit Skriptstart
MQTT (über eine verbundene MQTT-Sitzung):
  mqtt_publish("thema", "inhalt" [, qos, retain, "Sitzungsname"])
HTTP / REST (liefert Status und Antwort):
  local st, txt = http_get("https://…")
  local st, txt = http_post("https://…", '{"wert":1}')   (JSON)
  http_request("PUT", url, body, "text/plain")
Helfer:
  from_hex("01 02"), to_hex(daten), string.char(1, 2), #daten
  crc16_modbus(d), crc16_ccitt(d), crc32(d), crc8(d), sum8(d), xor8(d)"#,
    ),
];

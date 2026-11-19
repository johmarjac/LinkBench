//! Script runner (Lua 5.4) for protocol work that goes beyond fixed send
//! lines. A script runs on its own thread; it sends through the tab's
//! connection and reads what that connection receives.
//!
//! In Lua, strings are byte strings, so binary data is simply a string:
//! `string.char(0x01, 0x03)`, `from_hex("01 03")`, `#data`, `data:byte(1)`.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread;
use std::time::{Duration, Instant};

use mlua::{HookTriggers, Lua, MultiValue, VmState};

use crate::codec;
use crate::i18n::tr;

pub enum ScriptMsg {
    Send(Vec<u8>),
    Log(String),
    Error(String),
    Done,
}

pub struct ScriptRun {
    stop: Arc<AtomicBool>,
    feed: Sender<Vec<u8>>,
    out: Receiver<ScriptMsg>,
}

impl ScriptRun {
    pub fn stop(&self) {
        self.stop.store(true, Ordering::Relaxed);
    }
    pub fn feed(&self, data: &[u8]) {
        let _ = self.feed.send(data.to_vec());
    }
    pub fn poll(&self) -> Vec<ScriptMsg> {
        self.out.try_iter().collect()
    }
}

impl Drop for ScriptRun {
    fn drop(&mut self) {
        self.stop();
    }
}

struct RxState {
    rx: Receiver<Vec<u8>>,
    buf: Vec<u8>,
}

impl RxState {
    fn pull(&mut self) {
        for d in self.rx.try_iter() {
            self.buf.extend_from_slice(&d);
        }
    }
}

/// Internal marker of the error that stops a script (never shown; the log line uses `tr`).
const STOPPED: &str = "script stopped";

fn stopped() -> mlua::Error {
    mlua::Error::runtime(STOPPED)
}

/// Sleep in slices, aborting early when the stop flag goes up.
fn nap(stop: &AtomicBool, ms: i64) -> mlua::Result<()> {
    let end = Instant::now() + Duration::from_millis(ms.max(0) as u64);
    while Instant::now() < end {
        if stop.load(Ordering::Relaxed) {
            return Err(stopped());
        }
        thread::sleep(Duration::from_millis(5).min(end.saturating_duration_since(Instant::now())));
    }
    Ok(())
}

fn find(hay: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() {
        return Some(0);
    }
    hay.windows(needle.len()).position(|w| w == needle)
}

fn values_to_text(args: MultiValue) -> String {
    args.iter()
        .map(|v| match v {
            mlua::Value::String(s) => s.to_string_lossy(),
            other => other.to_string().unwrap_or_else(|_| format!("{other:?}")),
        })
        .collect::<Vec<_>>()
        .join("\t")
}

fn setup(lua: &Lua, emit: impl Fn(ScriptMsg) + Clone + 'static, stop: Arc<AtomicBool>, rx: Rc<RefCell<RxState>>) -> mlua::Result<()> {
    let g = lua.globals();
    {
        let stop = stop.clone();
        lua.set_hook(HookTriggers::new().every_nth_instruction(1000), move |_, _| {
            if stop.load(Ordering::Relaxed) { Err(stopped()) } else { Ok(VmState::Continue) }
        })?;
    }

    // --- output
    {
        let e = emit.clone();
        g.set("log", lua.create_function(move |_, args: MultiValue| {
            e(ScriptMsg::Log(values_to_text(args)));
            Ok(())
        })?)?;
    }
    {
        let e = emit.clone();
        g.set("print", lua.create_function(move |_, args: MultiValue| {
            e(ScriptMsg::Log(values_to_text(args)));
            Ok(())
        })?)?;
    }

    // --- sending
    {
        let e = emit.clone();
        g.set("send", lua.create_function(move |_, s: mlua::LuaString| {
            let text = String::from_utf8_lossy(&s.as_bytes()).into_owned();
            let d = codec::parse_text(&text).map_err(mlua::Error::runtime)?;
            e(ScriptMsg::Send(d));
            Ok(())
        })?)?;
    }
    {
        let e = emit.clone();
        g.set("send_hex", lua.create_function(move |_, s: String| {
            let d = codec::parse_hex(&s).map_err(mlua::Error::runtime)?;
            e(ScriptMsg::Send(d));
            Ok(())
        })?)?;
    }
    {
        let e = emit.clone();
        g.set("send_bytes", lua.create_function(move |_, s: mlua::LuaString| {
            e(ScriptMsg::Send(s.as_bytes().to_vec()));
            Ok(())
        })?)?;
    }

    // --- timing
    {
        let st = stop.clone();
        g.set("wait", lua.create_function(move |_, ms: i64| nap(&st, ms))?)?;
    }
    {
        let start = Instant::now();
        g.set("now_ms", lua.create_function(move |_, ()| Ok(start.elapsed().as_millis() as i64))?)?;
    }

    // --- receiving
    let recv_raw = {
        let rx = rx.clone();
        let st = stop.clone();
        move |timeout: Option<i64>| -> mlua::Result<Vec<u8>> {
            let end = Instant::now() + Duration::from_millis(timeout.unwrap_or(1000).max(0) as u64);
            loop {
                rx.borrow_mut().pull();
                if !rx.borrow().buf.is_empty() {
                    // give the rest of a burst a moment to arrive
                    nap(&st, 20)?;
                    rx.borrow_mut().pull();
                    return Ok(std::mem::take(&mut rx.borrow_mut().buf));
                }
                if Instant::now() >= end {
                    return Ok(Vec::new());
                }
                nap(&st, 5)?;
            }
        }
    };
    {
        let r = recv_raw.clone();
        g.set("recv", lua.create_function(move |lua, t: Option<i64>| lua.create_string(r(t)?))?)?;
    }
    {
        let r = recv_raw.clone();
        g.set("recv_hex", lua.create_function(move |_, t: Option<i64>| Ok(codec::to_hex(&r(t)?)))?)?;
    }
    {
        let rx = rx.clone();
        let st = stop.clone();
        g.set("wait_for", lua.create_function(move |_, (pat, timeout): (mlua::LuaString, Option<i64>)| {
            let text = String::from_utf8_lossy(&pat.as_bytes()).into_owned();
            let needle = codec::parse_text(&text).map_err(mlua::Error::runtime)?;
            let end = Instant::now() + Duration::from_millis(timeout.unwrap_or(1000).max(0) as u64);
            loop {
                {
                    let mut s = rx.borrow_mut();
                    s.pull();
                    if let Some(p) = find(&s.buf, &needle) {
                        s.buf.drain(..p + needle.len());
                        return Ok(true);
                    }
                }
                if Instant::now() >= end {
                    return Ok(false);
                }
                nap(&st, 5)?;
            }
        })?)?;
    }
    {
        let rx = rx.clone();
        g.set("clear_rx", lua.create_function(move |_, ()| {
            let mut s = rx.borrow_mut();
            s.pull();
            s.buf.clear();
            Ok(())
        })?)?;
    }

    // --- helpers (byte data = Lua string)
    g.set("from_hex", lua.create_function(|lua, s: String| {
        lua.create_string(codec::parse_hex(&s).map_err(mlua::Error::runtime)?)
    })?)?;
    g.set("to_hex", lua.create_function(|_, s: mlua::LuaString| Ok(codec::to_hex(&s.as_bytes())))?)?;
    g.set("crc16_modbus", lua.create_function(|_, s: mlua::LuaString| Ok(codec::crc16_modbus(&s.as_bytes()) as i64))?)?;
    g.set("crc16_ccitt", lua.create_function(|_, s: mlua::LuaString| Ok(codec::crc16_ccitt(&s.as_bytes()) as i64))?)?;
    g.set("crc32", lua.create_function(|_, s: mlua::LuaString| Ok(codec::crc32(&s.as_bytes()) as i64))?)?;
    g.set("crc8", lua.create_function(|_, s: mlua::LuaString| Ok(codec::crc8(&s.as_bytes()) as i64))?)?;
    g.set("sum8", lua.create_function(|_, s: mlua::LuaString| Ok(codec::sum8(&s.as_bytes()) as i64))?)?;
    g.set("xor8", lua.create_function(|_, s: mlua::LuaString| Ok(codec::xor8(&s.as_bytes()) as i64))?)?;

    // --- MQTT: publish through a connected MQTT session
    g.set("mqtt_publish", lua.create_function(
        |_, (topic, payload, q, retain, session): (String, mlua::LuaString, Option<u8>, Option<bool>, Option<String>)| {
            crate::mqtt::script_publish(
                session.as_deref().unwrap_or(""),
                &topic,
                payload.as_bytes().to_vec(),
                q.unwrap_or(0),
                retain.unwrap_or(false),
            )
            .map_err(mlua::Error::runtime)
        },
    )?)?;

    // --- HTTP (e.g. forward measured values to a REST API)
    g.set("http_get", lua.create_function(|lua, url: String| {
        let (status, body) = crate::rest::simple_request("GET", &url, None, None).map_err(mlua::Error::runtime)?;
        Ok((status as i64, lua.create_string(body)?))
    })?)?;
    g.set("http_post", lua.create_function(|lua, (url, body, ct): (String, mlua::LuaString, Option<String>)| {
        let ct = ct.unwrap_or_else(|| "application/json".into());
        let (status, body) = crate::rest::simple_request("POST", &url, Some(body.as_bytes().to_vec()), Some(&ct))
            .map_err(mlua::Error::runtime)?;
        Ok((status as i64, lua.create_string(body)?))
    })?)?;
    g.set("http_request", lua.create_function(|lua, (method, url, body, ct): (String, String, Option<mlua::LuaString>, Option<String>)| {
        let (status, body) = crate::rest::simple_request(&method, &url, body.map(|b| b.as_bytes().to_vec()), ct.as_deref())
            .map_err(mlua::Error::runtime)?;
        Ok((status as i64, lua.create_string(body)?))
    })?)?;

    // scripts talk to the connection only: no files, processes or modules
    for name in ["os", "io", "require", "dofile", "loadfile", "package", "debug"] {
        g.set(name, mlua::Value::Nil)?;
    }
    Ok(())
}

pub fn run(ctx: eframe::egui::Context, src: String) -> ScriptRun {
    let stop = Arc::new(AtomicBool::new(false));
    let (feed_tx, feed_rx) = channel::<Vec<u8>>();
    let (out_tx, out_rx) = channel::<ScriptMsg>();
    let stop2 = stop.clone();
    thread::spawn(move || {
        let stop = stop2;
        let emit = move |m: ScriptMsg| {
            let _ = out_tx.send(m);
            ctx.request_repaint();
        };
        let rx = Rc::new(RefCell::new(RxState { rx: feed_rx, buf: Vec::new() }));
        let lua = Lua::new();
        if let Err(e) = setup(&lua, emit.clone(), stop.clone(), rx) {
            emit(ScriptMsg::Error(crate::i18n::trf("Script could not be prepared: {}", &[&e])));
            emit(ScriptMsg::Done);
            return;
        }
        emit(ScriptMsg::Log(tr("Script started").into()));
        match lua.load(&src).set_name(tr("Script")).exec() {
            Ok(()) => emit(ScriptMsg::Log(tr("Script finished").into())),
            Err(e) => {
                let msg = e.to_string();
                if stop.load(Ordering::Relaxed) || msg.contains(STOPPED) {
                    emit(ScriptMsg::Log(tr("Script stopped").into()));
                } else {
                    let first = msg.lines().next().unwrap_or(&msg).replace("runtime error: ", "");
                    emit(ScriptMsg::Error(crate::i18n::trf("Script error: {}", &[&first])));
                }
            }
        }
        emit(ScriptMsg::Done);
    });
    ScriptRun { stop, feed: feed_tx, out: out_rx }
}

/// The example scripts as (name, code) in the current language.
pub fn examples() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            tr("Loop: 10 messages"),
            tr(r#"-- Sends 10 numbered messages 500 ms apart
for i = 1, 10 do
    send("Message " .. i .. "\r\n")
    wait(500)
end
"#),
        ),
        (
            tr("Question / answer (AT command)"),
            tr(r#"-- Sends a command and waits up to 2 s for "OK"
clear_rx()
send("AT\r")
if wait_for("OK", 2000) then
    log("Device answered with OK")
else
    log("No answer within 2 s")
end
"#),
        ),
        (
            tr("Modbus RTU with CRC"),
            tr(r#"-- Read holding registers: device 1, starting at address 0, 10 registers
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
"#),
        ),
        (
            tr("Automatic responder"),
            tr(r#"-- Answers every "PING" with "PONG" – runs until Stop
while true do
    if wait_for("PING", 1000) then
        send("PONG\r\n")
        log("PING answered")
    end
end
"#),
        ),
        (
            tr("Forward measurement to REST API"),
            tr(r#"-- Reads replies from the device and sends each line as JSON to an API
local url = "https://httpbin.org/post"
while true do
    local line = recv(2000)
    if #line > 0 then
        local json = '{"value": "' .. line:gsub("[\r\n\"]", "") .. '"}'
        local status, answer = http_post(url, json)
        log("API response: " .. status)
    end
end
"#),
        ),
        (
            tr("Forward measurement to MQTT"),
            tr(r#"-- Forwards every received line to MQTT as JSON
-- (connect a session in the MQTT tab first)
while true do
    local line = recv(2000)
    if #line > 0 then
        local value = line:gsub("[\r\n\"]", "")
        mqtt_publish("linkbench/measurement", '{"value": "' .. value .. '"}')
        log("sent to MQTT: " .. value)
    end
end
"#),
        ),
        (
            tr("Endless loop with counter"),
            tr(r#"-- Sends AA + a counter byte every second – runs until Stop
local n = 0
while true do
    send_bytes(string.char(0xAA, n % 256))
    n = n + 1
    wait(1000)
end
"#),
        ),
    ]
}

/// The help text for the script commands, in the current language.
pub fn help() -> &'static str {
    tr(r#"Language: Lua 5.4  (comment: -- ..., join text: ..)
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
  crc16_modbus(d), crc16_ccitt(d), crc32(d), crc8(d), sum8(d), xor8(d)"#)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn script_sends_and_waits_for_answer() {
        let ctx = eframe::egui::Context::default();
        let src = r#"
            local req = from_hex("01 03 00 00 00 0A")
            local crc = crc16_modbus(req)
            req = req .. string.char(crc & 0xFF, (crc >> 8) & 0xFF)
            send_bytes(req)
            if wait_for("OK", 2000) then log("ja") else log("nein") end
        "#;
        let run = run(ctx, src.into());
        let mut sent = None;
        let mut logs = Vec::new();
        let end = Instant::now() + Duration::from_secs(5);
        'o: while Instant::now() < end {
            for m in run.poll() {
                match m {
                    ScriptMsg::Send(d) => {
                        sent = Some(d);
                        run.feed(b"xxOKyy");
                    }
                    ScriptMsg::Log(l) => logs.push(l),
                    ScriptMsg::Error(e) => panic!("{e}"),
                    ScriptMsg::Done => break 'o,
                }
            }
            thread::sleep(Duration::from_millis(5));
        }
        assert_eq!(sent.unwrap(), vec![1, 3, 0, 0, 0, 0x0A, 0xC5, 0xCD]);
        assert!(logs.iter().any(|l| l == "ja"), "{logs:?}");
    }

    #[test]
    fn script_can_be_stopped() {
        let ctx = eframe::egui::Context::default();
        let run = run(ctx, "while true do end".into());
        thread::sleep(Duration::from_millis(100));
        run.stop();
        let end = Instant::now() + Duration::from_secs(3);
        let mut done = false;
        while Instant::now() < end && !done {
            done = run.poll().into_iter().any(|m| matches!(m, ScriptMsg::Done));
            thread::sleep(Duration::from_millis(5));
        }
        assert!(done);
    }

    #[test]
    fn examples_compile() {
        let lua = Lua::new();
        // tests run at the same time, so the language isn't switched: the German texts are
        // looked up in the table directly
        let all = examples();
        assert_eq!(all.len(), 7);
        for (name, src) in all {
            for (lang, code) in [("en", src), ("de", crate::i18n::german(src))] {
                lua.load(code).into_function().unwrap_or_else(|e| panic!("{lang} {name}: {e}"));
            }
        }
    }
}

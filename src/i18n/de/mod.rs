//! German translations, one table per part of the program: (English, German).

mod app;
mod sessions;
mod link;
mod terminal;
mod send_ui;
mod script;
mod mqtt;
mod modbus;
mod modbus_server;
mod rest;
mod tools;
mod codec;
mod ascii;
mod glyph_check;
mod project;

pub const TABLES: &[&[(&str, &str)]] = &[
    app::T,
    sessions::T,
    link::T,
    terminal::T,
    send_ui::T,
    script::T,
    mqtt::T,
    modbus::T,
    modbus_server::T,
    rest::T,
    tools::T,
    codec::T,
    ascii::T,
    glyph_check::T,
    project::T,
];

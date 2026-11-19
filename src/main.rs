#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod ascii;
mod codec;
mod glyph_check;
mod i18n;
mod link;
mod modbus;
mod modbus_server;
mod mqtt;
mod project;
mod rest;
mod script;
mod sessions;
mod terminal;
mod tools;

use std::path::PathBuf;

use eframe::egui;

fn main() -> eframe::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let shot = args.iter().position(|a| a == "--screenshot").map(|i| {
        PathBuf::from(args.get(i + 1).cloned().unwrap_or_else(|| "screenshot.png".into()))
    });
    // a project file given on the command line (double click on a project file)
    let mut open_file = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--screenshot" | "--tab" | "--mode" | "--lang" => i += 2,
            a if a.starts_with("--") => i += 1,
            a => {
                open_file = Some(PathBuf::from(a));
                i += 1;
            }
        }
    }

    migrate_old_settings();

    let mut viewport = egui::ViewportBuilder::default()
        .with_title(format!("{} {}", app::APP_NAME, app::APP_VERSION))
        .with_app_id(app::APP_ID)
        .with_inner_size([1180.0, 760.0])
        .with_min_inner_size([820.0, 520.0]);

    #[allow(unused_mut)]
    let mut options = eframe::NativeOptions::default();
    if shot.is_some() {
        viewport = viewport.with_active(false);
        #[cfg(windows)]
        {
            viewport = viewport.with_window_level(egui::WindowLevel::AlwaysOnBottom);
        }
        #[cfg(target_os = "macos")]
        {
            options.event_loop_builder = Some(Box::new(|b| {
                use winit::platform::macos::EventLoopBuilderExtMacOS;
                b.with_activate_ignoring_other_apps(false);
            }));
        }
        options.persist_window = false;
    }
    options.viewport = viewport;

    eframe::run_native(
        app::APP_NAME,
        options,
        Box::new(move |cc| {
            let get = |k: &str| args.iter().position(|a| a == k).and_then(|i| args.get(i + 1)).cloned();
            let lang = get("--lang").and_then(|l| i18n::Lang::from_code(&l));
            let mut a = app::LinkBenchApp::new(cc, shot, open_file, lang);
            // optional: --tab <serial|tcpc|tcps|udp|modbus|rest|tools|info> [--mode lines|loop|script] [--ascii]
            //           --lang <en|de>
            a.apply_start_args(get("--tab").as_deref(), get("--mode").as_deref(), args.iter().any(|a| a == "--ascii"));
            Ok(Box::new(a))
        }),
    )
}

/// The app used to be called PortWerk. Its saved sessions move over once, on the first start
/// under the new name.
fn migrate_old_settings() {
    let (Some(old), Some(new)) = (eframe::storage_dir(app::OLD_APP_NAME), eframe::storage_dir(app::APP_NAME)) else {
        return;
    };
    if old == new || !old.is_dir() || new.exists() {
        return;
    }
    if std::fs::create_dir_all(&new).is_err() {
        return;
    }
    if let Ok(entries) = std::fs::read_dir(&old) {
        for e in entries.flatten() {
            if e.path().is_file() {
                let _ = std::fs::copy(e.path(), new.join(e.file_name()));
            }
        }
    }
}

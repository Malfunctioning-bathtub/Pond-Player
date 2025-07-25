#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use log;

fn main() -> eframe::Result {
    std::env::set_var("RUST_LOG", "eframe=warn, egui_glow=warn, calloop=warn, trace");
    println!("meow meow meow meow meow meow meow meow meow meow meow meow meow meow meow meow meow meow meow meow meow meow meow meow meow meow");
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    log::info!("Starting up");

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 22.0])
            .with_icon(
                // NOTE: Adding an icon is optional
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
                    .expect("Failed to load icon"),
            ),
        ..Default::default()
    };
    eframe::run_native(
        "Player",
        native_options,
        Box::new(|cc| Ok(Box::new(pond_player::PlayerApp::new(cc)))),
    )
}
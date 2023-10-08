mod app;
mod components;
mod config;
mod utils;

use std::path::PathBuf;

use clap::Parser;
use eframe::egui;
use tracing_subscriber::prelude::*;

const MIN_WINDOW_SIZE: egui::Vec2 = egui::Vec2 { x: 900.0, y: 600.0 };

#[derive(Parser)]
struct Args {
    #[arg(long)]
    override_config: Option<PathBuf>,
}

fn main() -> miette::Result<()> {
    miette::set_panic_hook();

    let Args { override_config } = Args::parse();

    #[cfg(debug_assertions)]
    let log_level = {
        std::env::set_var("RUST_LOG", "debug");
        tracing::Level::DEBUG
    };

    #[cfg(not(debug_assertions))]
    let log_level = {
        std::env::set_var("RUST_LOG", "info");
        tracing::Level::INFO
    };

    let log_event_collector = egui_tracing::EventCollector::new().with_level(log_level);

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(log_event_collector.clone())
        .init();

    let native_options = eframe::NativeOptions {
        always_on_top: false,
        maximized: false,
        decorated: true,
        fullscreen: false,
        drag_and_drop_support: true,
        icon_data: None, // TODO
        initial_window_pos: None,
        initial_window_size: Some(MIN_WINDOW_SIZE),
        min_window_size: Some(MIN_WINDOW_SIZE),
        max_window_size: None,
        resizable: true,
        mouse_passthrough: false,
        active: true,
        follow_system_theme: true,
        default_theme: eframe::Theme::Dark,
        centered: true,
        app_id: Some("tukanoidd.rsfm".into()),
        ..Default::default()
    };

    eframe::run_native(
        "Rusty File Manager",
        native_options,
        Box::new(|cc| Box::new(app::App::new(cc, override_config, log_event_collector))),
    )
    .map_err(|err| miette::miette!("{err}"))
}

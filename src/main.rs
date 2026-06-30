#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod avatars;
mod data;
mod game;
mod neon_text;
mod theme;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1100.0, 750.0])
            .with_min_inner_size([900.0, 600.0])
            .with_title("Neon Diaspora — Search for the Lost"),
        ..Default::default()
    };

    eframe::run_native(
        "Neon Diaspora",
        options,
        Box::new(|_cc| Ok(Box::new(app::NeonDiasporaApp::new()))),
    )
}
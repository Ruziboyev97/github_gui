#![windows_subsystem = "windows"]
mod github_client;
mod gui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "GitHub GUI",
        options,
        Box::new(|_cc| Box::new(gui::MyApp::default())),
    )
}

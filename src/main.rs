#![windows_subsystem = "windows"]
mod github_client;
mod gui;
mod models;


fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native("GitHub Manager", options, Box::new(|_| Box::new(gui::MyApp::default())))
}

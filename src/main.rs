mod github_client;
mod gui;

use gui::MyApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "GitHub Repos",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

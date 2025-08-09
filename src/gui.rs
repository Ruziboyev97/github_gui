use crate::github_client::{GitHubClient, Repo};
use eframe::egui;

pub struct MyApp {
    username: String,
    repos: Vec<Repo>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            username: "rust-lang".to_string(),
            repos: vec![],
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("GitHub Repositories Viewer");

            ui.horizontal(|ui| {
                ui.label("Username:");
                ui.text_edit_singleline(&mut self.username);
                if ui.button("Load").clicked() {
                    self.repos = GitHubClient::fetch_repos(&self.username);
                }
            });

            ui.separator();

            if self.repos.is_empty() {
                ui.label("No repositories loaded.");
            } else {
                for repo in &self.repos {
                    ui.hyperlink_to(&repo.name, &repo.html_url);
                }
            }
        });
    }
}

use crate::{github_client::GitHubClient, models::Repo};
use eframe::egui::{CentralPanel, Context, TextEdit, Ui};
use std::sync::{Arc, Mutex};

pub struct MyApp {
    token: String,
    repos: Arc<Mutex<Vec<Repo>>>,
    client: Option<GitHubClient>,
    new_repo_name: String,
    status: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            token: String::new(),
            repos: Arc::new(Mutex::new(Vec::new())),
            client: None,
            new_repo_name: String::new(),
            status: String::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            if self.client.is_none() {
                self.show_token_input(ui);
            } else {
                self.show_repo_manager(ui);
            }
        });
    }
}

impl MyApp {
    fn show_token_input(&mut self, ui: &mut Ui) {
        ui.label("Введите GitHub Personal Access Token:");
        ui.add(TextEdit::singleline(&mut self.token).password(true));

        if ui.button("Войти").clicked() {
            let client = GitHubClient::new(self.token.clone());
            match client.list_repos() {
                Ok(list) => {
                    *self.repos.lock().unwrap() = list;
                    self.client = Some(client);
                    self.status.clear();
                }
                Err(e) => {
                    self.status = e;
                }
            }
        }

        if !self.status.is_empty() {
            ui.label(&self.status);
        }
    }

    fn show_repo_manager(&mut self, ui: &mut Ui) {
        ui.heading("Ваши репозитории");

        if ui.button("Обновить список").clicked() {
            if let Some(client) = &self.client {
                match client.list_repos() {
                    Ok(list) => *self.repos.lock().unwrap() = list,
                    Err(e) => self.status = e,
                }
            }
        }

        for repo in self.repos.lock().unwrap().clone() {
            ui.horizontal(|ui| {
                ui.label(&repo.name);
                if ui.button("Удалить").clicked() {
                    if let Some(client) = &self.client {
                        // Нужно указать owner (можно извлечь из html_url)
                        let parts: Vec<&str> = repo.html_url.split('/').collect();
                        if parts.len() >= 5 {
                            let owner = parts[3];
                            let name = parts[4];
                            match client.delete_repo(owner, name) {
                                Ok(_) => {
                                    self.status = format!("Репозиторий {} удалён", name);
                                }
                                Err(e) => self.status = e,
                            }
                        }
                    }
                }
            });
        }

        ui.separator();
        ui.label("Создать новый репозиторий:");
        ui.add(TextEdit::singleline(&mut self.new_repo_name));

        if ui.button("Создать").clicked() {
            if let Some(client) = &self.client {
                match client.create_repo(&self.new_repo_name) {
                    Ok(_) => self.status = "Репозиторий создан".into(),
                    Err(e) => self.status = e,
                }
            }
        }

        if !self.status.is_empty() {
            ui.label(&self.status);
        }
    }
}

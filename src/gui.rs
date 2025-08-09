use crate::github_client::GitHubClient;
use copypasta::{ClipboardContext, ClipboardProvider};
use eframe::egui::{CentralPanel, Context, TextEdit};

pub struct MyApp {
    token: String,
    owner: Option<String>,
    repos: Vec<String>,
    new_repo: String,
    status: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            token: String::new(),
            owner: None,
            repos: Vec::new(),
            new_repo: String::new(),
            status: String::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("GitHub Репозиторий Менеджер");

            ui.horizontal(|ui| {
                ui.add(
                    TextEdit::singleline(&mut self.token)
                        .password(true)
                        .desired_width(250.0),
                );
                if ui.button("Вставить из буфера").clicked() {
                    let mut clipboard = ClipboardContext::new().unwrap();
                    if let Ok(contents) = clipboard.get_contents() {
                        self.token = contents;
                        self.status = "Токен вставлен из буфера обмена".to_string();
                    } else {
                        self.status = "Не удалось получить содержимое буфера".to_string();
                    }
                }
            });

            if ui.button("Загрузить репозитории").clicked() {
                if self.token.trim().is_empty() {
                    self.status = "Введите токен!".to_string();
                } else {
                    let client = GitHubClient::new(self.token.clone());
                    match client.get_owner() {
                        Ok(owner_login) => {
                            self.owner = Some(owner_login.clone());
                            match client.list_repos() {
                                Ok(list) => {
                                    self.repos = list;
                                    self.status = format!(
                                        "Репозитории загружены. Пользователь: {}",
                                        owner_login
                                    );
                                }
                                Err(e) => {
                                    self.status = format!("Ошибка загрузки репозиториев: {}", e)
                                }
                            }
                        }
                        Err(e) => {
                            self.status = format!("Ошибка получения пользователя: {}", e);
                            self.owner = None;
                        }
                    }
                }
            }

            ui.separator();

            ui.label("Ваши репозитории:");

            // Сбор репозиториев, которые пользователь хочет удалить
            let mut repos_to_delete = Vec::new();

            for repo in &self.repos {
                ui.horizontal(|ui| {
                    ui.label(repo);
                    if ui.button("Удалить").clicked() {
                        repos_to_delete.push(repo.clone());
                    }
                });
            }

            // Удаляем репозитории вне цикла (после borrow-immutable)
for repo_name in repos_to_delete {
    if let Some(owner) = &self.owner {
        match GitHubClient::new(self.token.clone()).delete_repo(owner, &repo_name) {
            Ok(_) => {
                self.status = format!("Репозиторий '{}' удалён", repo_name);
                self.repos.retain(|r| r != &repo_name);
            }
            Err(e) => self.status = format!("Ошибка удаления: {}", e),
        }
    } else {
        self.status = "Неизвестный владелец репозитория. Пожалуйста, загрузите репозитории заново.".to_string();
    }
}


            ui.separator();

            ui.label("Создать новый репозиторий:");
            ui.add(TextEdit::singleline(&mut self.new_repo).desired_width(250.0));
            if ui.button("Создать").clicked() {
                if self.new_repo.trim().is_empty() {
                    self.status = "Введите имя нового репозитория".to_string();
                } else {
                    match GitHubClient::new(self.token.clone()).create_repo(&self.new_repo) {
                        Ok(_) => {
                            self.status = format!("Репозиторий '{}' создан", self.new_repo);
                            self.repos.push(self.new_repo.clone());
                            self.new_repo.clear();
                        }
                        Err(e) => self.status = format!("Ошибка создания: {}", e),
                    }
                }
            }

            ui.separator();

            ui.label(&self.status);
        });
    }
}

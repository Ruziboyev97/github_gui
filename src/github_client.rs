use crate::models::Repo;
use reqwest::blocking::Client;
use serde_json::json;

pub struct GitHubClient {
    token: String,
    client: Client,
}

impl GitHubClient {
    pub fn new(token: String) -> Self {
        Self {
            token,
            client: Client::new(),
        }
    }

    pub fn list_repos(&self) -> Result<Vec<Repo>, String> {
        let res = self
            .client
            .get("https://api.github.com/user/repos")
            .header("User-Agent", "rust-gui")
            .bearer_auth(&self.token)
            .send()
            .map_err(|e| e.to_string())?;

        if res.status().is_success() {
            res.json::<Vec<Repo>>().map_err(|e| e.to_string())
        } else {
            Err(format!("Ошибка: {}", res.status()))
        }
    }

    pub fn create_repo(&self, name: &str) -> Result<(), String> {
        let body = json!({
            "name": name,
            "private": false
        });

        let res = self
            .client
            .post("https://api.github.com/user/repos")
            .header("User-Agent", "rust-gui")
            .bearer_auth(&self.token)
            .json(&body)
            .send()
            .map_err(|e| e.to_string())?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err(format!("Ошибка: {}", res.status()))
        }
    }

    pub fn delete_repo(&self, owner: &str, repo: &str) -> Result<(), String> {
        let url = format!("https://api.github.com/repos/{}/{}", owner, repo);

        let res = self
            .client
            .delete(&url)
            .header("User-Agent", "rust-gui")
            .bearer_auth(&self.token)
            .send()
            .map_err(|e| e.to_string())?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err(format!("Ошибка: {}", res.status()))
        }
    }
}

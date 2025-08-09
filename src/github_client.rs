use anyhow::Result;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct User {
    login: String,
}
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

    pub fn get_owner(&self) -> Result<String> {
        let url = "https://api.github.com/user";
        let user: User = self
            .client
            .get(url)
            .header("User-Agent", "rust-github-client")
            .bearer_auth(&self.token)
            .send()?
            .error_for_status()?
            .json()?;

        Ok(user.login)
    }

    pub fn list_repos(&self) -> Result<Vec<String>> {
        let url = "https://api.github.com/user/repos";
        let repos: Vec<Repo> = self
            .client
            .get(url)
            .header("User-Agent", "rust-github-client")
            .bearer_auth(&self.token)
            .send()?
            .error_for_status()?
            .json()?;

        Ok(repos.into_iter().map(|r| r.name).collect())
    }

    pub fn delete_repo(&self, owner: &str, repo_name: &str) -> Result<()> {
        let url = format!("https://api.github.com/repos/{}/{}", owner, repo_name);
        self.client
            .delete(&url)
            .header("User-Agent", "rust-github-client")
            .bearer_auth(&self.token)
            .send()?
            .error_for_status()?;

        Ok(())
    }

    pub fn create_repo(&self, name: &str) -> Result<()> {
        #[derive(Serialize)]
        struct NewRepo<'a> {
            name: &'a str,
        }

        let new_repo = NewRepo { name };

        let url = "https://api.github.com/user/repos";

        self.client
            .post(url)
            .header("User-Agent", "rust-github-client")
            .bearer_auth(&self.token)
            .json(&new_repo)
            .send()?
            .error_for_status()?;

        Ok(())
    }
}

#[derive(Deserialize)]
struct Repo {
    name: String,
}

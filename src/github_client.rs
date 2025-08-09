use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Repo {
    pub name: String,
    pub html_url: String,
}

pub struct GitHubClient;

impl GitHubClient {
    pub fn fetch_repos(user: &str) -> Vec<Repo> {
        let url = format!("https://api.github.com/users/{}/repos", user);
        let client = reqwest::blocking::Client::new();

        let res = client
            .get(&url)
            .header("User-Agent", "rust-github-client")
            .send();

        match res {
            Ok(resp) => {
                if let Ok(repos) = resp.json::<Vec<Repo>>() {
                    repos
                } else {
                    vec![]
                }
            }
            Err(_) => vec![],
        }
    }
}

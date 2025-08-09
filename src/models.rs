use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Repo {
    pub name: String,
    pub html_url: String,
}

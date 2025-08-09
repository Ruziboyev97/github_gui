use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub token: String,
}

fn config_path() -> PathBuf {
    let mut dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    dir.push("github_gui");
    fs::create_dir_all(&dir).ok();
    dir.push("config.json");
    dir
}

pub fn save_token(token: &str) -> std::io::Result<()> {
    let cfg = Config {
        token: token.to_string(),
    };
    let data = serde_json::to_string_pretty(&cfg).unwrap();
    fs::write(config_path(), data)
}

pub fn load_token() -> Option<String> {
    let path = config_path();
    if path.exists() {
        if let Ok(data) = fs::read_to_string(path) {
            if let Ok(cfg) = serde_json::from_str::<Config>(&data) {
                return Some(cfg.token);
            }
        }
    }
    None
}

pub fn delete_token() {
    let path = config_path();
    let _ = fs::remove_file(path);
}

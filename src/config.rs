use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default = "default_true")]
    pub show_banner: bool,
}

fn default_true() -> bool {
    true
}

fn config_path() -> PathBuf {
    let dir = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("pwatch");
    dir.join("config.toml")
}

pub fn load() -> Config {
    let path = config_path();
    if let Ok(content) = fs::read_to_string(&path) {
        toml::from_str(&content).unwrap_or_default()
    } else {
        Config { show_banner: true }
    }
}

pub fn save(config: &Config) -> Result<(), String> {
    let path = config_path();
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir).map_err(|e| format!("ディレクトリ作成失敗: {}", e))?;
    }
    let content = toml::to_string_pretty(config).map_err(|e| format!("シリアライズ失敗: {}", e))?;
    fs::write(&path, content).map_err(|e| format!("書き込み失敗: {}", e))?;
    Ok(())
}

use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub api_token: String,
}

pub fn get_config_path() -> PathBuf {
    let base_dirs = BaseDirs::new().expect("Failed to get base directories");
    base_dirs.config_dir().join("kanikani").join("config.toml")
}

pub fn load_config() -> Option<Config> {
    let config_file = get_config_path();
    if config_file.exists() {
        let config_str = fs::read_to_string(&config_file).expect("Failed to read config file");
        let config: Config = toml::from_str(&config_str).expect("Failed to parse config file");
        Some(config)
    } else {
        None
    }
}

pub fn save_config(config: &Config) {
    let config_file = get_config_path();
    let config_dir = config_file
        .parent()
        .expect("Failed to get config directory");
    fs::create_dir_all(config_dir).expect("Failed to create config directory");
    let config_str = toml::to_string(config).expect("Failed to serialize config");
    let mut file = fs::File::create(&config_file).expect("Failed to create config file");
    file.write_all(config_str.as_bytes())
        .expect("Failed to write config file");
}

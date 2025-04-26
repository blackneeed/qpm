use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref CONFIG_DIR: String = {
        let config_dir = dirs::config_dir().unwrap();
        config_dir.join("qpm").to_string_lossy().to_string()
    };
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub repos: Vec<String>,
}

impl Config {
    pub fn check_config_dir() -> Result<(), Box<dyn std::error::Error>> {
        let config_dir = dirs::config_dir().unwrap();
        let qpm_dir = config_dir.join("qpm");
        if !qpm_dir.exists() {
            std::fs::create_dir_all(&qpm_dir)?;
        }
        Ok(())
    }

    pub fn add_repo(repo: String) -> Result<(), Box<dyn std::error::Error>> {
        let mut config = Config::load()?;
        if !config.repos.contains(&repo) {
            config.repos.push(repo);
            Config::save(config)?;
        }
        Ok(())
    }

    pub fn load() -> Result<Config, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(&*CONFIG_DIR)?;
        let config: Config = serde_json::from_reader(file)?;
        Ok(config)
    }

    pub fn save(config: Config) -> Result<(), Box<dyn std::error::Error>> {
        let file = std::fs::File::create(&*CONFIG_DIR)?;
        serde_json::to_writer(file, &config)?;
        Ok(())
    }
}

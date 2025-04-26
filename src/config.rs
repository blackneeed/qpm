use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

lazy_static! {
    static ref CONFIG_FILE: PathBuf = {
        let config_dir = dirs::config_dir().unwrap();
        config_dir.join("qpm").join("config.json")
    };
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    pub repos: Vec<Repo>,
}

#[derive(PartialEq, Deserialize, Serialize, Clone)]
pub struct Repo {
    pub name: String,
    pub url: String,
}

impl Config {
    pub fn check_config_dir() -> Result<(), Box<dyn std::error::Error>> {
        let config_dir = CONFIG_FILE.parent().unwrap();
        if !config_dir.exists() {
            fs::create_dir_all(config_dir)?;
        }
        if !CONFIG_FILE.exists() {
            let default_config = Config { repos: Vec::new() };
            Config::save(&default_config)?;
        }
        Ok(())
    }

    pub fn add_repo(
        &mut self,
        name: String,
        url: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let repo = Repo { name, url };
        if !self.repos.contains(&repo) {
            self.repos.push(repo);
            self.save()?;
        }
        Ok(())
    }

    pub fn load() -> Result<Config, Box<dyn std::error::Error>> {
        Config::check_config_dir()?;
        let file = fs::File::open(&*CONFIG_FILE)?;
        let config: Config = serde_json::from_reader(file)?;
        Ok(config)
    }

    fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        Config::check_config_dir()?;
        let file = fs::File::create(&*CONFIG_FILE)?;
        serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }
}

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Repository {
    pub name: String,
    pub hash: String,
    pub path: String,
}

impl Repository {
    pub fn from(json: &str) -> Result<Vec<Repository>, Box<dyn std::error::Error>> {
        let repo: Vec<Repository> = serde_json::from_str(json)?;
        Ok(repo)
    }
}

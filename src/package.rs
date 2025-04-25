pub use serde::Deserialize;

#[derive(Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub description: String,
    pub platforms: Vec<Platform>,
}

#[derive(Deserialize)]
pub struct Platform {
    pub name: String,
    pub install_script_url: String,
}

pub fn read_package_file(json: &str) -> Result<Package, Box<dyn std::error::Error>> {
    let package: Package = serde_json::from_str(json)?;
    Ok(package)
}

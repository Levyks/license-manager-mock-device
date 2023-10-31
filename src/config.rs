use serde::{Deserialize};


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigDevice {
    pub license_id: String,
    pub start: Location,
    pub end: Location,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub base_url: String,
    pub interval: u64,
    pub speed: f64,
    pub devices: Vec<ConfigDevice>,
}

pub async fn read_config(path: &str) -> anyhow::Result<Config> {
    let config = tokio::fs::read_to_string(path).await?;
    let config: Config = serde_yaml::from_str(&config)?;
    Ok(config)
}
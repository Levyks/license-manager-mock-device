use serde::Serialize;
use crate::config::Location;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct HeartbeatRequest {
    license_id: String,
    latitude: f64,
    longitude: f64,
}

pub fn normalize_url(base_url: &str) -> String {
    if base_url.ends_with("/") {
        base_url[..base_url.len() - 1].to_string()
    } else {
        base_url.to_string()
    }
}

pub async fn send_heartbeat(base_url: &str, license_id: &str, location: &Location) -> anyhow::Result<()> {
    let url = format!("{}/v1/collect/heartbeat", normalize_url(base_url));
    let client = reqwest::Client::new();
    let request = HeartbeatRequest {
        license_id: license_id.to_string(),
        latitude: location.lat,
        longitude: location.lng,
    };
    client.post(url)
        .json(&request)
        .send()
        .await?;
    Ok(())
}
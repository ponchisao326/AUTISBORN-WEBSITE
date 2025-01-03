use serde::Deserialize;
use reqwest;
use crate::components::sensible_data;
use crate::components::sensible_data::SERVER_URL;

// Definir la estructura que coincide con los datos del JSON
#[derive(Deserialize, Clone)]
pub struct ServerStats {
    // CamelCase cause the .json is with this name ;)
    pub onlinePlayers: i16,
    pub totalDeaths: i32,
    pub daysPlayed: i32,
}

pub async fn fetch_server_stats() -> Result<ServerStats, reqwest::Error> {
    let url = SERVER_URL;
    let response = reqwest::get(url).await?;
    let server_stats: ServerStats = response.json().await?;
    Ok(server_stats)
}

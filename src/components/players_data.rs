use serde::Deserialize;
use reqwest;

// Para la estructura de los poderes de los jugadores
#[derive(Deserialize, Clone)]
pub struct Power {
    pub name: String,
    pub baseStrength: f32,
    pub totalStrength: f32,
}

#[derive(Deserialize, Clone)]
pub struct PlayerPowers {
    pub player: String,
    pub powers: Vec<Power>,
}

// Para la estructura de los efectos del jugador
#[derive(Deserialize, Clone)]
pub struct PlayerEffects {
    pub player: String,
    pub effectsInfo: i32,
}

// Función para obtener los poderes de un jugador desde una URL
pub async fn fetch_player_powers(url: &str) -> Result<PlayerPowers, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let player_powers: PlayerPowers = response.json().await?;
    Ok(player_powers)
}

// Función para obtener los efectos de un jugador desde una URL
pub async fn fetch_player_effects(url: &str) -> Result<PlayerEffects, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let player_effects: PlayerEffects = response.json().await?;
    Ok(player_effects)
}

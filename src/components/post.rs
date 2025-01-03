use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Post {
    pub title: String,
    pub content: String,
}

pub async fn fetch_posts() -> Result<Vec<Post>, reqwest::Error> {
    let url = format!(
        "https://ponchisaohosting.xyz/downloads/cosmere/assets/actualizaciones.json?ts={}",
        js_sys::Date::now()
    );
    let response = reqwest::get(&url).await?;
    let posts: Vec<Post> = response.json().await?;
    Ok(posts)
}

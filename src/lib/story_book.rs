use std::collections::HashMap;

use axum::http::Response;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use short_uuid::short;

use super::capture_screen_shots::ScreenShotParams;

#[derive(Serialize, Deserialize, Debug)]
#[serde_with::serde_as]
pub struct StoryBookConfig {
    pub v: i64,
    #[serde_as(as = "HashMap<serde_with::json::JsonString, _>")]
    pub entries: HashMap<String, StoryBookConfigEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StoryBookConfigEntry {
    pub id: String,
    pub name: String,
    pub title: String,
    pub r#type: String,
}

pub async fn get_story_book_config(url: &str) -> Result<StoryBookConfig, String> {
    tracing::info!("Fetching storybook config from: {}", url);
    let response = reqwest::get(format!("http://{}/index.json", url))
        .await
        .map_err(|e| e.to_string())?;

    let body = response.text().await.map_err(|e| e.to_string())?;

    let config: StoryBookConfig = serde_json::from_str(body.as_str())
        .map_err(|_| format!("Error parsing config from {}", { url }))?;

    Ok(config)
}

pub async fn get_screen_shot_params_by_url(url: String) -> Result<Vec<ScreenShotParams>, String> {
    let config = match get_story_book_config(url.as_str()).await {
        Ok(config) => {
            let config_filtered = config
                .entries
                .into_iter()
                .filter(|entry| entry.1.r#type == "story")
                .collect();

            StoryBookConfig {
                v: config.v,
                entries: config_filtered,
            }
        }
        Err(e) => return Err(e.to_string()),
    };

    Ok(get_screen_shot_params_from_config(config, url.as_str()))
}

fn get_screen_shot_params_from_config(config: StoryBookConfig, url: &str) -> Vec<ScreenShotParams> {
    let time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let random_folder_name: String = format!("{}-{}", time, short!());

    config
        .entries
        .into_iter()
        .map(|entry| ScreenShotParams {
            url: format!(
                "http://{}/iframe.html?args=&id={}&viewMode=story",
                url, entry.1.id
            ),
            id: entry.1.id,
            folder: random_folder_name.clone(),
        })
        .collect()
}

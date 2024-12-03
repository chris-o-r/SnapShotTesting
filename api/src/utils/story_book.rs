use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use anyhow::Error;

use super::capture_screenshots::ScreenShotParams;

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


pub async fn get_screenshot_params_by_url(
    url: &str,
    folder_name: &str,
) -> Result<Vec<ScreenShotParams>, Error> {
    let story_book_config = get_story_book_config(url).await.map_err(|err| {
        tracing::error!("Failed to get story book config for url {}\n{}", url, err);
        anyhow::Error::msg(format!("Failed to find story book config at: {}", url))

    })?;

    let config_filtered = story_book_config
        .entries
        .into_iter()
        .filter(|entry| entry.1.r#type == "story")
        .collect();

    Ok(get_screen_shot_params_from_config(
        StoryBookConfig {
            v: story_book_config.v,
            entries: config_filtered,
        },
        url,
        folder_name,
    ))
}

async fn get_story_book_config(url: &str) -> Result<StoryBookConfig, Error> {
    let response: reqwest::Response = reqwest::get(format!("http://{}/index.json", url)).await?;
    let body = response.text().await?;

    let config: StoryBookConfig = serde_json::from_str(body.as_str())?;

    Ok(config)
}


fn get_screen_shot_params_from_config(
    config: StoryBookConfig,
    url: &str,
    folder_name: &str,
) -> Vec<ScreenShotParams> {
    config
        .entries
        .into_iter()
        .map(|entry| ScreenShotParams {
            url: format!(
                "http://{}/iframe.html?args=&id={}&viewMode=story",
                url, entry.1.id
            ),
            id: entry.1.id,
            folder: folder_name.to_string(),
        })
        .collect()
}

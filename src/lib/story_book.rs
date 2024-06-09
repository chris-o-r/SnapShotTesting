use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use super::capture_screen_shots::{self, ScreenShotParams};

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
}

pub async fn get_story_book_config(url: &str) -> String {
    println!("Fetching storybook config from: {}", url);
    reqwest::get(url).await.unwrap().text().await.unwrap()
}

pub async fn get_snap_shots_by_url(
    url: String,
    folder_name: String,
) -> Vec<Result<String, std::io::Error>> {
    let config = serde_json::from_str::<StoryBookConfig>(
        get_story_book_config(format!("http:///{url}/index.json").as_str())
            .await
            .as_str(),
    )
    .unwrap();

    let urls: Vec<ScreenShotParams> =
        get_screen_shot_params_from_config(config, url.as_str(), folder_name);

    capture_screen_shots::capture_screen_shots(urls).await
}

fn get_screen_shot_params_from_config(
    config: StoryBookConfig,
    url: &str,
    folder: String,
) -> Vec<ScreenShotParams> {
    config
        .entries
        .into_values()
        .map(|entry| ScreenShotParams {
            url: format!(
                "http://{}/iframe.html?args=&id={}&viewMode=story",
                url, entry.id
            ),
            id: entry.id,
            folder: folder.clone(),
        })
        .collect()
}

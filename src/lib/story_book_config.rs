use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

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

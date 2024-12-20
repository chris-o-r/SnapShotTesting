use serde::{Deserialize, Serialize};

use crate::utils::save_images::safe_save_image;

use super::snapshot::SnapShotType;


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RawImage {
    pub raw_image: Vec<u8>,
    pub height: f64,
    pub width: f64,
    pub image_type: SnapShotType,
    pub image_name: String,
}

impl RawImage {
    pub fn save(self, folder: &str) -> Result<String, anyhow::Error> {
        safe_save_image(self.raw_image, folder, self.image_name.as_str())
    }
}
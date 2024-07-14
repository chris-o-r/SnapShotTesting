use chrono::NaiveDateTime;
use lib::{compare_images::CompareImagesReturn, date_format};
use sqlx::{postgres::PgRow, Row};
use uuid::Uuid;

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct SnapShotBatch {
    pub id: Uuid,
    pub name: String,
    #[serde(with = "date_format")]
    pub created_at: NaiveDateTime,
    pub new_story_book_version: String,
    pub old_story_book_version: String,
    pub diff_images_paths: CompareImagesReturn,
    pub new_images_paths: Vec<String>,
    pub old_images_paths: Vec<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct SnapShotBatchDTO {
    pub id: Uuid,
    pub name: String,
    #[serde(with = "date_format")]
    pub created_at: NaiveDateTime,
    pub new_story_book_version: String,
    pub old_story_book_version: String,
}

impl<'r> sqlx::FromRow<'r, PgRow> for SnapShotBatchDTO {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(SnapShotBatchDTO {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            created_at: row.try_get("created_at")?,
            new_story_book_version: row.try_get("new_story_book_version")?,
            old_story_book_version: row.try_get("old_story_book_version")?,
        })
    }
}

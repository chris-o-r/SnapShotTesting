use lib::date_format;
use sqlx::{postgres::PgRow, Row};

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct SnapShotBatch {
    pub id: String,
    pub name: String,
    #[serde(with = "date_format")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub new_story_book_version: String,
    pub old_story_book_version: String,
}

impl<'r> sqlx::FromRow<'r, PgRow> for SnapShotBatch {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(SnapShotBatch {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            created_at: row.try_get("created_at")?,
            new_story_book_version: row.try_get("new_story_book_version")?,
            old_story_book_version: row.try_get("old_story_book_version")?,
        })
    }
}

use lib::date_format;
use sqlx::{postgres::PgRow, Row};

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]

pub struct SnapShot {
    pub id: String,
    pub batch_id: String,
    pub name: String,
    pub path: String,
    pub snap_shot_type: String,
    #[serde(with = "date_format")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl<'r> sqlx::FromRow<'r, PgRow> for SnapShot {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(SnapShot {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            path: row.try_get("path")?,
            created_at: row.try_get("created_at")?,
            batch_id: row.try_get("batch_id")?,
            snap_shot_type: row.try_get("snap_shot_type")?,
        })
    }
}

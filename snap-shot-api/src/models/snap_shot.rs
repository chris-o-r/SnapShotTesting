use chrono::NaiveDateTime;
use lib::date_format;
use sqlx::{postgres::PgRow, Row};
use uuid::Uuid;

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct SnapShot {
    pub id: Uuid,
    pub batch_id: Uuid,
    pub name: String,
    pub path: String,
    pub snap_shot_type: String,
    #[serde(with = "date_format")]
    pub created_at: NaiveDateTime,
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

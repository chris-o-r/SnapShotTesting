use core::fmt;

use chrono::NaiveDateTime;
use lib::date_format;
use sqlx::{postgres::PgRow, Row};
use uuid::Uuid;

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, sqlx::Type)]
#[sqlx(type_name = "snap_shot_type", rename_all = "lowercase")]
pub enum SnapShotType {
    New,
    Old,
    Diff,
    Create,
    Deleted,
}

impl fmt::Display for SnapShotType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct SnapShot {
    pub id: Uuid,
    pub batch_id: Uuid,
    pub name: String,
    pub path: String,
    pub snap_shot_type: SnapShotType,
    #[serde(with = "date_format")]
    pub created_at: NaiveDateTime,
}

impl<'r> sqlx::FromRow<'r, PgRow> for SnapShot {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let snap_shot_type_dto: String = row.try_get("snap_shot_type")?;

        let snap_shot_type = match snap_shot_type_dto.as_str() {
            "New" => SnapShotType::New,
            "Old" => SnapShotType::Old,
            "Diff" => SnapShotType::Diff,
            "Create" => SnapShotType::Create,
            "Deleted" => SnapShotType::Deleted,
            _ => SnapShotType::New,
        };
        Ok(SnapShot {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            path: row.try_get("path")?,
            created_at: row.try_get("created_at")?,
            batch_id: row.try_get("batch_id")?,
            snap_shot_type: snap_shot_type,
        })
    }
}

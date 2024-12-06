use crate::utils::date_format;
use axum::{body::Body, http::Response, response::IntoResponse};
use chrono::NaiveDateTime;
use sqlx::{postgres::PgRow, Row};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, ToSchema)]
pub struct SnapShotBatchV2 {
    pub id: Uuid,
    pub name: String,
    #[serde(with = "date_format")]
    pub created_at: NaiveDateTime,
    pub new_story_book_version: String,
    pub old_story_book_version: String,
    pub created_image_paths: Vec<String>,
    pub deleted_image_paths: Vec<String>,
    pub diff_image: Vec<DiffImage>
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, ToSchema)]
pub struct DiffImage {
    pub diff: String, 
    pub new: String, 
    pub old: String
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

impl IntoResponse for SnapShotBatchV2 {
    fn into_response(self) -> Response<Body> {
        (
            axum::http::StatusCode::OK,
            serde_json::to_string(&self).unwrap_or_default(),
        )
            .into_response()
    }
}

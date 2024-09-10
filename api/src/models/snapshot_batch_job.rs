use crate::utils::date_format;
use axum::{body::Body, http::Response, response::IntoResponse};
use chrono::NaiveDateTime;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, Copy, ToSchema)]

pub struct SnapShotBatchJob {
    pub id: Uuid,
    pub snap_shot_batch_id: Option<Uuid>,
    pub status: SnapShotBatchJobStatus,
    pub progress: f32,
    #[serde(with = "date_format")]
    pub created_at: NaiveDateTime,
    #[serde(with = "date_format")]
    pub updated_at: NaiveDateTime,
}

#[derive(
    Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, sqlx::Type, Copy, ToSchema,
)]
#[sqlx(type_name = "snap_shot_type", rename_all = "lowercase")]
pub enum SnapShotBatchJobStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

impl IntoResponse for SnapShotBatchJob {
    fn into_response(self) -> Response<Body> {
        (
            axum::http::StatusCode::OK,
            serde_json::to_string(&self).unwrap_or_default(),
        )
            .into_response()
    }
}

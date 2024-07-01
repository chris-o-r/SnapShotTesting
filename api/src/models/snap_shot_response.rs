use axum::{body::Body, http::Response, response::IntoResponse};
use lib::compare_images::CompareImagesReturn;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct SnapShotResponse {
    pub id: Uuid,
    pub new_images_paths: Vec<String>,
    pub old_images_paths: Vec<String>,
    pub diff_images_paths: CompareImagesReturn,
}

impl IntoResponse for SnapShotResponse {
    fn into_response(self) -> Response<Body> {
        (
            axum::http::StatusCode::OK,
            serde_json::to_string(&self).unwrap_or_default(),
        )
            .into_response()
    }
}

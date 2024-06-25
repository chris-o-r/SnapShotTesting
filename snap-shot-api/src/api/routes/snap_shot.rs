use anyhow::Error;
use axum::body::Body;
use axum::http::{Response, StatusCode};
use axum::{extract, response::IntoResponse};
use lib::compare_images::CompareImagesReturn;
use lib::{
    capture_screen_shots::capture_screen_shots, compare_images,
    story_book::get_screen_shot_params_by_url,
};
use serde::{de, Deserialize, Deserializer, Serialize};
use std::sync::Arc;
use std::{fmt, str::FromStr, time};
use uuid::Uuid;

use crate::api::errors::AppError;
use crate::models::app_state::AppState;
use crate::models::snap_shot::SnapShot;
use axum::extract::State;

#[derive(Debug, Deserialize)]
pub struct SnapShotParams {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    new: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    old: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SnapShotResponse {
    pub id: String,
    pub new_images_paths: Vec<String>,
    pub old_images_paths: Vec<String>,
    pub diff_images_paths: CompareImagesReturn,
}

impl IntoResponse for SnapShotResponse {
    fn into_response(self) -> Response<Body> {
        (
            StatusCode::OK,
            serde_json::to_string(&self).unwrap_or_default(),
        )
            .into_response()
    }
}

/// Serde deserialization decorator to map empty Strings to None,
fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}

pub async fn handle_snap_shot(
    State(state): State<Arc<AppState>>,
    extract::Json(payload): extract::Json<SnapShotParams>,
) -> Result<SnapShotResponse, AppError> {
    let time_start = time::Instant::now();

    let new = match payload.new {
        Some(new) => new,
        None => {
            return Err(AppError(
                Error::msg("New URL is required"),
                StatusCode::BAD_REQUEST,
            ))
        }
    };
    let old = match payload.old {
        Some(old) => old,
        None => {
            return Err(AppError(
                Error::msg("New URL is required"),
                StatusCode::BAD_REQUEST,
            ));
        }
    };

    let id = uuid::Uuid::new_v4();
    let random_folder_name = format!(
        "{}-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        id
    );
    let images_1: Vec<String> =
        handle_snap_shot_for_url(&new, &random_folder_name.as_str(), "new").await?;

    let mut elapsed: time::Duration = time_start.elapsed();
    tracing::debug!("Elapsed time: {:?}", elapsed);

    let images_2: Vec<String> =
        handle_snap_shot_for_url(&old, random_folder_name.as_str(), "old").await?;

    elapsed = time_start.elapsed();

    tracing::debug!("Elapsed time: {:?}", elapsed);

    let diff_images = compare_images::compare_images(
        images_1.clone(),
        images_2.clone(),
        random_folder_name.as_str(),
    )
    .await;

    elapsed = time_start.elapsed();

    tracing::info!("Time taken: {:?}", elapsed);

    let result = SnapShotResponse {
        id: id.to_string(),
        new_images_paths: images_1,
        old_images_paths: images_2,
        diff_images_paths: diff_images.unwrap(),
    };

    save_results_to_db(&result, id.to_string().as_str(), state.db_pool.clone()).await?;

    Ok(result)
}

async fn save_results_to_db(
    snap_shot_response: &SnapShotResponse,
    batch_id: &str,
    db_pool: sqlx::Pool<sqlx::Postgres>,
) -> Result<(), Error> {
    let mut snap_shots = Vec::new();

    snap_shots.extend(
        snap_shot_response
            .new_images_paths
            .iter()
            .map(|path| SnapShot {
                id: short_uuid::short!().to_string(),
                batch_id: batch_id.to_string().clone(),
                name: path.clone(),
                path: path.clone(),
                snap_shot_type: "new".to_string(),
                created_at: chrono::Utc::now(),
            }),
    );
    snap_shots.extend(
        snap_shot_response
            .old_images_paths
            .iter()
            .map(|path| SnapShot {
                id: short_uuid::short!().to_string(),
                batch_id: batch_id.to_string().clone(),
                name: path.clone(),
                path: path.clone(),
                snap_shot_type: "old".to_string(),
                created_at: chrono::Utc::now(),
            }),
    );
    snap_shots.extend(
        snap_shot_response
            .diff_images_paths
            .created_images_paths
            .iter()
            .map(|path| SnapShot {
                id: short_uuid::short!().to_string(),
                batch_id: batch_id.to_string().clone(),
                name: path.clone(),
                path: path.clone(),
                snap_shot_type: "created".to_string(),
                created_at: chrono::Utc::now(),
            }),
    );
    snap_shots.extend(
        snap_shot_response
            .diff_images_paths
            .deleted_images_paths
            .iter()
            .map(|path| SnapShot {
                id: short_uuid::short!().to_string(),
                batch_id: batch_id.to_string().clone(),
                name: path.clone(),
                path: path.clone(),
                snap_shot_type: "deleted".to_string(),
                created_at: chrono::Utc::now(),
            }),
    );

    snap_shots.extend(
        snap_shot_response
            .diff_images_paths
            .diff_images_paths
            .iter()
            .map(|path| SnapShot {
                id: "-1".to_string(),
                batch_id: batch_id.to_string().clone(),
                name: path.clone(),
                path: path.clone(),
                snap_shot_type: "diff".to_string(),
                created_at: chrono::Utc::now(),
            }),
    );

    // save to db
    crate::db::snap_shot_store::insert_snap_shots(&db_pool, snap_shots).await?;

    Ok(())
}

async fn handle_snap_shot_for_url(
    url: &str,
    random_folder_name: &str,
    param_name: &str,
) -> Result<Vec<String>, Error> {
    let image_params = get_screen_shot_params_by_url(url, param_name).await?;

    capture_screen_shots(image_params, random_folder_name).await
}

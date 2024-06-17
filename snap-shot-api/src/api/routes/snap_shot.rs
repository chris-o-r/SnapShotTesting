use anyhow::Error;
use axum::body::Body;
use axum::http::{Response, StatusCode};
use axum::{extract::Query, response::IntoResponse};
use lib::compare_images::CompareImagesReturn;
use lib::{
    capture_screen_shots::capture_screen_shots, compare_images,
    story_book::get_screen_shot_params_by_url,
};
use serde::{de, Deserialize, Deserializer, Serialize};
use std::{fmt, str::FromStr, time};

use crate::api::errors::AppError;

#[derive(Debug, Deserialize)]
pub struct SnapShotParams {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    new: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    old: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SnapShotResponse {
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
    Query(params): Query<SnapShotParams>,
) -> Result<SnapShotResponse, AppError> {
    let time_start = time::Instant::now();

    let new = match params.new {
        Some(new) => new,
        None => {
            return Err(AppError(
                Error::msg("New URL is required"),
                StatusCode::BAD_REQUEST,
            ))
        }
    };
    let old = match params.old {
        Some(old) => old,
        None => {
            return Err(AppError(
                Error::msg("New URL is required"),
                StatusCode::BAD_REQUEST,
            ));
        }
    };

    tracing::debug!("Getting snapshots for new: {}", new,);

    let random_folder_name = format!(
        "{}-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        short_uuid::short!()
    );
    let images_1: Vec<String> =
        handle_snap_shot_for_url(&new, &random_folder_name.as_str(), "new").await?;

    let mut elapsed: time::Duration = time_start.elapsed();
    tracing::debug!("Elapsed time: {:?}", elapsed);
    tracing::debug!("Getting snapshots for old: {}", old);

    let images_2: Vec<String> =
        handle_snap_shot_for_url(&old, random_folder_name.as_str(), "old").await?;

    elapsed = time_start.elapsed();

    tracing::info!("Elapsed time: {:?}", elapsed);
    tracing::info!("Comparing images");
    let diff_images = compare_images::compare_images(
        images_1.clone(),
        images_2.clone(),
        random_folder_name.as_str(),
    )
    .await;

    elapsed = time_start.elapsed();

    tracing::info!("Time taken: {:?}", elapsed);

    let result = SnapShotResponse {
        new_images_paths: images_1,
        old_images_paths: images_2,
        diff_images_paths: diff_images.unwrap(),
    };

    Ok(result)
}

async fn handle_snap_shot_for_url(
    url: &str,
    random_folder_name: &str,
    param_name: &str,
) -> Result<Vec<String>, Error> {
    let image_params = get_screen_shot_params_by_url(url, param_name).await?;

    capture_screen_shots(image_params, random_folder_name).await
}

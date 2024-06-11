use axum::{body::Body, extract::Query, http::Response, response::IntoResponse};
use lib::{
    capture_screen_shots::capture_screen_shots, compare_images,
    story_book::get_screen_shot_params_by_url,
};
use serde::{de, Deserialize, Deserializer, Serialize};
use std::{fmt, str::FromStr, time};

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
    pub diff_images_paths: Vec<String>,
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

pub async fn handle_snap_shot(Query(params): Query<SnapShotParams>) -> impl IntoResponse {
    let time_start = time::Instant::now();

    let new = match params.new {
        Some(new) => new,
        None => {
            return Response::new(Body::from("New URL is required"));
        }
    };
    let old = match params.old {
        Some(old) => old,
        None => {
            return Response::new(Body::from("Old URL is required"));
        }
    };
    tracing::info!("Getting snapshots for new: {}", new,);

    let image_1_params = match get_screen_shot_params_by_url(new).await {
        Ok(images) => images,
        Err(e) => {
            return Response::new(Body::from(format!(
                "Error getting screen shots for new: {}",
                e
            )))
        }
    };

    let images_1: Vec<String> = capture_screen_shots(image_1_params)
        .await
        .into_iter()
        .map(|res| res.unwrap())
        .collect();

    let mut elapsed: time::Duration = time_start.elapsed();
    tracing::info!("Elapsed time: {:?}", elapsed);
    tracing::info!("Getting snapshots for old: {}", old);

    let image_2_params = match get_screen_shot_params_by_url(old).await {
        Ok(images) => images,
        Err(e) => {
            return Response::new(Body::from(format!(
                "Error getting screen shots for old: {}",
                e
            )))
        }
    };
    let images_2: Vec<String> = capture_screen_shots(image_2_params)
        .await
        .into_iter()
        .map(|res| res.unwrap())
        .collect();

    elapsed = time_start.elapsed();

    tracing::info!("Elapsed time: {:?}", elapsed);
    tracing::info!("Comparing images");
    let diff_images = compare_images::compare_images(images_1.clone(), images_2.clone()).await;

    elapsed = time_start.elapsed();

    tracing::info!("Time taken: {:?}", elapsed);

    let result = SnapShotResponse {
        new_images_paths: images_1,
        old_images_paths: images_2,
        diff_images_paths: diff_images.unwrap(),
    };

    Response::new(Body::from(serde_json::to_string(&result).unwrap()))
}

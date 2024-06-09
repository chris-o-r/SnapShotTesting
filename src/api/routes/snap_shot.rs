use axum::{body::Body, extract::Query, http::Response, response::IntoResponse};
use serde::{de, Deserialize, Deserializer};
use std::{fmt, str::FromStr, time};

use crate::lib::{compare_images, story_book::get_snap_shots_by_url}; // 0.11.14

#[derive(Debug, Deserialize)]
pub struct SnapShotParams {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    new: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    old: Option<String>,
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
    println!("Getting snapshots for new: {}", new,);
    let images_new = get_snap_shots_by_url(new, "new".to_string())
        .await
        .into_iter()
        .map(|res| res.unwrap())
        .collect();

    let mut elapsed = time_start.elapsed();
    println!("Elapsed time: {:?}", elapsed);
    println!("Getting snapshots for old: {}", old);

    let images_old = get_snap_shots_by_url(old, "old".to_string())
        .await
        .into_iter()
        .map(|res| res.unwrap())
        .collect();

    elapsed = time_start.elapsed();
    println!("Elapsed time: {:?}", elapsed);
    println!("Comparing images");
    let _ = compare_images::compare_images(images_new, images_old).await;

    elapsed = time_start.elapsed();

    println!("Time taken: {:?}", elapsed);

    Response::new(Body::from("Snapshots taken"))
}

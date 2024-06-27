use anyhow::Error;
use chrono::{DateTime, Utc};
use lib::{
    capture_screen_shots::capture_screen_shots,
    compare_images::{self, CompareImagesReturn},
    story_book::get_screen_shot_params_by_url,
};

use crate::{
    api::routes::snap_shot::SnapShotResponse,
    db::{snap_shot_batch_store, snap_shot_store},
    models::{snap_shot::SnapShot, snap_shot_batch::SnapShotBatch},
};

pub async fn create_snap_shots(
    new_url: &str,
    old_url: &str,
    db_pool: sqlx::Pool<sqlx::Postgres>,
) -> Result<SnapShotResponse, Error> {
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
        handle_snap_shot_for_url(&new_url, &random_folder_name.as_str(), "new").await?;

    let images_2: Vec<String> =
        handle_snap_shot_for_url(&old_url, random_folder_name.as_str(), "old").await?;

    let diff_images = compare_images::compare_images(
        images_1.clone(),
        images_2.clone(),
        random_folder_name.as_str(),
    )
    .await?;

    save_results_to_db(
        diff_images.clone(),
        images_1.clone(),
        images_2.clone(),
        id.to_string().as_str(),
        new_url,
        old_url,
        db_pool,
    )
    .await?;

    Ok(SnapShotResponse {
        id: id.to_string(),
        new_images_paths: images_1,
        old_images_paths: images_2,
        diff_images_paths: diff_images,
    })
}

async fn save_results_to_db(
    diff_images: CompareImagesReturn,
    new_images: Vec<String>,
    old_images: Vec<String>,
    batch_id: &str,
    new_url: &str,
    old_url: &str,
    db_pool: sqlx::Pool<sqlx::Postgres>,
) -> Result<(), Error> {
    let snap_shots = create_snap_shot_array(diff_images, new_images, old_images, batch_id);

    snap_shot_store::insert_snap_shots(&db_pool, snap_shots).await?;
    tracing::info!("Snap shots saved to db {}", batch_id);

    snap_shot_batch_store::save_snap_shot_batch(
        &db_pool,
        SnapShotBatch {
            id: batch_id.to_string(),
            created_at: Utc::now(),
            name: "snap-shot".to_string(),
            new_story_book_version: new_url.to_string(),
            old_story_book_version: old_url.to_string(),
        },
    )
    .await?;

    Ok(())
}

fn create_snap_shot_array(
    diff_images: CompareImagesReturn,
    new_images: Vec<String>,
    old_images: Vec<String>,
    batch_id: &str,
) -> Vec<SnapShot> {
    let mut snap_shots = Vec::new();

    snap_shots.extend(paths_to_snap_shot(
        batch_id,
        diff_images.diff_images_paths,
        "diff",
        Utc::now(),
    ));

    snap_shots.extend(paths_to_snap_shot(
        batch_id,
        diff_images.deleted_images_paths,
        "deleted",
        Utc::now(),
    ));

    snap_shots.extend(paths_to_snap_shot(
        batch_id,
        diff_images.created_images_paths,
        "created",
        Utc::now(),
    ));

    snap_shots.extend(paths_to_snap_shot(batch_id, new_images, "new", Utc::now()));

    snap_shots.extend(paths_to_snap_shot(batch_id, old_images, "old", Utc::now()));

    snap_shots
}

fn paths_to_snap_shot(
    batch_id: &str,
    paths: Vec<String>,
    snap_shot_type: &str,
    created_at: DateTime<Utc>,
) -> Vec<SnapShot> {
    paths
        .iter()
        .map(|path| SnapShot {
            id: short_uuid::short!().to_string(),
            batch_id: batch_id.to_string().clone(),
            name: path.clone(),
            path: path.clone(),
            snap_shot_type: snap_shot_type.to_string(),
            created_at: created_at,
        })
        .collect()
}

async fn handle_snap_shot_for_url(
    url: &str,
    random_folder_name: &str,
    param_name: &str,
) -> Result<Vec<String>, Error> {
    let image_params = get_screen_shot_params_by_url(url, param_name).await?;

    capture_screen_shots(image_params, random_folder_name).await
}

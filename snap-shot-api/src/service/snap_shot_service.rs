use anyhow::Error;
use chrono::{NaiveDateTime, Utc};
use lib::{
    capture_screen_shots::capture_screen_shots,
    compare_images::{self, CompareImagesReturn},
    story_book::get_screen_shot_params_by_url,
};
use uuid::Uuid;

use crate::{
    db::{
        snap_shot_batch_store,
        snap_shot_store::{self, get_all_snap_shots_by_batch_id},
    },
    models::{
        snap_shot::SnapShot, snap_shot_batch::SnapShotBatch, snap_shot_response::SnapShotResponse,
    },
};

pub async fn create_snap_shots(
    new_url: &str,
    old_url: &str,
    db_pool: sqlx::Pool<sqlx::Postgres>,
) -> Result<SnapShotResponse, Error> {
    let mut transaction: sqlx::Transaction<'_, sqlx::Postgres> = db_pool.begin().await?;

    let saved_batch = snap_shot_batch_store::insert_snap_shot_batch(
        &mut transaction,
        SnapShotBatch {
            id: uuid::Uuid::new_v4(),
            created_at: Utc::now().naive_utc(),
            name: "snap-shot".to_string(),
            new_story_book_version: new_url.to_string(),
            old_story_book_version: old_url.to_string(),
        },
    )
    .await?;

    let random_folder_name = format!(
        "{}-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        &saved_batch.id
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

    let snap_shots = create_snap_shot_array(
        diff_images.clone(),
        images_1.clone(),
        images_2.clone(),
        &saved_batch.id,
    );

    snap_shot_store::insert_snap_shots(&mut transaction, snap_shots).await?;

    transaction.commit().await?;

    Ok(SnapShotResponse {
        id: saved_batch.id.clone(),
        new_images_paths: images_1,
        old_images_paths: images_2,
        diff_images_paths: diff_images,
    })
}

fn create_snap_shot_array(
    diff_images: CompareImagesReturn,
    new_images: Vec<String>,
    old_images: Vec<String>,
    batch_id: &Uuid,
) -> Vec<SnapShot> {
    let mut snap_shots = Vec::new();

    snap_shots.extend(paths_to_snap_shot(
        batch_id,
        diff_images.diff_images_paths,
        "diff",
        Utc::now().naive_utc(),
    ));

    snap_shots.extend(paths_to_snap_shot(
        batch_id,
        diff_images.deleted_images_paths,
        "deleted",
        Utc::now().naive_utc(),
    ));

    snap_shots.extend(paths_to_snap_shot(
        batch_id,
        diff_images.created_images_paths,
        "created",
        Utc::now().naive_utc(),
    ));

    snap_shots.extend(paths_to_snap_shot(
        batch_id,
        new_images,
        "new",
        Utc::now().naive_utc(),
    ));

    snap_shots.extend(paths_to_snap_shot(
        batch_id,
        old_images,
        "old",
        Utc::now().naive_utc(),
    ));

    snap_shots
}

fn paths_to_snap_shot(
    batch_id: &Uuid,
    paths: Vec<String>,
    snap_shot_type: &str,
    created_at: NaiveDateTime,
) -> Vec<SnapShot> {
    paths
        .iter()
        .map(|path| SnapShot {
            id: Uuid::new_v4(),
            batch_id: batch_id.clone(),
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

pub async fn get_snap_shot_history(
    db_pool: sqlx::Pool<sqlx::Postgres>,
) -> Result<Vec<SnapShotResponse>, Error> {
    let mut result: Vec<SnapShotResponse> = Vec::new();
    let snap_shot_batches = snap_shot_batch_store::get_all_snap_shot_batches(&db_pool).await?;

    for batch in snap_shot_batches {
        let snap_shots = get_all_snap_shots_by_batch_id(&db_pool, &batch.id).await?;
        let mut new_images = Vec::new();
        let mut old_images = Vec::new();
        let mut diff_images = Vec::new();

        for snap_shot in snap_shots {
            match snap_shot.snap_shot_type.as_str() {
                "new" => new_images.push(snap_shot.path.clone()),
                "old" => old_images.push(snap_shot.path.clone()),
                "diff" => diff_images.push(snap_shot.path.clone()),
                _ => (),
            }
        }

        result.push(SnapShotResponse {
            id: batch.id.clone(),
            new_images_paths: new_images,
            old_images_paths: old_images,
            diff_images_paths: CompareImagesReturn {
                created_images_paths: Vec::new(),
                deleted_images_paths: Vec::new(),
                diff_images_paths: diff_images,
            },
        });
    }

    Ok(result)
}

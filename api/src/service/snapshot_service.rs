use anyhow::Error;
use chrono::{NaiveDateTime, Utc};
use lib::{
    capture_screenshots,
    compare_images::{self, CompareImagesReturn},
    story_book::get_screenshot_params_by_url,
};
use uuid::Uuid;

use crate::{
    db::{
        snapshot_batch_job_store, snap_shot_batch_store,
        snapshot_store::{self},
    },
    models::{
        snapshot::{SnapShot, SnapShotType},
        snapshot_batch_job::{SnapShotBatchJob, SnapShotBatchJobStatus},
        snapshot_batch::{SnapShotBatch, SnapShotBatchDTO},
    },
};

pub async fn create_snap_shots(
    new_url: &str,
    old_url: &str,
    db_pool: sqlx::Pool<sqlx::Postgres>,
    redis_pool: bb8_redis::bb8::Pool<bb8_redis::RedisConnectionManager>,
) -> Result<SnapShotBatch, Error> {
    let mut job: SnapShotBatchJob = create_batch_job(&redis_pool).await?;

    let new_url = new_url.to_string();
    let old_url = old_url.to_string();

    let mut transaction: sqlx::Transaction<'_, sqlx::Postgres> = db_pool.begin().await?;

    let batch = snap_shot_batch_store::insert_snap_shot_batch(
        &mut transaction,
        SnapShotBatchDTO {
            id: job.id.clone(),
            created_at: Utc::now().naive_utc(),
            name: format!("{}-{}", new_url, old_url),
            new_story_book_version: new_url.clone().to_string(),
            old_story_book_version: old_url.clone().to_string(),
        },
    )
    .await?;

    job.snap_shot_batch_id = Some(batch.id.clone());
    job.updated_at = Utc::now().naive_utc();
    job.progress = 0.1;
    snapshot_batch_job_store::insert_snapshot_batch_job(&redis_pool, job.clone()).await?;

    let random_folder_name = format!(
        "{}-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        &batch.id.clone(),
    );

    let images_1: Vec<String> =
        handle_snap_shot_for_url(&new_url, &random_folder_name.as_str(), "new").await?;

    job.updated_at = Utc::now().naive_utc();
    job.progress = 0.4;
    snapshot_batch_job_store::insert_snapshot_batch_job(&redis_pool, job.clone()).await?;

    let images_2: Vec<String> =
        handle_snap_shot_for_url(&old_url, random_folder_name.as_str(), "old").await?;

    job.updated_at = Utc::now().naive_utc();
    job.progress = 0.7;

    snapshot_batch_job_store::insert_snapshot_batch_job(&redis_pool, job.clone()).await?;

    let diff_images = compare_images::compare_images(
        images_1.clone(),
        images_2.clone(),
        random_folder_name.as_str(),
    )
    .await?;

    let snap_shots = create_snapshot_array(
        diff_images.clone(),
        images_1.clone(),
        images_2.clone(),
        &batch.id.clone(),
    );

    snapshot_store::insert_snapshots(&mut transaction, snap_shots).await?;

    transaction.commit().await?;

    job.status = SnapShotBatchJobStatus::Completed;
    job.progress = 1.0;
    job.updated_at = Utc::now().naive_utc();
    snapshot_batch_job_store::insert_snapshot_batch_job(&redis_pool, job.clone()).await?;

    Ok(SnapShotBatch {
        id: batch.id,
        name: batch.name,
        created_at: batch.created_at,
        new_story_book_version: batch.new_story_book_version,
        old_story_book_version: batch.old_story_book_version,
        diff_images_paths: diff_images,
        new_images_paths: images_1,
        old_images_paths: images_2,
    })
}

fn create_snapshot_array(
    diff_images: CompareImagesReturn,
    new_images: Vec<String>,
    old_images: Vec<String>,
    batch_id: &Uuid,
) -> Vec<SnapShot> {
    let mut snap_shots = Vec::new();

    snap_shots.extend(paths_to_snap_shot(
        batch_id,
        diff_images.diff_images_paths,
        SnapShotType::Diff,
        Utc::now().naive_utc(),
    ));

    snap_shots.extend(paths_to_snap_shot(
        batch_id,
        diff_images.deleted_images_paths,
        SnapShotType::Deleted,
        Utc::now().naive_utc(),
    ));

    snap_shots.extend(paths_to_snap_shot(
        batch_id,
        diff_images.created_images_paths,
        SnapShotType::Create,
        Utc::now().naive_utc(),
    ));

    snap_shots.extend(paths_to_snap_shot(
        batch_id,
        new_images,
        SnapShotType::New,
        Utc::now().naive_utc(),
    ));

    snap_shots.extend(paths_to_snap_shot(
        batch_id,
        old_images,
        SnapShotType::Old,
        Utc::now().naive_utc(),
    ));

    snap_shots
}

fn paths_to_snap_shot(
    batch_id: &Uuid,
    paths: Vec<String>,
    snap_shot_type: SnapShotType,
    created_at: NaiveDateTime,
) -> Vec<SnapShot> {
    paths
        .iter()
        .map(|path| SnapShot {
            id: Uuid::new_v4(),
            batch_id: batch_id.clone(),
            name: path.clone(),
            path: path.clone(),
            snap_shot_type: snap_shot_type.clone(),
            created_at: created_at,
        })
        .collect()
}

async fn handle_snap_shot_for_url(
    url: &str,
    random_folder_name: &str,
    param_name: &str,
) -> Result<Vec<String>, Error> {
    tracing::info!("Capturing screen shots for url: {}", url);

    let image_params = get_screenshot_params_by_url(url, param_name).await?;

    let results =
        capture_screenshots::capture_screenshots(&image_params, random_folder_name).await?;

    tracing::info!(
        "Captured {}/{} for url {}",
        results.len(),
        image_params.len(),
        url
    );

    Ok(results)
}

async fn create_batch_job(
    redis_pool: &bb8_redis::bb8::Pool<bb8_redis::RedisConnectionManager>,
) -> Result<SnapShotBatchJob, Error> {
    let snap_shot_batch_job = SnapShotBatchJob {
        id: Uuid::new_v4(),
        snap_shot_batch_id: None,
        progress: 0.0,
        status: SnapShotBatchJobStatus::Pending,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    snapshot_batch_job_store::insert_snapshot_batch_job(redis_pool, snap_shot_batch_job.clone())
        .await?;

    Ok(snap_shot_batch_job)
}

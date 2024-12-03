use crate::utils::{
    capture_screenshots,
    compare_images::{self, CompareImagesReturn},
    env_variables,
    story_book::get_screenshot_params_by_url,
};
use anyhow::Error;
use chrono::{NaiveDateTime, Utc};
use uuid::Uuid;

use crate::{
    db::{
        snap_shot_batch_store, snapshot_batch_job_store,
        snapshot_store::{self},
    },
    models::{
        snapshot::{SnapShot, SnapShotType},
        snapshot_batch::{SnapShotBatch, SnapShotBatchDTO},
        snapshot_batch_job::{SnapShotBatchJob, SnapShotBatchJobStatus},
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

    let images_1: Vec<Result<String, Error>> = match handle_snap_shot_for_url(&new_url, &random_folder_name.as_str(), "new").await {
        Ok(res) => res, 
        Err(err) => {
            job.updated_at = Utc::now().naive_utc();
            job.status = SnapShotBatchJobStatus::Failed;
            snapshot_batch_job_store::insert_snapshot_batch_job(&redis_pool, job.clone()).await?;

            return Err(err);
        } 
    };
        

    job.updated_at = Utc::now().naive_utc();
    job.progress = 0.4;
    snapshot_batch_job_store::insert_snapshot_batch_job(&redis_pool, job.clone()).await?;

    let images_2: Vec<Result<String, Error>> = match
        handle_snap_shot_for_url(&old_url, random_folder_name.as_str(), "old").await {
            Ok(res) => res,
            Err(err) => {
                job.updated_at = Utc::now().naive_utc();
                job.status = SnapShotBatchJobStatus::Failed;
                snapshot_batch_job_store::insert_snapshot_batch_job(&redis_pool, job.clone()).await?;
    
                return Err(err);
            }
        };

    job.updated_at = Utc::now().naive_utc();
    job.progress = 0.7;

    snapshot_batch_job_store::insert_snapshot_batch_job(&redis_pool, job.clone()).await?;

    /*
       Remove images that are not valid from each list
       If an image in array 1 is not valid, remove it
       from array 2 at the same index and vice versa
    */

    let mut images_1_cleaned: Vec<String> = vec![];
    let mut images_2_cleaned: Vec<String> = vec![];

    images_1
        .into_iter()
        .zip(images_2.iter())
        .for_each(|(image_1, image_2)| match (image_1, image_2) {
            (Ok(img_1), Ok(img_2)) => {
                images_1_cleaned.push(img_1);
                images_2_cleaned.push(String::from(img_2));
            }
            (Err(_), Ok(img_2)) => {
                images_2_cleaned.push(String::from(img_2));
            }
            (Ok(img_1), Err(_)) => {
                images_1_cleaned.push(String::from(img_1));
            }
            (Err(_), Err(_)) => {
                // Do nothing
            }
        });

    tracing::info!("Comparing images");
    let diff_images = match compare_images::compare_images(
        images_1_cleaned.clone(),
        images_2_cleaned.clone(),
        random_folder_name.as_str(),
    )
    .await {
        Ok(res) => res, 
        Err(err) => {
            job.updated_at = Utc::now().naive_utc();
            job.status = SnapShotBatchJobStatus::Failed;
            snapshot_batch_job_store::insert_snapshot_batch_job(&redis_pool, job.clone()).await?;

            return Err(err);
        }
    };

    let snap_shots = create_snapshot_array(
        diff_images.clone(),
        images_1_cleaned.clone(),
        images_2_cleaned.clone(),
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
        new_images_paths: images_1_cleaned,
        old_images_paths: images_2_cleaned,
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
    let asset_folder = env_variables::EnvVariables::new().assets_folder;

    paths
        .iter()
        .map(|path| SnapShot {
            id: Uuid::new_v4(),
            batch_id: batch_id.clone(),
            name: path.clone(),
            path: path.replace(&asset_folder, "assets"),
            snap_shot_type: snap_shot_type.clone(),
            created_at: created_at,
        })
        .collect()
}

async fn handle_snap_shot_for_url(
    url: &str,
    random_folder_name: &str,
    param_name: &str,
) -> Result<Vec<Result<String, Error>>, Error> {
    tracing::debug!("Capturing screen shots for url: {}", url);

    let image_params = get_screenshot_params_by_url(url, param_name).await?;

    let results =
        capture_screenshots::capture_screenshots(&image_params, random_folder_name).await?;

    let num_ok_results = results.iter().filter(|r| r.is_ok()).count();
    tracing::debug!(
        "Captured {}/{} for url {}",
        num_ok_results,
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

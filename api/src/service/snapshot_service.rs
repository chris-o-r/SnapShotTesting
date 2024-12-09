use std::path::Path;

use crate::{
    models::snapshot_batch_v2::{DiffImage, SnapShotBatchV2},
    utils::{
        capture_screenshots::{self, RawImage},
        compare_images::{self},
        env_variables,
        save_images::safe_save_image,
        story_book::get_screenshot_params_by_url,
    },
};
use anyhow::Error;
use chrono::Utc;
use uuid::Uuid;

use crate::{
    db::{
        snap_shot_batch_store,
        snapshot_store::{self},
    },
    models::{
        snapshot::{SnapShot, SnapShotType},
        snapshot_batch::SnapShotBatchDTO,
    },
};

pub async fn create_snap_shots(
    new_url: &str,
    old_url: &str,
    db_pool: sqlx::Pool<sqlx::Postgres>,
) -> Result<SnapShotBatchV2, Error> {
    let asset_folder = env_variables::EnvVariables::new().assets_folder;

    let new_url = new_url.to_string();
    let old_url = old_url.to_string();

    let mut transaction: sqlx::Transaction<'_, sqlx::Postgres> = db_pool.begin().await?;

    let batch = snap_shot_batch_store::insert_snap_shot_batch(
        &mut transaction,
        SnapShotBatchDTO {
            id: Uuid::new_v4(),
            created_at: Utc::now().naive_utc(),
            name: format!("{}-{}", new_url, old_url),
            new_story_book_version: new_url.clone().to_string(),
            old_story_book_version: old_url.clone().to_string(),
        },
    )
    .await?;

    let random_folder_name = format!(
        "{}-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        &batch.id.clone(),
    );

    let images_1: Vec<Result<RawImage, Error>> =
        handle_snap_shot_for_url(&new_url, SnapShotType::New).await?;

    let images_2: Vec<Result<RawImage, Error>> =
        handle_snap_shot_for_url(&old_url, SnapShotType::Old).await?;
    /*
       Remove images that are not valid from each list
       If an image in array 1 is not valid, remove it
       from array 2 at the same index and vice versa
    */

    let mut images_1_cleaned: Vec<RawImage> = vec![];
    let mut images_2_cleaned: Vec<RawImage> = vec![];

    images_1
        .into_iter()
        .zip(images_2.iter())
        .for_each(|(image_1, image_2)| match (image_1, image_2) {
            (Ok(img_1), Ok(img_2)) => {
                images_1_cleaned.push(img_1);
                images_2_cleaned.push(img_2.clone());
            }
            (Err(_), Ok(img_2)) => {
                images_2_cleaned.push(img_2.clone());
            }
            (Ok(img_1), Err(_)) => {
                images_1_cleaned.push(img_1);
            }
            (Err(_), Err(_)) => {
                // Do nothing
            }
        });

    tracing::info!("Comparing images");
    let diff_images = compare_images::compare_images(
        images_1_cleaned.clone(),
        images_2_cleaned.clone(),
        random_folder_name.as_str(),
    )
    .await?;

    let batch = SnapShotBatchV2 {
        id: batch.id,
        name: batch.name,
        created_at: batch.created_at,
        new_story_book_version: batch.new_story_book_version,
        old_story_book_version: batch.old_story_book_version,
        created_image_paths: diff_images.created_images_paths,
        deleted_image_paths: diff_images.deleted_images_paths,
        diff_image: diff_images
            .diff_images_paths
            .clone()
            .into_iter()
            .filter_map(|snap| {
                let image_name = Path::new(&snap)
                    .file_stem()
                    .and_then(|os_str| os_str.to_str())
                    .unwrap();

                let old_image = images_1_cleaned
                    .clone()
                    .into_iter()
                    .find(|item| {
                        return item.image_name == image_name;
                    })
                    .unwrap();

                let new_image = images_1_cleaned
                    .clone()
                    .into_iter()
                    .find(|item| {
                        return item.image_name == image_name;
                    })
                    .unwrap();

                let image_new_name = safe_save_image(
                    new_image.raw_image,
                    format!("{}/new", random_folder_name).as_str(),
                    &new_image.image_name,
                );
                let image_old_name = safe_save_image(
                    old_image.raw_image,
                    format!("{}/old", random_folder_name).as_str(),
                    &old_image.image_name,
                );

                Some(DiffImage {
                    new: image_new_name.unwrap(),
                    old: image_old_name.unwrap(),
                    diff: snap.clone(),
                })
            })
            .collect(),
    };

    snapshot_store::insert_snapshots(
        &mut transaction,
        create_snapshot_array(batch.clone(), &asset_folder),
    )
    .await?;

    transaction.commit().await?;

    Ok(batch)
}

fn create_snapshot_array(batch: SnapShotBatchV2, asset_folder: &str) -> Vec<SnapShot> {
    let mut snap_shots = Vec::new();

    snap_shots.extend(batch.created_image_paths.iter().map(|item| SnapShot {
        id: uuid::Uuid::new_v4(),
        created_at: Utc::now().naive_utc(),
        batch_id: batch.id,
        path: item.replace(asset_folder, "assets").to_string(),
        snap_shot_type: SnapShotType::Create,
        name: item.split('/').last().unwrap().to_string(),
    }));

    snap_shots.extend(batch.deleted_image_paths.iter().map(|item| SnapShot {
        id: uuid::Uuid::new_v4(),
        created_at: Utc::now().naive_utc(),
        batch_id: batch.id,
        path: item.replace(asset_folder, "assets").to_string(),
        snap_shot_type: SnapShotType::Deleted,
        name: item.split('/').last().unwrap().to_string(),
    }));

    snap_shots.extend(
        batch
            .diff_image
            .iter()
            .map(|item| item.diff.clone())
            .map(|item| SnapShot {
                id: uuid::Uuid::new_v4(),
                created_at: Utc::now().naive_utc(),
                batch_id: batch.id,
                path: item.replace(asset_folder, "assets").to_string(),
                snap_shot_type: SnapShotType::Diff,
                name: item
                    .replace(asset_folder, "assets")
                    .split('/')
                    .last()
                    .unwrap()
                    .to_string(),
            }),
    );

    snap_shots.extend(
        batch
            .diff_image
            .iter()
            .map(|item| item.new.clone())
            .map(|item| SnapShot {
                id: uuid::Uuid::new_v4(),
                created_at: Utc::now().naive_utc(),
                batch_id: batch.id,
                path: item.replace(asset_folder, "assets").to_string(),
                snap_shot_type: SnapShotType::New,
                name: item.split('/').last().unwrap().to_string(),
            }),
    );

    snap_shots.extend(
        batch
            .diff_image
            .iter()
            .map(|item| item.old.clone())
            .map(|item| SnapShot {
                id: uuid::Uuid::new_v4(),
                created_at: Utc::now().naive_utc(),
                batch_id: batch.id,
                path: item.replace(asset_folder, "assets").to_string(),
                snap_shot_type: SnapShotType::Old,
                name: item.split('/').last().unwrap().to_string(),
            }),
    );

    snap_shots
}

async fn handle_snap_shot_for_url(
    url: &str,
    image_type: SnapShotType,
) -> Result<Vec<Result<RawImage, Error>>, Error> {
    tracing::debug!("Capturing screen shots for url: {}", url);

    let image_params = get_screenshot_params_by_url(url, image_type).await?;

    let results = capture_screenshots::capture_screenshots(&image_params).await?;

    let num_ok_results = results.iter().filter(|r| r.is_ok()).count();

    tracing::debug!(
        "Captured {}/{} for url {}",
        num_ok_results,
        image_params.len(),
        url
    );

    Ok(results)
}

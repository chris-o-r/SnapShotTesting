use std::fs;

use crate::{
    models::{
        raw_image::RawImage,
        snapshot_batch::{DiffImage, SnapShotBatch, SnapShotBatchImage},
    },
    utils::{
        capture_screenshots::{self},
        compare_images::{self},
        env_variables,
        story_book::get_screenshot_params_by_url,
    },
};
use anyhow::Error;
use chrono::Utc;
use uuid::Uuid;

use crate::{
    db::{
        snapshot_batch_store,
        snapshot_store::{self},
    },
    models::{
        snapshot::{SnapShot, SnapShotType},
        snapshot_batch::SnapShotBatchDTO,
    },
};

pub async fn create_snapshots(
    new_url: &str,
    old_url: &str,
    db_pool: &sqlx::Pool<sqlx::Postgres>,
) -> Result<SnapShotBatch, Error> {
    let asset_folder = env_variables::EnvVariables::new().assets_folder;

    let new_url = new_url.to_string();
    let old_url = old_url.to_string();

    let mut transaction: sqlx::Transaction<'_, sqlx::Postgres> = db_pool.begin().await?;

    let batch = snapshot_batch_store::insert_snap_shot_batch(
        &mut transaction,
        &SnapShotBatchDTO {
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

    let mut images_before_cleaned: Vec<RawImage> = vec![];
    let mut images_after_cleaned: Vec<RawImage> = vec![];

    images_1
        .into_iter()
        .zip(images_2.iter())
        .for_each(|(image_1, image_2)| match (image_1, image_2) {
            (Ok(img_1), Ok(img_2)) => {
                images_before_cleaned.push(img_1);
                images_after_cleaned.push(img_2.clone());
            }
            (Err(_), Ok(img_2)) => {
                images_after_cleaned.push(img_2.clone());
            }
            (Ok(img_1), Err(_)) => {
                images_before_cleaned.push(img_1);
            }
            (Err(_), Err(_)) => {
                // Do nothing
            }
        });

    let diff_images: compare_images::CompareImagesReturn =
        compare_images::compare_images(images_before_cleaned.clone(), images_after_cleaned.clone())
            .await?;

    create_folders(format!("{}/{}", asset_folder, random_folder_name).as_str())?;

    let batch = SnapShotBatch {
        id: batch.id,
        name: batch.name,
        created_at: batch.created_at,
        new_story_book_version: batch.new_story_book_version,
        old_story_book_version: batch.old_story_book_version,
        created_image_paths: diff_images
            .created_images_paths
            .into_iter()
            .map(|img| {
                let path = img
                    .clone()
                    .save(format!("{}/created", &random_folder_name).as_str())
                    .unwrap();

                SnapShotBatchImage {
                    name: img.image_name,
                    path: path,
                    width: img.width,
                    height: img.height,
                }
            })
            .collect(),
        deleted_image_paths: diff_images
            .deleted_images_paths
            .into_iter()
            .map(|img| {
                let path = img
                    .clone()
                    .save(format!("{}/deleted", &random_folder_name).as_str())
                    .unwrap();

                SnapShotBatchImage {
                    name: img.image_name,
                    path,
                    width: img.width,
                    height: img.height,
                }
            })
            .collect(),
        diff_image: diff_images
            .diff_images_paths
            .clone()
            .into_iter()
            .filter_map(|(color_image, lcs_image)| {
                let image_name = color_image.image_name.clone();

                let old_image = images_before_cleaned
                    .clone()
                    .into_iter()
                    .find(|item| item.image_name == image_name)
                    .unwrap();

                let new_image = images_after_cleaned
                    .clone()
                    .into_iter()
                    .find(|item| item.image_name == image_name)
                    .unwrap();

                let new_image_path = new_image
                    .clone()
                    .save(format!("{}/new", random_folder_name).as_str())
                    .unwrap();
                let old_image_path = old_image
                    .clone()
                    .save(format!("{}/old", random_folder_name).as_str())
                    .unwrap();

                let color_diff_path = color_image
                    .clone()
                    .save(format!("{}/diff/color", random_folder_name).as_str())
                    .unwrap();

                let lcs_diff_path = lcs_image
                    .clone()
                    .save(format!("{}/diff/lcs", random_folder_name).as_str())
                    .unwrap();

                Some(DiffImage {
                    new: SnapShotBatchImage {
                        name: new_image.image_name,
                        width: new_image.width,
                        height: new_image.height,
                        path: new_image_path,
                    },
                    old: SnapShotBatchImage {
                        name: old_image.image_name,
                        width: old_image.width,
                        height: old_image.height,
                        path: old_image_path,
                    },
                    color_diff: SnapShotBatchImage {
                        name: image_name,
                        width: color_image.width,
                        height: color_image.height,
                        path: color_diff_path,
                    },
                    lcs_diff: SnapShotBatchImage {
                        name: lcs_image.image_name,
                        width: lcs_image.width,
                        height: lcs_image.height,
                        path: lcs_diff_path,
                    },
                })
            })
            .collect(),
    };

    let snap_shot_array = batch
        .clone()
        .into_snapshots()
        .iter_mut()
        .map(|item| {
            item.path = item.path.replace(&asset_folder, "assets");
            item.to_owned()
        })
        .collect::<Vec<SnapShot>>();

    snapshot_store::insert_snapshots(&mut transaction, &snap_shot_array).await?;

    transaction.commit().await?;

    Ok(batch)
}

async fn handle_snap_shot_for_url(
    url: &str,
    image_type: SnapShotType,
) -> Result<Vec<Result<RawImage, Error>>, Error> {
    tracing::debug!("Capturing screen shots for url: {}", url);

    let image_params = get_screenshot_params_by_url(url, &image_type).await?;

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

fn create_folders(folder_name: &str) -> Result<(), anyhow::Error> {
    fs::create_dir_all(folder_name)?;
    fs::create_dir_all(format!("{}/deleted", folder_name))?;
    fs::create_dir_all(format!("{}/created", folder_name))?;
    fs::create_dir_all(format!("{}/diff", folder_name))?;

    Ok(())
}

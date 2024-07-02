use anyhow::{Error, Ok};
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
        snap_shot::{SnapShot, SnapShotType},
        snap_shot_batch::{self, SnapShotBatch, SnapShotBatchDTO},
        snap_shot_response::SnapShotResponse,
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
        SnapShotBatchDTO {
            id: uuid::Uuid::new_v4(),
            created_at: Utc::now().naive_utc(),
            name: format!("{}-{}", new_url, old_url),
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
    let image_params = get_screen_shot_params_by_url(url, param_name).await?;

    capture_screen_shots(image_params, random_folder_name).await
}

pub async fn get_snap_shot_history(
    db_pool: sqlx::Pool<sqlx::Postgres>,
) -> Result<Vec<SnapShotBatch>, Error> {
    let mut result: Vec<SnapShotBatch> = Vec::new();
    let snap_shot_batches = snap_shot_batch_store::get_all_snap_shot_batches(&db_pool).await?;

    for batch in snap_shot_batches {
        let snap_shots = get_all_snap_shots_by_batch_id(&db_pool, &batch.id).await?;

        let snap_shot_batch = create_snap_shot_batch_from_dto(batch, snap_shots);

        result.push(snap_shot_batch);
    }
    Ok(result)
}

pub async fn get_snap_shot_batch_by_id(
    id: Uuid,
    db_pool: sqlx::Pool<sqlx::Postgres>,
) -> Result<Option<SnapShotBatch>, Error> {
    let batch_dto = match snap_shot_batch_store::get_snap_batch_by_id(&db_pool, &id).await? {
        Some(batch) => batch,
        None => return Ok(None),
    };

    let snap_shots = get_all_snap_shots_by_batch_id(&db_pool, &batch_dto.id).await?;

    Ok(Some(create_snap_shot_batch_from_dto(batch_dto, snap_shots)))
}

fn create_snap_shot_batch_from_dto(
    snap_shot_batch_dto: SnapShotBatchDTO,
    snap_shots: Vec<SnapShot>,
) -> SnapShotBatch {
    let mut snap_shot_batch = SnapShotBatch {
        id: snap_shot_batch_dto.id,
        name: snap_shot_batch_dto.name,
        created_at: snap_shot_batch_dto.created_at,
        new_story_book_version: snap_shot_batch_dto.new_story_book_version,
        old_story_book_version: snap_shot_batch_dto.old_story_book_version,
        diff_images_paths: CompareImagesReturn {
            diff_images_paths: Vec::new(),
            created_images_paths: Vec::new(),
            deleted_images_paths: Vec::new(),
        },
        new_images_paths: Vec::new(),
        old_images_paths: Vec::new(),
    };

    for snap_shot in snap_shots {
        match snap_shot.snap_shot_type {
            SnapShotType::New => snap_shot_batch
                .new_images_paths
                .push(snap_shot.path.clone()),
            SnapShotType::Old => snap_shot_batch
                .old_images_paths
                .push(snap_shot.path.clone()),
            SnapShotType::Diff => snap_shot_batch
                .diff_images_paths
                .diff_images_paths
                .push(snap_shot.path.clone()),
            SnapShotType::Create => snap_shot_batch
                .diff_images_paths
                .created_images_paths
                .push(snap_shot.path.clone()),
            SnapShotType::Deleted => snap_shot_batch
                .diff_images_paths
                .deleted_images_paths
                .push(snap_shot.path.clone()),
        }
    }

    snap_shot_batch
}

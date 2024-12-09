use crate::models::snapshot_batch_v2::{DiffImage, SnapShotBatchV2};
use anyhow::Error;
use uuid::Uuid;

use crate::{
    db::{snap_shot_batch_store, snapshot_store::get_all_snapshots_by_batch_id},
    models::{
        snapshot::{SnapShot, SnapShotType},
        snapshot_batch::SnapShotBatchDTO,
    },
};

pub async fn get_snapshot_history(
    db_pool: sqlx::Pool<sqlx::Postgres>,
) -> Result<Vec<SnapShotBatchV2>, Error> {
    let mut result: Vec<SnapShotBatchV2> = Vec::new();
    let snap_shot_batches = snap_shot_batch_store::get_all_snapshot_batches(&db_pool).await?;

    for batch in snap_shot_batches {
        let snap_shots = get_all_snapshots_by_batch_id(&db_pool, &batch.id).await?;

        let snap_shot_batch = create_snapshot_batch_from_dto(batch, snap_shots);

        result.push(snap_shot_batch);
    }
    Ok(result)
}

pub async fn get_snap_shot_batch_by_id(
    id: Uuid,
    db_pool: sqlx::Pool<sqlx::Postgres>,
) -> Result<Option<SnapShotBatchV2>, Error> {
    let batch_dto = match snap_shot_batch_store::get_snap_batch_by_id(&db_pool, &id).await? {
        Some(batch) => batch,
        None => return Ok(None),
    };

    let snap_shots = get_all_snapshots_by_batch_id(&db_pool, &batch_dto.id).await?;

    Ok(Some(create_snapshot_batch_from_dto(batch_dto, snap_shots)))
}

fn create_snapshot_batch_from_dto(
    snap_shot_batch_dto: SnapShotBatchDTO,
    snap_shots: Vec<SnapShot>,
) -> SnapShotBatchV2 {
    let old_images: Vec<SnapShot> = snap_shots
        .clone()
        .into_iter()
        .filter(|item| item.snap_shot_type == SnapShotType::Old)
        .collect();
    
    let new_images: Vec<SnapShot> = snap_shots
        .clone()
        .into_iter()
        .filter(|item| item.snap_shot_type == SnapShotType::New)
        .collect();

    SnapShotBatchV2 {
        id: snap_shot_batch_dto.id,
        name: snap_shot_batch_dto.name,
        created_at: snap_shot_batch_dto.created_at,
        new_story_book_version: snap_shot_batch_dto.new_story_book_version,
        old_story_book_version: snap_shot_batch_dto.old_story_book_version,
        diff_image: snap_shots
            .clone()
            .into_iter()
            .filter_map(|snap| {
                if snap.snap_shot_type != SnapShotType::Diff {
                    return None;
                }

                let old_image = old_images
                    .clone()
                    .into_iter()
                    .find(|item| {
                        return item.name == snap.name;
                    })
                    .unwrap();

                let new_image = new_images
                    .clone()
                    .into_iter()
                    .find(|item| {
                        return item.name == snap.name;
                    })
                    .unwrap();

                Some(DiffImage {
                    new: new_image.path.clone(),
                    old: old_image.path.clone(),
                    diff: snap.path.clone(),
                })
            })
            .collect(),
        deleted_image_paths: snap_shots
            .clone()
            .into_iter()
            .filter_map(|snap| {
                if snap.snap_shot_type == SnapShotType::Deleted {
                    return Some(snap.path);
                }

                return None;
            })
            .collect(),
        created_image_paths: snap_shots
            .clone()
            .into_iter()
            .filter_map(|snap| {
                if snap.snap_shot_type == SnapShotType::Create {
                    return Some(snap.path);
                }

                return None;
            })
            .collect(),
    }
}

pub async fn delete_all_batches(db_pool: sqlx::Pool<sqlx::Postgres>) -> Result<(), anyhow::Error> {
    snap_shot_batch_store::delete_all_snapshot_batches(&db_pool).await
}

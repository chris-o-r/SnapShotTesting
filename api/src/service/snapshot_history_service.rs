use crate::utils::compare_images::CompareImagesReturn;
use anyhow::Error;
use uuid::Uuid;

use crate::{
    db::{snap_shot_batch_store, snapshot_store::get_all_snapshots_by_batch_id},
    models::{
        snapshot::{SnapShot, SnapShotType},
        snapshot_batch::{SnapShotBatch, SnapShotBatchDTO},
    },
};

pub async fn get_snapshot_history(
    db_pool: sqlx::Pool<sqlx::Postgres>,
) -> Result<Vec<SnapShotBatch>, Error> {
    let mut result: Vec<SnapShotBatch> = Vec::new();
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
) -> Result<Option<SnapShotBatch>, Error> {
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

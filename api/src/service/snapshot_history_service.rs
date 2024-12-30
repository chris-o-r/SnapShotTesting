use crate::{
    db::snapshot_store,
    models::snapshot_batch::{DiffImage, SnapShotBatch},
};
use anyhow::Error;
use uuid::Uuid;

use crate::{
    db::{snapshot_batch_store, snapshot_store::get_all_snapshots_by_batch_id},
    models::{
        snapshot::{SnapShot, SnapShotType},
        snapshot_batch::SnapShotBatchDTO,
    },
};

pub async fn get_snapshot_history(
    db_pool: &sqlx::Pool<sqlx::Postgres>,
) -> Result<Vec<SnapShotBatch>, Error> {
    let mut result: Vec<SnapShotBatch> = Vec::new();
    let snap_shot_batches = snapshot_batch_store::get_all_snapshot_batches(&db_pool).await?;

    for batch in snap_shot_batches {
        let snapshots = get_all_snapshots_by_batch_id(&db_pool, &batch.id).await?;

        let snapshot_batch = create_snapshot_batch_from_dto(batch, snapshots);

        result.push(snapshot_batch);
    }
    Ok(result)
}

pub async fn get_snap_shot_batch_by_id(
    id: Uuid,
    db_pool: &sqlx::Pool<sqlx::Postgres>,
) -> Result<Option<SnapShotBatch>, Error> {
    let batch_dto = match snapshot_batch_store::get_snap_batch_by_id(&db_pool, &id).await? {
        Some(batch) => batch,
        None => return Ok(None),
    };

    let snapshots = get_all_snapshots_by_batch_id(&db_pool, &batch_dto.id).await?;

    Ok(Some(create_snapshot_batch_from_dto(batch_dto, snapshots)))
}

pub async fn delete_snapshot_batch_by_id(
    id: Uuid,
    db_pool: &sqlx::Pool<sqlx::Postgres>,
) -> Result<Option<SnapShotBatch>, Error> {
    let mut transaction: sqlx::Transaction<'_, sqlx::Postgres> = db_pool.begin().await?;

    let batch_deletion =
        snapshot_batch_store::delete_snapshot_batches_by_id(&mut transaction, &id).await?;

    let snapshots_deletion =
        snapshot_store::delete_all_snapshots_by_batch_id(&mut transaction, &id).await?;

    if snapshots_deletion.is_none() || batch_deletion.is_none() {
        transaction.rollback().await?;
        tracing::error!("Cannot delete snap shot batch by id: {}. Wrong ID", id);
        return Ok(None);
    }
    transaction.commit().await?;

    Ok(Some(create_snapshot_batch_from_dto(
        batch_deletion.unwrap(),
        snapshots_deletion.unwrap(),
    )))
}

fn create_snapshot_batch_from_dto(
    snap_shot_batch_dto: SnapShotBatchDTO,
    snapshots: Vec<SnapShot>,
) -> SnapShotBatch {
    let old_images: Vec<SnapShot> = snapshots
        .clone()
        .into_iter()
        .filter(|item| item.snap_shot_type == SnapShotType::Old)
        .collect();

    let new_images: Vec<SnapShot> = snapshots
        .clone()
        .into_iter()
        .filter(|item| item.snap_shot_type == SnapShotType::New)
        .collect();

    let lcs_diffs: Vec<SnapShot> = snapshots
        .clone()
        .into_iter()
        .filter(|item| item.snap_shot_type == SnapShotType::LcsDiff)
        .collect();

    SnapShotBatch {
        id: snap_shot_batch_dto.id,
        name: snap_shot_batch_dto.name,
        created_at: snap_shot_batch_dto.created_at,
        new_story_book_version: snap_shot_batch_dto.new_story_book_version,
        old_story_book_version: snap_shot_batch_dto.old_story_book_version,
        diff_image: snapshots
            .clone()
            .into_iter()
            .filter_map(|color_diff| {
                if color_diff.snap_shot_type != SnapShotType::ColorDiff {
                    return None;
                }

                let old_image = old_images
                    .clone()
                    .into_iter()
                    .find(|item| {
                        return item.name == color_diff.name;
                    })
                    .unwrap();

                let new_image = new_images
                    .clone()
                    .into_iter()
                    .find(|item| {
                        return item.name == color_diff.name;
                    })
                    .unwrap();

                let lcs_image = lcs_diffs
                    .clone()
                    .into_iter()
                    .find(|item| item.name == color_diff.name)
                    .unwrap();

                Some(DiffImage {
                    new: new_image.into_snapshot_batch_image(),
                    old: old_image.into_snapshot_batch_image(),
                    color_diff: color_diff.into_snapshot_batch_image(),
                    lcs_diff: lcs_image.into_snapshot_batch_image(),
                })
            })
            .collect(),
        deleted_image_paths: snapshots
            .clone()
            .into_iter()
            .filter_map(|snap| {
                if snap.snap_shot_type == SnapShotType::Deleted {
                    return Some(snap.into_snapshot_batch_image());
                }

                return None;
            })
            .collect(),
        created_image_paths: snapshots
            .clone()
            .into_iter()
            .filter_map(|snap| {
                if snap.snap_shot_type == SnapShotType::Create {
                    return Some(snap.into_snapshot_batch_image());
                }

                return None;
            })
            .collect(),
    }
}

pub async fn delete_all_batches(db_pool: sqlx::Pool<sqlx::Postgres>) -> Result<(), anyhow::Error> {
    snapshot_batch_store::delete_all_snapshot_batches(&db_pool).await
}

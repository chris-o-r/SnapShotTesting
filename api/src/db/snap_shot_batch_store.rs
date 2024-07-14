use sqlx::{Pool, Postgres};

use crate::models::snapshot_batch::SnapShotBatchDTO;

pub async fn insert_snap_shot_batch(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    snap_shot_batch: SnapShotBatchDTO,
) -> Result<SnapShotBatchDTO, anyhow::Error> {
    let sql = r"
    INSERT INTO snap_shots_batches (
            name,
            created_at,
            new_story_book_version,
            old_story_book_version
        )
    VALUES ($1, $2, $3, $4)
    RETURNING *;
    ";

    let res = sqlx::query_as::<_, SnapShotBatchDTO>(sql)
        .bind(snap_shot_batch.name)
        .bind(snap_shot_batch.created_at)
        .bind(snap_shot_batch.new_story_book_version)
        .bind(snap_shot_batch.old_story_book_version)
        .fetch_all(&mut **transaction)
        .await
        .map_err(|err| {
            tracing::error!("Cannot insert snap shot batch [{}]", err.to_string());
            anyhow::Error::from(err)
        })?;

    match res.first() {
        Some(snap_shot_batch) => Ok(snap_shot_batch.clone()),
        None => {
            return Err(anyhow::Error::msg("Cannot get snap shot batch"));
        }
    }
}

pub async fn get_all_snapshot_batches(
    pool: &Pool<Postgres>,
) -> Result<Vec<SnapShotBatchDTO>, anyhow::Error> {
    let sql = r"
    SELECT * FROM snap_shots_batches
    ";

    let snap_shot_batches = sqlx::query_as::<_, SnapShotBatchDTO>(sql)
        .fetch_all(pool)
        .await
        .map_err(|err| {
            tracing::error!("Cannot get snap shot batches [{}]", err.to_string());
            anyhow::Error::from(err)
        })?;

    Ok(snap_shot_batches)
}

pub async fn get_snap_batch_by_id(
    pool: &Pool<Postgres>,
    id: &uuid::Uuid,
) -> Result<Option<SnapShotBatchDTO>, anyhow::Error> {
    let sql = r"
    SELECT * FROM snap_shots_batches
    WHERE id = $1
    ";

    let snap_shot_batch = sqlx::query_as::<_, SnapShotBatchDTO>(sql)
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|err| {
            tracing::error!("Cannot get snap shot batch [{}]", err.to_string());
            anyhow::Error::from(err)
        })?;

    Ok(snap_shot_batch)
}

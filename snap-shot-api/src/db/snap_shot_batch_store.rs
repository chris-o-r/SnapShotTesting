use sqlx::{Pool, Postgres};

use crate::models::snap_shot_batch::SnapShotBatch;

pub async fn save_snap_shot_batch(
    pool: &Pool<Postgres>,
    snap_shot_batch: SnapShotBatch,
) -> Result<(), anyhow::Error> {
    let sql = r"
    INSERT INTO snap_shots_batches (
            id,
            name,
            created_at,
            new_story_book_version,
            old_story_book_version
        )
    VALUES ($1, $2, $3, $4, $5)
    ";

    sqlx::query(sql)
        .bind(snap_shot_batch.id)
        .bind(snap_shot_batch.name)
        .bind(snap_shot_batch.created_at)
        .bind(snap_shot_batch.new_story_book_version)
        .bind(snap_shot_batch.old_story_book_version)
        .execute(pool)
        .await
        .map_err(|err| {
            tracing::error!("Cannot insert snap shot batch [{}]", err.to_string());
            anyhow::Error::from(err)
        })?;

    Ok(())
}

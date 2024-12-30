use sqlx::{Pool, Postgres};

use crate::models::snapshot_batch::SnapShotBatchDTO;

pub async fn insert_snap_shot_batch(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    snap_shot_batch: SnapShotBatchDTO,
) -> Result<SnapShotBatchDTO, anyhow::Error> {
    let sql = r"
    INSERT INTO snapshots_batches (
            name,
            created_at,
            new_story_book_version,
            old_story_book_version
        )
    VALUES ($1, $2, $3, $4)
    RETURNING *;
    ";

    let res: Vec<SnapShotBatchDTO> = sqlx::query_as::<_, SnapShotBatchDTO>(sql)
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
    SELECT * FROM snapshots_batches
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
    SELECT * FROM snapshots_batches
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

pub async fn delete_all_snapshot_batches(pool: &Pool<Postgres>) -> Result<(), anyhow::Error> {
    let sql = r"
    DELETE FROM snapshots_batches
    ";

    sqlx::query(sql).execute(pool).await.map_err(|err| {
        tracing::error!("Cannot delete all snap shot batches [{}]", err.to_string());
        anyhow::Error::from(err)
    })?;

    Ok(())
}

pub async fn delete_snapshot_batches_by_id(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    id: &uuid::Uuid,
) -> Result<Option<SnapShotBatchDTO>, anyhow::Error> {
    let sql = r"
    DELETE FROM snapshots_batches 
    WHERE id = $1
    RETURNING *;";

    let batches = sqlx::query_as::<_, SnapShotBatchDTO>(sql)
        .bind(id)
        .fetch_all(&mut **transaction)
        .await
        .map_err(|err| {
            tracing::error!("Cannot delete all snap shot batches [{}]", err.to_string());
            anyhow::Error::from(err)
        })?;

    if batches.is_empty() {
        return Ok(None);
    }

    Ok(Some(batches[0].clone()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use sqlx::PgPool;
    use uuid::Uuid;

    #[sqlx::test]
    async fn test_snapshot_batches(pool: PgPool) {
        let mut transaction: sqlx::Transaction<'_, sqlx::Postgres> = pool.begin().await.unwrap();
        let batch = insert_snap_shot_batch(
            &mut transaction,
            SnapShotBatchDTO {
                id: Uuid::new_v4(),
                created_at: Utc::now().naive_utc(),
                name: format!("{}-{}", "", ""),
                new_story_book_version: String::from(""),
                old_story_book_version: String::from(""),
            },
        )
        .await;
        assert!(batch.is_ok());

        let _ = transaction.commit().await;

        let all_batches = get_all_snapshot_batches(&pool).await.unwrap();

        assert_eq!(all_batches.len(), 1);

        let batch_by_id = get_snap_batch_by_id(&pool, &all_batches[0].id)
            .await
            .unwrap();

        assert!(batch_by_id.is_some());

        let all_deleted = delete_all_snapshot_batches(&pool).await;

        assert!(all_deleted.is_ok());

        let all_batches = get_all_snapshot_batches(&pool).await.unwrap();

        assert_eq!(all_batches.len(), 0);
    }
}

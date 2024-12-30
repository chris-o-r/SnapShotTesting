use anyhow::Result;
use chrono::NaiveDateTime;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::models::snapshot::SnapShot;

pub async fn insert_snapshots(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    snapshots: &Vec<SnapShot>,
) -> Result<Vec<SnapShot>, sqlx::Error> {
    let sql = r"
    INSERT INTO snapshots (
            batch_id,
            name,
            path,
            width,
            height,
            snap_shot_type,
            created_at
        )
    SELECT * FROM UNNEST(
        $1::UUID[],
        $2::VARCHAR(100)[],
        $3::VARCHAR(200)[],
        $4::DOUBLE PRECISION[],
        $5::DOUBLE PRECISION[],
        $6::VARCHAR(100)[],
        $7::TIMESTAMP[]
    )
    RETURNING *;";

    let res: std::result::Result<Vec<SnapShot>, sqlx::Error> = sqlx::query_as::<_, SnapShot>(sql)
        .bind(snapshots.iter().map(|s| s.batch_id).collect::<Vec<Uuid>>())
        .bind(
            snapshots
                .iter()
                .map(|s| s.name.clone())
                .collect::<Vec<String>>(),
        )
        .bind(
            snapshots
                .iter()
                .map(|s| s.path.clone())
                .collect::<Vec<String>>(),
        )
        .bind(
            snapshots
                .iter()
                .map(|item| item.width)
                .collect::<Vec<f64>>(),
        )
        .bind(
            snapshots
                .iter()
                .map(|item| item.height)
                .collect::<Vec<f64>>(),
        )
        .bind(
            snapshots
                .iter()
                .map(|s| s.snap_shot_type.to_string())
                .collect::<Vec<String>>(),
        )
        .bind(
            snapshots
                .iter()
                .map(|s| s.created_at)
                .collect::<Vec<NaiveDateTime>>(),
        )
        .fetch_all(&mut **transaction)
        .await
        .map_err(|err| {
            tracing::error!("Cannot insert snap shots [{}]", err.to_string());
            err
        });

    res
}

pub async fn get_all_snapshots_by_batch_id(
    pool: &Pool<Postgres>,
    batch_id: &Uuid,
) -> Result<Vec<SnapShot>, anyhow::Error> {
    let sql = r"
    SELECT * FROM snapshots where batch_id = $1";

    let snapshots = sqlx::query_as::<_, SnapShot>(sql)
        .bind(batch_id)
        .fetch_all(pool)
        .await
        .map_err(|err| {
            tracing::error!("Cannot get snap shots [{}]", err.to_string());
            anyhow::Error::from(err)
        })?;

    Ok(snapshots)
}

pub async fn delete_all_snapshots(pool: &Pool<Postgres>) -> Result<(), anyhow::Error> {
    let sql = r"
    DELETE FROM snapshots
    ";

    sqlx::query(sql).execute(pool).await.map_err(|err| {
        tracing::error!("Cannot delete all snapshots [{}]", err.to_string());
        anyhow::Error::from(err)
    })?;

    Ok(())
}

pub async fn delete_all_snapshots_by_batch_id(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    id: &uuid::Uuid,
) -> Result<Option<Vec<SnapShot>>, anyhow::Error> {
    let sql = r"
    DELETE FROM snapshots  
    WHERE batch_id = $1
    RETURNING *;";

    let val = sqlx::query_as::<_, SnapShot>(sql)
        .bind(id)
        .fetch_all(&mut **transaction)
        .await
        .map_err(|err| {
            tracing::error!("Cannot delete all snap shots by id: {}", err.to_string());
            anyhow::Error::from(err)
        })?;

    if val.is_empty() {
        return Ok(None);
    }

    Ok(Some(val))
}

#[cfg(test)]
mod tests {
    use crate::models::snapshot::SnapShotType;

    use super::*;
    use chrono::Utc;
    use sqlx::PgPool;
    use uuid::Uuid;

    #[sqlx::test]
    async fn test_snapshot_insert_snapshot(pool: PgPool) {
        let mut transaction: sqlx::Transaction<'_, sqlx::Postgres> = pool.begin().await.unwrap();

        let batch_id = Uuid::new_v4();
        let batch = insert_snapshots(
            &mut transaction,
            &vec![SnapShot {
                id: Uuid::new_v4(),
                batch_id,
                name: "name".to_string(),
                path: "path".to_string(),
                created_at: Utc::now().naive_utc(),
                height: 100.0,
                width: 100.0,
                snap_shot_type: SnapShotType::New,
            }],
        )
        .await;
        let _ = transaction.commit().await;

        assert!(batch.is_ok());

        let snapshots_by_batch = get_all_snapshots_by_batch_id(&pool, &batch_id)
            .await
            .unwrap();

        assert_eq!(batch.unwrap()[0].id, snapshots_by_batch[0].id);

        assert_eq!(snapshots_by_batch.len(), 1);

        let all_deleted = delete_all_snapshots(&pool).await;

        assert!(all_deleted.is_ok());

        let snapshots_by_batch = get_all_snapshots_by_batch_id(&pool, &batch_id)
            .await
            .unwrap();

        assert_eq!(snapshots_by_batch.len(), 0);
    }

    #[sqlx::test]
    async fn test_snapshot_delete_all_snapshots_by_batch_id(pool: PgPool) {
        let mut transaction: sqlx::Transaction<'_, sqlx::Postgres> = pool.begin().await.unwrap();

        let batch_id = Uuid::new_v4();
        let _batch = insert_snapshots(
            &mut transaction,
            &vec![SnapShot {
                id: Uuid::new_v4(),
                batch_id,
                name: "name".to_string(),
                path: "path".to_string(),
                created_at: Utc::now().naive_utc(),
                height: 100.0,
                width: 100.0,
                snap_shot_type: SnapShotType::New,
            }],
        )
        .await;

        let deleted = delete_all_snapshots_by_batch_id(&mut transaction, &batch_id)
            .await
            .unwrap();

        
        assert!(deleted.is_some());

        

    }
}

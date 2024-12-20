use anyhow::Result;
use chrono::NaiveDateTime;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::models::snapshot::SnapShot;

pub async fn insert_snapshots(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    snap_shots: Vec<SnapShot>,
) -> Result<(), anyhow::Error> {
    let sql = r"
    INSERT INTO snap_shots (
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
    )";

    sqlx::query_as::<_, SnapShot>(sql)
        .bind(
            snap_shots
                .iter()
                .map(|s| s.batch_id.clone())
                .collect::<Vec<Uuid>>(),
        )
        .bind(
            snap_shots
                .iter()
                .map(|s| s.name.clone())
                .collect::<Vec<String>>(),
        )
        .bind(
            snap_shots
                .iter()
                .map(|s| s.path.clone())
                .collect::<Vec<String>>(),
        )
        .bind(
            snap_shots
                .iter()
                .map(|item| item.width)
                .collect::<Vec<f64>>(),
        )
        .bind(
            snap_shots
                .iter()
                .map(|item| item.height)
                .collect::<Vec<f64>>(),
        )
        .bind(
            snap_shots
                .iter()
                .map(|s| s.snap_shot_type.to_string())
                .collect::<Vec<String>>(),
        )
        .bind(
            snap_shots
                .iter()
                .map(|s| s.created_at)
                .collect::<Vec<NaiveDateTime>>(),
        )
        .fetch_all(&mut **transaction)
        .await
        .map_err(|err| {
            tracing::error!("Cannot insert snap shots [{}]", err.to_string());
            err
        })?;

    Ok(())
}

pub async fn get_all_snapshots_by_batch_id(
    pool: &Pool<Postgres>,
    batch_id: &Uuid,
) -> Result<Vec<SnapShot>, anyhow::Error> {
    let sql = r"
    SELECT * FROM snap_shots where batch_id = $1";

    let snap_shots = sqlx::query_as::<_, SnapShot>(sql)
        .bind(batch_id)
        .fetch_all(pool)
        .await
        .map_err(|err| {
            tracing::error!("Cannot get snap shots [{}]", err.to_string());
            anyhow::Error::from(err)
        })?;

    Ok(snap_shots)
}

pub async fn delete_all_snapshots(pool: &Pool<Postgres>) -> Result<(), anyhow::Error> {
    let sql = r"
    DELETE FROM snap_shots
    ";

    sqlx::query(sql).execute(pool).await.map_err(|err| {
        tracing::error!("Cannot delete all snapshots [{}]", err.to_string());
        anyhow::Error::from(err)
    })?;

    Ok(())
}

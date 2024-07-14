use anyhow::Result;
use chrono::NaiveDateTime;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::models::snap_shot::SnapShot;

pub async fn insert_snap_shots(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    snap_shots: Vec<SnapShot>,
) -> Result<(), anyhow::Error> {
    let sql = r"
    INSERT INTO snap_shots (
            batch_id,
            name,
            path,
            snap_shot_type,
            created_at
        )
    SELECT * FROM UNNEST(
        $1::UUID[],
        $2::VARCHAR(100)[],
        $3::VARCHAR(200)[],
        $4::VARCHAR(100)[],
        $5::TIMESTAMP[]
    )";

    let batch_ids: Vec<Uuid> = snap_shots.iter().map(|s| s.batch_id.clone()).collect();

    let names = snap_shots
        .iter()
        .map(|s| s.name.clone())
        .collect::<Vec<String>>();

    let paths = snap_shots
        .iter()
        .map(|s| s.path.clone())
        .collect::<Vec<String>>();

    let snap_shot_types = snap_shots
        .iter()
        .map(|s| s.snap_shot_type.to_string())
        .collect::<Vec<String>>();

    let created_ats = snap_shots
        .iter()
        .map(|s| s.created_at)
        .collect::<Vec<NaiveDateTime>>();

    sqlx::query_as::<_, SnapShot>(sql)
        .bind(batch_ids)
        .bind(names)
        .bind(paths)
        .bind(snap_shot_types)
        .bind(created_ats)
        .fetch_all(&mut **transaction)
        .await
        .map_err(|err| {
            tracing::error!("Cannot insert snap shots [{}]", err.to_string());
            err
        })?;

    Ok(())
}

pub async fn get_all_snap_shots_by_batch_id(
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

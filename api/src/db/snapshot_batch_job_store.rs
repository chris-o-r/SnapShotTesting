use crate::models::snapshot_batch_job::SnapShotBatchJob;

use bb8_redis::{bb8::Pool, redis::cmd, RedisConnectionManager};
const REDIS_KEY: &str = "snap_shot_batch_job";

pub async fn insert_snapshot_batch_job(
    pool: &Pool<RedisConnectionManager>,
    snap_shot_batch_job: SnapShotBatchJob,
) -> Result<SnapShotBatchJob, anyhow::Error> {
    let mut conn: bb8_redis::bb8::PooledConnection<RedisConnectionManager> =
        pool.get().await.unwrap();

    let key = format!("{}:{}", REDIS_KEY, snap_shot_batch_job.id);
    let value = serde_json::to_string(&snap_shot_batch_job)?;

    let _: () = cmd("SET")
        .arg(key)
        .arg(value)
        .query_async(&mut *conn)
        .await
        .map_err(|e| anyhow::Error::msg(e.to_string()))?;

    Ok(snap_shot_batch_job)
}

pub async fn remove_snapshot_batch_job(
    pool: &Pool<RedisConnectionManager>,
    id: &uuid::Uuid,
) -> Result<(), anyhow::Error> {
    let mut conn = pool.get().await?;
    let _: () = cmd("DEL")
        .arg(format!("{}:{}", REDIS_KEY, id))
        .query_async(&mut *conn)
        .await?;

    Ok(())
}

pub async fn get_job_by_id(
    pool: &Pool<RedisConnectionManager>,
    id: &uuid::Uuid,
) -> Result<Option<SnapShotBatchJob>, anyhow::Error> {
    let mut conn = pool.get().await?;

    let key = format!("{}:{}", REDIS_KEY, id);
    let res: String = redis::cmd("GET").arg(key).query_async(&mut *conn).await?;

    let snap_shot_batch_job: SnapShotBatchJob = serde_json::from_str(&res)?;

    Ok(Some(snap_shot_batch_job))
}

pub async fn update_batch_job(
    pool: &Pool<RedisConnectionManager>,
    snapshot_batch_job: SnapShotBatchJob,
) -> Result<SnapShotBatchJob, anyhow::Error> {
    insert_snapshot_batch_job(pool, snapshot_batch_job).await
}

pub async fn get_all_jobs(
    pool: &Pool<RedisConnectionManager>,
) -> Result<Vec<SnapShotBatchJob>, anyhow::Error> {
    let mut jobs = Vec::new();
    let mut conn = pool.get().await?;

    let keys: Vec<String> = redis::cmd("KEYS")
        .arg(format!("{}:*", REDIS_KEY))
        .query_async(&mut *conn)
        .await?;

    for key in keys {
        let res: String = redis::cmd("GET").arg(key).query_async(&mut *conn).await?;
        let snap_shot_batch_job: SnapShotBatchJob = serde_json::from_str(&res)?;
        jobs.push(snap_shot_batch_job);
    }

    Ok(jobs)
}

pub async fn remove_all_jobs(pool: &Pool<RedisConnectionManager>) -> Result<(), anyhow::Error> {
    let mut conn = pool.get().await?;
    let keys: Vec<String> = redis::cmd("KEYS")
        .arg(format!("{}:*", REDIS_KEY))
        .query_async(&mut *conn)
        .await?;

    println!("Keys: {:?}", keys);
    for key in keys {
        let _: () = cmd("DEL").arg(key).query_async(&mut *conn).await?;
    }

    Ok(())
}

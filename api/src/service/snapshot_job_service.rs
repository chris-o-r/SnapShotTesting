use anyhow::Error;

use crate::{
    db::snapshot_batch_job_store,
    models::snapshot_batch_job::{SnapShotBatchJob, SnapShotBatchJobStatus},
};

pub async fn get_all_running_jobs(
    redis_pool: bb8_redis::bb8::Pool<bb8_redis::RedisConnectionManager>,
) -> Result<Vec<SnapShotBatchJob>, Error> {
    let jobs = snapshot_batch_job_store::get_all_jobs(&redis_pool)
        .await?
        .into_iter()
        .filter(|job| job.status != SnapShotBatchJobStatus::Completed)
        .collect();

    Ok(jobs)
}

pub async fn update_job_status(
    redis_pool: &bb8_redis::bb8::Pool<bb8_redis::RedisConnectionManager>,
    job_id: uuid::Uuid,
    status: SnapShotBatchJobStatus,
) -> Result<(), Error> {
    let mut job = snapshot_batch_job_store::get_job_by_id(&redis_pool, &job_id)
        .await?
        .ok_or_else(|| Error::msg("Job not found"))?;

    job.status = status;

    snapshot_batch_job_store::update_batch_job(&redis_pool, job).await?;

    Ok(())
}

pub async fn clear_all_runnning_jobs(
    redis_pool: bb8_redis::bb8::Pool<bb8_redis::RedisConnectionManager>,
) -> Result<usize, Error> {
    snapshot_batch_job_store::remove_all_jobs(&redis_pool).await
}

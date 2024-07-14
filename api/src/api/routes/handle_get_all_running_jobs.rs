use std::sync::Arc;

use axum::{extract::State, Json};

use crate::{
    api::errors::AppError,
    models::{app_state::AppState, snap_shot_batch_job::SnapShotBatchJob},
    service::snapshot_job_service,
};

pub async fn handle_get_all_running_jobs(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<SnapShotBatchJob>>, AppError> {
    let running_jobs =
        snapshot_job_service::get_all_running_jobs(state.redis_pool.clone()).await?;

    return Ok(Json(running_jobs));
}

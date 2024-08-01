use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

use crate::{
    api::errors::AppError,
    db::snapshot_batch_job_store,
    models::{app_state::AppState, snapshot_batch_job::SnapShotBatchJob},
};

pub async fn handle_get_job_by_id(
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<SnapShotBatchJob>, AppError> {
    let job = snapshot_batch_job_store::get_job_by_id(&state.redis_pool.clone(), &id).await?;

    if job.is_some() {
        return Ok(Json(job.unwrap()));
    } else {
        return Err(AppError(
            anyhow::Error::msg(format!("Job with id {} not found", id)),
            axum::http::StatusCode::NOT_FOUND,
        ));
    }
}

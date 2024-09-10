use std::sync::Arc;

use axum::{
    extract::{Path, State}, routing, Json, Router
};
use utoipa::OpenApi;
use uuid::Uuid;

use crate::{
    api::errors::AppError,
    db::snapshot_batch_job_store,
    models::{app_state::AppState, snapshot_batch_job::{SnapShotBatchJob, SnapShotBatchJobStatus}},
    service::snapshot_job_service,
};


#[derive(OpenApi)]
#[openapi(
    paths(handle_get_job_by_id, handle_get_all_running_jobs),
    components(
        schemas(SnapShotBatchJob, SnapShotBatchJobStatus),
    ),
    tags((name = "Batch Job", description = "All about jobs"))
)]
pub struct JobApiDoc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", routing::get(handle_get_all_running_jobs))
        .route("/:id", routing::get(handle_get_job_by_id))
}

#[utoipa::path(
    get,
    path = "/jobs",
    responses(
        (status = 200, description = "Jobs found successfully", body = Vec<SnapShotBatchJob>),
        (status = NOT_FOUND, description = "Jobs not found")
    ),
    tag = "Batch Job"
)]
async fn handle_get_all_running_jobs(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<SnapShotBatchJob>>, AppError> {
    let running_jobs = snapshot_job_service::get_all_running_jobs(state.redis_pool.clone()).await?;

    return Ok(Json(running_jobs));
}

#[utoipa::path(
    get,
    path = "/jobs/{id}",
    responses(
        (status = 200, description = "Jobs found successfully", body = SnapShotBatchJob),
        (status = NOT_FOUND, description = "Jobs not found")
    ),
    params(("id", description = "Job ID")),
    tag = "Batch Job"
    
)]
async fn handle_get_job_by_id(
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

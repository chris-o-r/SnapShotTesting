use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{routing, Json, Router};

use serde::Deserialize;
use std::sync::Arc;
use utoipa::{OpenApi, ToSchema};
use uuid::Uuid;
use validator::Validate;

use crate::api::errors::AppError;
use crate::api::extractors::ValidateJson;
use crate::models::app_state::AppState;
use crate::models::snapshot_batch::{DiffImage, SnapShotBatch, SnapShotBatchImage};
use crate::service::{snapshot_history_service, snapshot_service};

#[derive(OpenApi)]
#[openapi(
    paths(handle_snapshot, handle_get_snapshot_history, handle_get_snapshot_by_id),
    components(
        schemas(SnapShotParams, SnapShotBatch, DiffImage, SnapShotBatchImage),
    ),
    tags((name = "Snapshot", description = "All about jobs"))
)]
pub struct SnapshotDoc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", routing::post(handle_snapshot))
        .route("/", routing::get(handle_get_snapshot_history))
        .route("/:id", routing::get(handle_get_snapshot_by_id))
        .route("/:id", routing::delete(handle_delete_snapshot_by_id))
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct SnapShotParams {
    #[validate(url)]
    new: String,
    #[validate(url)]
    old: String,
}

#[utoipa::path(
    post,
    path = "/api/snap-shots",
    request_body = SnapShotParams,
    responses(
        (status = 200, description = "Creates snap shots", body = SnapShotBatch),
    ),
    tag="Snapshot"

)]
pub async fn handle_snapshot(
    State(state): State<Arc<AppState>>,
    ValidateJson(payload): ValidateJson<SnapShotParams>,
) -> Result<SnapShotBatch, AppError> {
    snapshot_service::create_snap_shots(payload.new.as_str(), payload.old.as_str(), &state.db_pool)
        .await
        .map_err(|e| AppError(e, StatusCode::INTERNAL_SERVER_ERROR))
}

#[utoipa::path(
    get,
    path = "/api/snap-shots",
    responses(
        (status = 200, description = "Partner account was created", body = Vec<SnapShotBatch>),
    ),
    tag="Snapshot"

)]
async fn handle_get_snapshot_history(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<SnapShotBatch>>, AppError> {
    let res = snapshot_history_service::get_snapshot_history(&state.db_pool)
        .await
        .map_err(|e| AppError(e, axum::http::StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(Json(res))
}

#[utoipa::path(
    get,
    path = "/api/snap-shots/{id}",
    params(("id", description = "Historical Item Id")),
    responses(
        (status = 200, description = "Get snapshot batch by id", body = Vec<SnapShotBatch>),
    ),
    tag="Snapshot"

)]
async fn handle_get_snapshot_by_id(
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<SnapShotBatch>, AppError> {
    let result: Option<SnapShotBatch> =
        snapshot_history_service::get_snap_shot_batch_by_id(id, &state.db_pool).await?;

    if result.is_some() {
        return Ok(Json(result.unwrap()));
    } else {
        return Err(AppError(
            anyhow::Error::msg(format!("Snap shot batch with id {} not found", id)),
            axum::http::StatusCode::NOT_FOUND,
        ));
    }
}

#[utoipa::path(
    delete,
    path = "/api/snap-shots/{id}",
    params(("id", description = "Historical Item Id")),
    responses(
        (status = 200, description = "Delete snapshot batch by id", body = Vec<SnapShotBatch>),
    ),
    tag="Snapshot"

)]
async fn handle_delete_snapshot_by_id(
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<StatusCode, AppError> {
    let result = snapshot_history_service::delete_snapshot_batch_by_id(id, &state.db_pool).await?;

    match result {
        Some(_) => Ok(StatusCode::NO_CONTENT),
        None => Err(AppError(
            anyhow::Error::msg(format!("Snap shot batch with id {} not found", id)),
            axum::http::StatusCode::NOT_FOUND,
        )),
    }
}

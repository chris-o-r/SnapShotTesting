use anyhow::Error;
use axum::http::StatusCode;
use axum::{routing, Json, Router};
use regex::Regex;
use serde::Deserialize;
use std::sync::Arc;
use utoipa::{OpenApi, ToSchema};
use uuid::Uuid;

use crate::api::errors::AppError;
use crate::models::app_state::AppState;
use crate::models::snapshot_batch::SnapShotBatch;
use crate::service::{snapshot_history_service, snapshot_service};
use crate::utils::compare_images::CompareImagesReturn;
use axum::extract::{Path, State};

#[derive(OpenApi)]
#[openapi(
    paths(handle_snapshot, handle_get_snapshot_history, handle_get_snapshot_by_id),
    components(
        schemas(SnapShotParams, SnapShotBatch, SnapShotBatch, CompareImagesReturn),
    ),
    tags((name = "Snapshot", description = "All about jobs"))
)]
pub struct SnapshotDoc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", routing::post(handle_snapshot))
        .route("/", routing::get(handle_get_snapshot_history))
        .route("/:id", routing::get(handle_get_snapshot_by_id))
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct SnapShotParams {
    new: Option<String>,
    old: Option<String>,
}

#[utoipa::path(
    post,
    path = "/api/snap-shots",
    request_body = SnapShotParams,
    responses(
        (status = 200, description = "Partner account was created", body = SnapShotBatch),
    ),
    tag="Snapshot"

)]
pub async fn handle_snapshot(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SnapShotParams>,
) -> Result<SnapShotBatch, AppError> {

    let (new, old) = validate_payload(payload)?;

    snapshot_service::create_snap_shots(
        new.as_str(),
        old.as_str(),
        state.db_pool.clone(),
        state.redis_pool.clone(),
    )
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
    let res = snapshot_history_service::get_snapshot_history(state.db_pool.clone())
        .await
        .map_err(|e| AppError(e, axum::http::StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(Json(res))
}

#[utoipa::path(
    get,
    path = "/api/snap-shots/{id}",
    params(("id", description = "Historical Item Id")),
    responses(
        (status = 200, description = "Partner account was created", body = Vec<SnapShotBatch>),
    ),
    tag="Snapshot"

)]
async fn handle_get_snapshot_by_id(
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<SnapShotBatch>, AppError> {
    let result =
        snapshot_history_service::get_snap_shot_batch_by_id(id, state.db_pool.clone()).await?;

    if result.is_some() {
        return Ok(Json(result.unwrap()));
    } else {
        return Err(AppError(
            anyhow::Error::msg(format!("Snap shot batch with id {} not found", id)),
            axum::http::StatusCode::NOT_FOUND,
        ));
    }
}


fn validate_payload(snap_shot_param: SnapShotParams) -> Result<(String, String), AppError> {
    
    let domain_regex = Regex::new(r"^(?:[a-zA-Z0-9-]+\.)+[a-zA-Z]{2,}(?:/.*)?$").unwrap();


    let new = match snap_shot_param.new {
        Some(new) => new,
        None => {
            return Err(AppError(
                Error::msg("New URL is required"),
                StatusCode::BAD_REQUEST,
            ))
        }
    };

    let old = match snap_shot_param.old {
        Some(old) => old,
        None => {
            return Err(AppError(
                Error::msg("New URL is required"),
                StatusCode::BAD_REQUEST,
            ));
        }
    };

    if !domain_regex.is_match(&new) {
        return Err(AppError(
            Error::msg("Incorrect format for new url"),
            StatusCode::BAD_REQUEST,
        ))
    }

    if !domain_regex.is_match(&old) {
        return Err(AppError(
            Error::msg("Incorrect format for old url"),
            StatusCode::BAD_REQUEST,
        ))
    }


    Ok((new, old))

}
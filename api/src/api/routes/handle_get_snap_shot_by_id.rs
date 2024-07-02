use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

use crate::{
    api::errors::AppError,
    models::{app_state::AppState, snap_shot_batch::SnapShotBatch},
    service::snap_shot_service,
};
pub async fn handle_get_snap_shot_by_id(
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<SnapShotBatch>, AppError> {
    let result = snap_shot_service::get_snap_shot_batch_by_id(id, state.db_pool.clone()).await?;

    if result.is_some() {
        return Ok(Json(result.unwrap()));
    } else {
        return Err(AppError(
            anyhow::Error::msg(format!("Snap shot batch with id {} not found", id)),
            axum::http::StatusCode::NOT_FOUND,
        ));
    }
}

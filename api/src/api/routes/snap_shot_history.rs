use std::sync::Arc;

use axum::{extract::State, Json};

use crate::{
    api::errors::AppError,
    models::{app_state::AppState, snap_shot_batch::SnapShotBatch},
    service::snap_shot_history_service,
};

pub async fn handle_get_snap_shot_history(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<SnapShotBatch>>, AppError> {
    let res = snap_shot_history_service::get_snap_shot_history(state.db_pool.clone())
        .await
        .map_err(|e| AppError(e, axum::http::StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(Json(res))
}

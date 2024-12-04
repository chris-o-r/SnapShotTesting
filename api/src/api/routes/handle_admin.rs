use std::{fs, sync::Arc};

use axum::{
    extract::State,
    routing, Router,
};
use crate::{
    api::errors::AppError,
    db::snapshot_store,
    models::    app_state::AppState,
    service::{snapshot_history_service, snapshot_job_service}, utils::env_variables::EnvVariables,
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/clean-up", routing::get(handle_clean_up))
}

async fn handle_clean_up(State(state): State<Arc<AppState>>) -> Result<(), AppError> {
    // Delete all jobs
    snapshot_job_service::clear_all_runnning_jobs(state.redis_pool.clone()).await?;
    // Delete all batches
    snapshot_history_service::delete_all_batches(state.db_pool.clone()).await?;
    // Remove all josb
    snapshot_store::delete_all_snapshots(&state.db_pool).await?;

    let folder_path = EnvVariables::new().assets_folder;
    fs::remove_dir_all(folder_path)?;

    
    Ok(())
}

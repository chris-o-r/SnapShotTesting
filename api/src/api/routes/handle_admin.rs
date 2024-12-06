use std::{fs, sync::Arc};

use axum::{
    extract::State,
    routing, Router,
};
use reqwest::StatusCode;

use utoipa::OpenApi;

use crate::{
    api::errors::AppError,
    db::snapshot_store,
    models::    app_state::AppState,
    service::{snapshot_history_service, snapshot_job_service}, utils::env_variables::EnvVariables,
};

#[derive(OpenApi)]
#[openapi(
    paths(handle_clean_up),
    tags((name = "Admin", description = "All Admin"))
)]
pub struct AdminDoc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/clean-up", routing::get(handle_clean_up))
}

#[utoipa::path(
    get,
    path = "api/admin/clean-up",
    responses(
        (status = 200, description = "Cleared all data"),
    ),
    tag = "Admin"
)]
async fn handle_clean_up(State(state): State<Arc<AppState>>) -> Result<(), AppError> {
    // Delete all jobs
    snapshot_job_service::clear_all_runnning_jobs(state.redis_pool.clone()).await?;
    // Delete all batches
    snapshot_history_service::delete_all_batches(state.db_pool.clone()).await?;
    // Remove all josb
    snapshot_store::delete_all_snapshots(&state.db_pool).await?;

    let folder_path = EnvVariables::new().assets_folder;
    
    match fs::exists(&folder_path) {
        Ok(_) => {
            fs::remove_dir_all(&folder_path).unwrap();
            Ok(())
        },
        Err(_) => {
            tracing::error!("Failed to delete folder {}", &folder_path);
            return Err(AppError(
                anyhow::Error::msg("Error deleting folders"),
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }
    
}

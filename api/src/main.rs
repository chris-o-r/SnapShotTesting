pub mod api;
pub mod db;
pub mod models;
pub mod service;
use api::routes::{
    handle_get_job_by_id, handle_get_snapshot_by_id::handle_get_snapshot_by_id,
    handle_snapshot::handle_snapshot, snapshot_history::handle_get_snapshot_history,
};
use axum::{
    routing::{get, post},
    Router,
};
use db::snapshot_batch_job_store;
use models::app_state::AppState;
use std::{net::SocketAddr, sync::Arc};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
    trace::{self, TraceLayer},
};
use tracing::Level;

#[tokio::main]
async fn main() {
    let app_state: Arc<AppState> = Arc::new(AppState::new().await);

    tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing_subscriber::filter::LevelFilter::INFO)
        .init();

    let port = match app_state.env_variables.port.parse::<u16>() {
        Ok(port) => port,
        Err(_) => {
            tracing::error!("Invalid port number. Exiting...");
            std::process::exit(1);
        }
    };

    match snapshot_batch_job_store::remove_all_jobs(&app_state.redis_pool).await {
        Ok(_) => {
            tracing::info!("All historical jobs cleaned");
        }
        Err(e) => {
            tracing::error!("Failed to remove all jobs: {}", e);
            std::process::exit(1);
        }
    }

    tokio::join!(serve(create_routes(app_state.clone()), port));
}

async fn serve(app: Router, port: u16) {
    let cors = CorsLayer::new()
        .allow_origin(Any) // Allow any origin
        .allow_methods(Any) // Allow any method (GET, POST, etc.)
        .allow_headers(Any); // Allow any header

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = match tokio::net::TcpListener::bind(addr).await {
        Ok(listener) => listener,
        Err(e) => {
            panic!("Failed to bind to port {}: {}", port, e);
        }
    };

    tracing::info!("Server Started on {}", listener.local_addr().unwrap());

    axum::serve(
        listener,
        app.layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(cors),
    )
    .await
    .unwrap();
}

fn create_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/snap-shot", post(handle_snapshot))
        .route("/snap-shot", get(handle_get_snapshot_history))
        .route("/snap-shot/:id", get(handle_get_snapshot_by_id))
        .route("/jobs/:id", get(handle_get_job_by_id::handle_get_job_by_id))
        .route(
            "/jobs",
            get(api::routes::handle_get_all_running_jobs::handle_get_all_running_jobs),
        )
        .with_state(app_state.clone())
}

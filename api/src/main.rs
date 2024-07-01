pub mod api;
pub mod db;
pub mod models;
pub mod service;
use api::routes::{snap_shot::handle_snap_shot, snap_shot_history::handle_get_snap_shot_history};
use axum::{
    routing::{get, post},
    Router,
};
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

    tokio::join!(serve(
        create_routes(app_state.clone()),
        app_state.env_variables.port.parse().unwrap()
    ));
}

fn create_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/snap-shot", post(handle_snap_shot))
        .route("/snap-shot", get(handle_get_snap_shot_history))
        .with_state(app_state.clone())
}

async fn serve(app: Router, port: u16) {
    let cors = CorsLayer::new()
        .allow_origin(Any) // Allow any origin
        .allow_methods(Any) // Allow any method (GET, POST, etc.)
        .allow_headers(Any); // Allow any header

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("Server Started on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        app.layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::DEBUG))
                .on_response(trace::DefaultOnResponse::new().level(Level::DEBUG)),
        )
        .layer(cors),
    )
    .await
    .unwrap();
}

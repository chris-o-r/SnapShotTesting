use std::sync::Arc;

use axum::{routing::get, Router};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
    trace::{self, TraceLayer},
};
use tracing::Level;
use utoipa_swagger_ui::SwaggerUi;

use crate::{models::app_state::AppState, utils::env_variables};

pub mod errors;
pub mod routes;
pub mod swagger_config;

pub async fn serve() {
    let app_state: Arc<AppState> = Arc::new(AppState::new().await);
    let env_variables = env_variables::EnvVariables::new();

    tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing_subscriber::filter::LevelFilter::INFO)
        .init();

    let port = match env_variables.port.parse::<u16>() {
        Ok(port) => port,
        Err(_) => {
            tracing::error!("Invalid port number. Exiting...");
            std::process::exit(1);
        }
    };

    let base_url = env_variables.base_url.clone();

    let url = format!("{}:{}", base_url, port);

    let listener = match tokio::net::TcpListener::bind(url.clone()).await {
        Ok(listener) => listener,
        Err(e) => {
            tracing::error!("Failed to listen on url {}", url);
            tracing::error!("Error: {}", e);
            panic!("Failed to listen on url {}", url);
        }
    };

    tracing::info!("Server Started on {}", listener.local_addr().unwrap());

    let app = create_routes(app_state);

    axum::serve(
        listener,
        app.layer(crete_trace_layer()).layer(create_cors_layer()),
    )
    .await
    .unwrap();
}

fn crete_trace_layer(
) -> TraceLayer<tower_http::classify::SharedClassifier<tower_http::classify::ServerErrorsAsFailures>>
{
    TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(Level::DEBUG))
        .on_response(trace::DefaultOnResponse::new().level(Level::DEBUG))
}

fn create_cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any) // Allow any origin
        .allow_methods(Any) // Allow any method (GET, POST, etc.)
        .allow_headers(Any) // Allow any header
}

fn create_routes(app_state: Arc<AppState>) -> Router {
    let env_variables = env_variables::EnvVariables::new();

    Router::new()
        .route("/ping", get(routes::handle_ping::handler))
        .nest_service("/api/assets", ServeDir::new(env_variables.assets_folder))
        .nest("/api/snap-shots", routes::handle_snapshot::router())
        .nest("/api/admin", routes::handle_admin::router())
        .merge(
            SwaggerUi::new("/swagger-ui")
                .url("/api-docs/openapi.json", swagger_config::generate_doc()),
        )
        .with_state(app_state.clone())
}

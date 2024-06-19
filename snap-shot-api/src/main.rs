pub mod api;
use api::routes::snap_shot::handle_snap_shot;
use axum::{routing::post, Router};
use std::net::SocketAddr;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
    trace::{self, TraceLayer},
};
use tracing::Level;

#[tokio::main]
async fn main() {
    tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing_subscriber::filter::LevelFilter::INFO)
        .init();
    tokio::join!(serve(using_serve_file_from_a_route(), 3307));
}

fn using_serve_file_from_a_route() -> Router {
    Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/snap-shot", post(handle_snap_shot))
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
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(cors),
    )
    .await
    .unwrap();
}

pub mod api;
use api::routes::snap_shot::handle_snap_shot;
use axum::{routing::get, Router};
use std::net::SocketAddr;
use tower_http::{
    services::ServeFile,
    trace::{self, TraceLayer},
};
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing_subscriber::filter::LevelFilter::INFO)
        .init();
    tokio::join!(serve(using_serve_file_from_a_route(), 3307));
}

fn using_serve_file_from_a_route() -> Router {
    Router::new()
        .route_service("/foo", ServeFile::new("assets/index.html"))
        .route("/snap-shot", get(handle_snap_shot))
}

async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("Server Started on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        app.layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        ),
    )
    .await
    .unwrap();
}

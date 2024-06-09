pub mod api;
pub mod lib;
use api::routes::snap_shot::handle_snap_shot;
use axum::{routing::get, Router};
use std::net::SocketAddr;
use tower_http::{services::ServeFile, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_static_file_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
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
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.layer(TraceLayer::new_for_http()))
        .await
        .unwrap();
}

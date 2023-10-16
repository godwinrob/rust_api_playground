use axum::{
    routing::{get, post},
    Router,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {

    // add tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "rust_api_playground=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // add CORS
    let cors = CorsLayer::new().allow_origin(Any);

    // define router paths
    let app = Router::new()
        .route("/", get(|| async {"hello, world!"}))
        .layer(cors);

    // define address and start server
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 9090));
    tracing::debug!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("failed to start server")
}
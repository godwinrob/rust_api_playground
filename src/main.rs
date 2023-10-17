use std::env;
use axum::{routing::{get}, Router, Extension};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tower_http::cors::{Any, CorsLayer};
use sqlx::{postgres::PgPoolOptions, postgres::PgPool};

#[tokio::main]
async fn main() {

    // create connection to DB
    let pool = db_connect().await;

    // add tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            env::var("RUST_LOG").unwrap_or_else(|_| "rust_api_playground=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // add CORS
    let cors = CorsLayer::new().allow_origin(Any);

    // define router paths and add middleware layers
    let app = Router::new()
        .route("/", get(|| async {"hello, world!"}))
        .layer(cors)
        .layer(Extension(pool));

    // define address and start server
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 9090));
    tracing::debug!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("failed to start server")
}

async fn db_connect() -> PgPool {
    let user = get_env_or_default("DB_USER", "admin");
    let password = get_env_or_default("DB_PASSWORD", "");
    let host = get_env_or_default("DB_HOST", "localhost");
    let port = get_env_or_default("DB_PORT", "26257");
    let name = get_env_or_default("DB_NAME", "defaultdb");
    let ssl_mode = get_env_or_default("SSL_MODE", "disable");
    let connection_url = if password.is_empty() {
        format!("postgresql://{}@{}:{}/{}?sslmode={}", user, host, port, name, ssl_mode)
    } else {
        format!(
            "postgresql://{}:{}@{}:{}/{}?sslmode={}",
            user, password, host, port, name, ssl_mode
        )
    };
    println!("Connecting to db: {}.", connection_url);

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&connection_url)
        .await
        .expect("Failed to connect to DB.")
}

fn get_env_or_default(env: &str, default: &str) -> String {
    return env::var(env).unwrap_or_else(|_| default.to_owned());
}
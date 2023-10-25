use axum::{
    routing::{get, post},
    Extension, Router,
};
use once_cell::sync::Lazy;
use sqlx::{postgres::PgPool, postgres::PgPoolOptions};
use std::env;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

// import module
mod controllers;
mod error;
mod models;
mod utils;

// secret key for JWT token
static KEYS: Lazy<models::auth::Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "my_secret_key_string".to_owned());
    models::auth::Keys::new(secret.as_bytes())
});

#[tokio::main]
async fn main() {
    // create connection to DB
    let pool = db_connect().await;

    // add tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    // add CORS
    let cors = CorsLayer::new().allow_origin(Any);

    // define router paths and add middleware layers
    let app = Router::new()
        // following routes can be accessed publicly
        .route("/", get(|| async { "hello, world!" }))
        .route("/register", post(controllers::auth::register))
        .route("/login", post(controllers::auth::login))
        // following routes require token from login
        .route("/delete", post(controllers::user::delete_user))
        .route("/update", post(controllers::user::update_user))
        .route("/user_profile", get(controllers::user::user_profile))
        .layer(cors)
        .layer(Extension(pool))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    // define address and start server
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 9090));
    tracing::info!("Listening on {}", addr);
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
        format!(
            "postgresql://{}@{}:{}/{}?sslmode={}",
            user, host, port, name, ssl_mode
        )
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

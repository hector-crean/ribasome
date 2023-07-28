pub mod conf;
pub mod consts;
pub mod csv_ops;
pub mod db;
pub mod errors;
pub mod models;

use axum::{
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use http::Method;
use std::sync::Arc;
use tokio::sync::RwLock;

use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::{env, net::SocketAddr};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
pub struct AppState {
    client: Arc<RwLock<edgedb_tokio::Client>>,
}

pub async fn initialize_db() -> Result<Arc<RwLock<edgedb_tokio::Client>>, edgedb_tokio::Error> {
    let pool = edgedb_tokio::create_client().await?;

    pool.ensure_connected().await?;
    Ok(Arc::new(RwLock::new(pool)))
}

pub async fn api() -> errors::Result<axum::Router> {
    dotenv().ok();

    // let db_url: String = env::var("DATABASE_URL")?;

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO));

    // setup connection pool
    let client = initialize_db().await?;

    // sqlx::migrate!("./migrations").run(&pool).await?;

    let state = AppState {
        client: client.clone(),
    };

    let cors_layer = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);

    let router = Router::new()
        .layer(cors_layer)
        .layer(trace_layer)
        .with_state(state);

    Ok(router)
}

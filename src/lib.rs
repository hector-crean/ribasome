pub mod csv_ops;
pub mod errors;
pub mod models;
pub mod services;

use axum::{
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use http::Method;
use services::{marker_3d, markup, thread, user};
use sqlx::{postgres::PgPoolOptions, PgConnection, Pool, Postgres};
use std::env;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
pub struct AppState {
    pool: Pool<Postgres>,
}

pub async fn api() -> errors::Result<axum::Router> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL")?;

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO));

    // setup connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("can't connect to database");

    sqlx::migrate!("./migrations").run(&pool).await?;

    let state = AppState { pool };
    let cors_layer = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);

    let router = Router::new()
        .layer(cors_layer)
        .layer(trace_layer)
        .route("/users", post(user::post::create_user))
        .route("/comments", post(markup::post::create_comment))
        .route("/marker3ds", post(marker_3d::post::create_marker_3d))
        .route("/posts", post(thread::post::create_post))
        .with_state(state);

    let api = Router::new().nest("/:version/api", router);

    Ok(api)
}

pub mod csv_ops;
pub mod errors;
pub mod models;
pub mod services;

use axum::{
    routing::{get, post},
    Router,
};
use http::Method;
use services::{marker_3d, markup, thread, user};
use sqlx::{Pool, Postgres};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

#[derive(Clone)]
pub struct AppState {
    pool: Pool<Postgres>,
}

impl AppState {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn router(self) -> errors::Result<axum::Router> {
        let trace_layer = TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
            .on_response(DefaultOnResponse::new().level(Level::INFO));

        sqlx::migrate!("./migrations").run(&self.pool).await?;

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
            .with_state(self);

        let api = Router::new().nest("/:version/api", router);

        Ok(api)
    }
}

use dotenv::dotenv;
use ribasome_server::{errors, AppState};
use sqlx::postgres::PgPoolOptions;

use std::{
    convert::AsRef,
    env,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use tower_http::{
    cors::{Any, CorsLayer},
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use axum::{
    extract::ConnectInfo,
    routing::{get, post},
    Json, Router,
};
use pbkdf2::password_hash::rand_core::OsRng;
use rand_chacha::ChaCha8Rng;
use rand_core::{RngCore, SeedableRng};

#[tokio::main]
async fn main() -> errors::Result<()> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 1690));

    tracing::debug!("listening on {}", addr);

    let db_url = env::var("DATABASE_URL")?;

    // setup connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("can't connect to database");

    let random = ChaCha8Rng::seed_from_u64(OsRng.next_u64());

    let router = AppState::new(pool, Arc::new(Mutex::new(random)))
        .router()
        .await?;

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        extract::connect_info::MockConnectInfo,
        http::{self, Request, StatusCode},
    };
    use ribasome_server::{
        models::user::{Role, User},
        services::user::post::CreateUser,
    };
    use serde_json::{json, Value};
    use std::net::SocketAddr;
    use tokio::net::TcpListener;
    use tower::Service; // for `call`
    use tower::ServiceExt; // for `oneshot` and `ready`

    #[tokio::test]
    async fn hello_world() -> errors::Result<()> {
        dotenv().ok();

        let addr = SocketAddr::from(([127, 0, 0, 1], 1691));

        let db_url = env::var("DATABASE_URL")?;

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await
            .expect("can't connect to database");

        let random = ChaCha8Rng::seed_from_u64(OsRng.next_u64());

        let router = AppState::new(pool, Arc::new(Mutex::new(random)))
            .router()
            .await?;

        tokio::spawn(async move {
            axum::Server::bind(&addr)
                .serve(router.into_make_service())
                .await
                .unwrap();
        });

        let client = reqwest::Client::new();

        // Create a `CreateUser` instance
        let user = CreateUser {
            username: "leon_cav".to_string(),
            email: "leon@r42.com".to_string(),
            password: "leonardo".to_string(),
            role: Role::User,
        };

        let resp = client
            .post(format!("http://{}/v1/api/users", addr))
            .json(&json!(user))
            .send()
            .await?
            .json::<User>()
            .await?;

        println!("{:?}", &resp);

        assert_eq!(1, 1);

        Ok(())
    }
}

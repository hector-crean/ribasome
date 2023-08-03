use dotenv::dotenv;
use ribasome_server::{errors, AppState};
use sqlx::postgres::PgPoolOptions;

use std::{
    env,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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

    use ribasome_server::{
        models::{linear_algebra::Vec3f64, post::Post},
        services::{
            marker_3d::post::CreateMarker3d,
            thread::post::CreatePost,
            user::post::{CreateUser, CreateUserResponse},
        },
    };
    use serde_json::json;
    use std::net::SocketAddr;

    // for `call`
    // for `oneshot` and `ready`

    async fn router_instance() -> errors::Result<(SocketAddr, axum::Router)> {
        dotenv().ok();

        let addr = SocketAddr::from(([127, 0, 0, 1], 1691));

        let db_url = env::var("DATABASE_URL").unwrap(); // Unwrap here for simplicity in tests

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await
            .expect("can't connect to the database");

        let random = ChaCha8Rng::seed_from_u64(OsRng.next_u64());

        let router = AppState::new(pool, Arc::new(Mutex::new(random)))
            .router()
            .await?;

        Ok((addr, router))
    }

    #[tokio::test]
    async fn mock_create_user() -> errors::Result<()> {
        dotenv().ok();

        let (addr, router) = router_instance().await?;

        tokio::spawn(async move {
            axum::Server::bind(&addr)
                .serve(router.into_make_service())
                .await
                .unwrap();
        });

        let client = reqwest::Client::new();

        // Create a `CreateUser` instance
        let user: CreateUser = rand::random();

        let CreateUserResponse {
            session_token,
            user_id,
        } = client
            .post(format!("http://{}/v1/api/users", addr))
            .json(&json!(user))
            .send()
            .await?
            .json::<CreateUserResponse>()
            .await?;

        let post = CreatePost {
            user_id,
            rich_text: "Some top quality content".to_string(),
            create_marker_3d: Some(CreateMarker3d::Point3d {
                coord: Vec3f64::new(1., 2., 3.),
            }),
        };

        let post_resp = client
            .post(format!("http://{}/v1/api/posts", addr))
            .json(&json!(post))
            .send()
            .await?
            .json::<Post>()
            .await?;

        println!("{:?}", &post_resp);

        assert_eq!(1, 1);

        Ok(())
    }
}

// create user
// create post
//

use aws_sdk_s3::config::{Credentials, Region};
use dotenv::dotenv;
use ribasome_server::services::s3::S3Bucket;
use ribasome_server::{errors, AppState};
use sqlx::postgres::PgPoolOptions;
use std::convert::From;

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

    let aws_key = std::env::var("AWS_ACCESS_KEY_ID").expect("Failed to get AWS key.");
    let aws_key_secret =
        std::env::var("AWS_SECRET_ACCESS_KEY").expect("Failed to get AWS secret key.");
    let s3_region = std::env::var("AWS_REGION").unwrap_or("eu-west-2".to_string());
    let aws_bucket = std::env::var("S3_BUCKET_NAME").expect("Failed to get AWS Bucket key");
    let aws_config = aws_sdk_s3::config::Builder::new()
        .region(Region::new(s3_region.clone()))
        .credentials_provider(Credentials::new(
            aws_key,
            aws_key_secret,
            None,
            None,
            "loaded-from-custom-env",
        ))
        .build();

    let bucket = S3Bucket::new(aws_config, &s3_region, &aws_bucket);

    let random = ChaCha8Rng::seed_from_u64(OsRng.next_u64());

    let router = AppState::new(pool, bucket, Arc::new(Mutex::new(random)))
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
        models::{post::Post, user::User},
        services::{
            marker_3d::post::CreateMarker3d,
            s3::S3Bucket,
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

        let addr = SocketAddr::from(([127, 0, 0, 1], 1699));

        let db_url = env::var("DATABASE_URL").unwrap(); // Unwrap here for simplicity in tests

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await
            .expect("can't connect to the database");

        let aws_key = std::env::var("AWS_ACCESS_KEY_ID").expect("Failed to get AWS key.");
        let aws_key_secret =
            std::env::var("AWS_SECRET_ACCESS_KEY").expect("Failed to get AWS secret key.");
        let s3_region = std::env::var("AWS_REGION").unwrap_or("eu-west-2".to_string());
        let aws_bucket = std::env::var("S3_BUCKET_NAME").expect("Failed to get AWS Bucket key");
        let aws_config = aws_sdk_s3::config::Builder::new()
            .region(Region::new(s3_region.clone()))
            .credentials_provider(Credentials::new(
                aws_key,
                aws_key_secret,
                None,
                None,
                "loaded-from-custom-env",
            ))
            .build();

        let bucket = S3Bucket::new(aws_config, &s3_region, &aws_bucket);

        let random = ChaCha8Rng::seed_from_u64(OsRng.next_u64());

        let router = AppState::new(pool, bucket, Arc::new(Mutex::new(random)))
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
            session_token: _,
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
                coord: bevy::math::Vec3::new(1., 2., 3.).into(),
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

    #[tokio::test]
    async fn mock_list_users() -> errors::Result<()> {
        dotenv().ok();

        let (addr, router) = router_instance().await?;

        tokio::spawn(async move {
            axum::Server::bind(&addr)
                .serve(router.into_make_service())
                .await
                .unwrap();
        });

        let client = reqwest::Client::new();

        let users = client
            .get(format!("http://{}/v1/api/users", addr))
            .send()
            .await?
            .json::<Vec<User>>()
            .await?;

        println!("{:?}", &users);

        assert_eq!(1, 1);

        Ok(())
    }
}

// create user
// create post
//

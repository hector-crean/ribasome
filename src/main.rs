use bibe_server::{api, errors};

#[tokio::main]
async fn main() -> errors::Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 1690));

    let router = api().await?;

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();

    Ok(())
}

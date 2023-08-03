use std::path::Path;

use aws_sdk_s3::{client::Client, types::ByteStream, Credentials, Region};

use tokio::io::AsyncReadExt;

#[derive(Clone)]
pub struct S3Bucket {
    client: Client,
    bucket_name: String,
}

#[derive(thiserror::Error, Debug)]
pub enum S3Error {
    #[error(transparent)]
    S3Error(aws_sdk_s3::Error),
}

impl S3Bucket {
    pub fn new(config: Option<aws_sdk_s3::Config>) -> Self {
        let config = match config {
            Some(config) => config,
            None => {
                let aws_key = std::env::var("AWS_ACCESS_KEY_ID").expect("Failed to get AWS key.");
                let aws_key_secret =
                    std::env::var("AWS_SECRET_ACCESS_KEY").expect("Failed to get AWS secret key.");
                let aws_region = std::env::var("AWS_REGION").unwrap_or("eu-west-2".to_string());

                let aws_config = aws_sdk_s3::config::Builder::new()
                    .region(Region::new(aws_region))
                    .credentials_provider(Credentials::new(
                        aws_key,
                        aws_key_secret,
                        None,
                        None,
                        "loaded-from-custom-env",
                    ))
                    .build();

                aws_config
            }
        };

        Self {
            client: aws_sdk_s3::Client::from_conf(config),
            bucket_name: std::env::var("AWS_S3_BUCKET_NAME").unwrap(),
        }
    }

    pub fn url(&self, key: &str) -> String {
        format!(
            "https://{}.s3.{}.amazonaws.com/{key}",
            std::env::var("AWS_S3_BUCKET_NAME").unwrap(),
            std::env::var("AWS_REGION").unwrap(),
        )
    }

    async fn upload_object<P: AsRef<Path>>(
        &self,
        file_path: P,
        key: &str,
    ) -> Result<String, S3Error> {
        let mut file = tokio::fs::File::open(file_path).await.unwrap();

        let size_estimate: usize = file
            .metadata()
            .await
            .map(|md| md.len())
            .unwrap_or(1024)
            .try_into()
            .expect("file too big");

        let mut contents = Vec::with_capacity(size_estimate);
        file.read_to_end(&mut contents).await.unwrap();

        let _res = self
            .client
            .put_object()
            .bucket(&self.bucket_name)
            .key(key)
            .body(ByteStream::from(contents))
            .send()
            .await
            .expect("Failed to put object");

        Ok(self.url(key))
    }

    pub async fn delete_file(&self, key: &str) -> bool {
        self.client
            .delete_object()
            .bucket(&self.bucket_name)
            .key(key)
            .send()
            .await
            .is_ok()
    }
}

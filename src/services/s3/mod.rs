use std::path::Path;

use aws_sdk_s3::{client::Client, types::ByteStream, Credentials, Region};
use dotenv::dotenv;
use rand::Rng;
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
    pub fn new(config: aws_sdk_s3::Config, bucket_name: &str) -> Self {
        Self {
            client: aws_sdk_s3::Client::from_conf(config),
            bucket_name: bucket_name.to_string(),
        }
    }

    pub fn url(&self, key: &str) -> String {
        format!(
            "https://{}.s3.{}.amazonaws.com/{key}",
            std::env::var("AWS_S3_BUCKET_NAME").unwrap(),
            std::env::var("S3_REGION").unwrap(),
        )
    }

    pub async fn upload_object<P: AsRef<Path>>(
        &self,
        file_path: P,
        key: &str,
    ) -> Result<String, S3Error> {
        let mut file = tokio::fs::File::open(file_path)
            .await
            .expect("File not found");

        let size_estimate: usize = file
            .metadata()
            .await
            .map(|md| md.len())
            .unwrap_or(1024)
            .try_into()
            .expect("file too big");

        let mut contents = Vec::with_capacity(size_estimate);
        file.read_to_end(&mut contents)
            .await
            .expect("Read to end of file failed");

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

#[cfg(test)]
mod tests {
    use rand::distributions::Alphanumeric;

    use super::*;

    // for `call`
    // for `oneshot` and `ready`

    async fn bucket_singleton() -> S3Bucket {
        dotenv().ok();

        let aws_key = std::env::var("AWS_ACCESS_KEY_ID").expect("Failed to get AWS key.");
        let aws_key_secret =
            std::env::var("AWS_SECRET_ACCESS_KEY").expect("Failed to get AWS secret key.");
        let s3_region = std::env::var("S3_REGION").unwrap_or("eu-west-2".to_string());
        let aws_bucket = std::env::var("S3_BUCKET_NAME").expect("Failed to get AWS Bucket key");
        let aws_config = aws_sdk_s3::config::Builder::new()
            .region(aws_sdk_s3::Region::new(s3_region))
            .credentials_provider(aws_sdk_s3::Credentials::new(
                aws_key,
                aws_key_secret,
                None,
                None,
                "loaded-from-custom-env",
            ))
            .build();

        S3Bucket::new(aws_config, &aws_bucket)
    }

    #[tokio::test]
    async fn upload_gltf() -> Result<(), S3Error> {
        let key: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        let bucket = bucket_singleton().await;

        let url = bucket.upload_object(
            "c:\\Users\\Hector.C\\desktop\\projects\\ribasome\\assets\\glb\\Eye_AMD_Atrophy.glb",
            format!("{}.glb", &key).as_str(),
        ).await?;

        println!("{}", url);

        Ok(())
    }
}

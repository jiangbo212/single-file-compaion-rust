use std::error::Error;
use std::io;
use std::io::Read;
use chrono::Local;
use dotenv::dotenv;
use log::info;
use s3::creds::Credentials;
use s3::{Bucket, Region};
use s3::error::S3Error;
use s3::request::ResponseData;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
struct SingleFile {
    method: String,
    #[serde(rename = "pageData")]
    page_data: PageData,
}

#[derive(Serialize, Deserialize)]
struct PageData {
    content: String,
    filename: String
}

async fn save_minio_s3(filename: &str, content: &[u8]) -> Result<ResponseData, S3Error> {
    let bucket_name = dotenv::var("MINIO_BUCKET_NAME").unwrap_or(format!("test"));
    let region = dotenv::var("MINIO_REGION").unwrap_or(format!(""));
    let endpoint = dotenv::var("MINIO_ENDPOINT").unwrap_or(format!(""));
    let access_key = dotenv::var("MINIO_ACCESS_KEY").unwrap_or(format!(""));
    let secret_key = dotenv::var("MINIO_SECRET_KEY").unwrap_or(format!(""));
    let region = Region::Custom {
        region,
        endpoint
    };
    let credentials = Credentials::new(
        Some(access_key.as_str()),
        Some(secret_key.as_str()),
        None,
        None,
        None
    ).map_err(|e| S3Error::Credentials(e))?;

    let now = Local::now();
    let current_date = now.format("%Y%m%d");

    let bucket =
        Bucket::new(bucket_name.as_str(), region.clone(), credentials.clone()).unwrap().with_path_style();

    bucket.put_object_with_content_type(format!("{}/{}", current_date, filename), content, "text/html").await
}

pub fn read_input<R: Read>(mut input: R) -> Result<Value, Box<dyn Error>> {
    let mut buf = [0; 4];
    let length =  input.read_exact(&mut buf).map(|()| u32::from_ne_bytes(buf))?;
    info!("Found length: {}", length);
    let mut buffer = vec![0; length as usize];
    input.read_exact(&mut buffer)?;
    let value = serde_json::from_slice(&buffer)?;
    Ok(value)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv()?;
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("receive SingleFile Plugins native message");
    match read_input(io::stdin()) {
        Ok(value) => {
            let single_file = match serde_json::from_value::<SingleFile>(value) {
                Ok(single_file) => single_file,
                Err(e) => {
                    info!("serde data to SingleFile struct error。{:?}", e);
                    return Ok(());
                }
            };
            if single_file.method == "save" {
                if let Err(e) = save_minio_s3(&single_file.page_data.filename, &single_file.page_data.content.as_bytes()).await {
                    info!("save to minio-s3 error: {:?}", e);
                } else {
                    info!("save to minio-s3 success.");
                }
            } else {
                info!("SingleFile send {} message, not need save to minio-s3", &single_file.method);
            }
        },
        Err(e) => {
            info!("error:{:?}", e);
        }
    }
    Ok(())
}

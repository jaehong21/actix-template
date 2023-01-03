use crate::controller::JsonMessage;
use actix_web::HttpResponse;
use s3::Bucket;
use std::sync::Arc;

pub fn getenv(key: &str) -> String {
    std::env::var(key).expect(&format!("{} must be set", key))
}

pub async fn upload_s3(
    s3: Arc<Bucket>,
    filename: String,
    data: &[u8],
    content_type: &str,
) -> HttpResponse {
    match s3
        .put_object_with_content_type(filename.clone(), data, content_type)
        .await
    {
        Ok(_) => HttpResponse::Created().json(JsonMessage {
            msg: format!("{}/{}", getenv("AWS_S3_BUCKET_URL"), filename),
        }),
        Err(_) => HttpResponse::InternalServerError().json(JsonMessage {
            msg: format!("Failed to upload file {} to s3", filename),
        }),
    }
}

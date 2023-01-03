use crate::database::SurrealClient;
use crate::utils::getenv;
use actix_web::dev::ResourcePath;
use dotenv::dotenv;
use s3::creds::Credentials;
use s3::{Bucket, BucketConfiguration, Region};
use surrealdb_rs::param::Root;
use surrealdb_rs::protocol::Ws;
use surrealdb_rs::Surreal;

pub fn init_env() {
    dotenv().ok();
    let env_list = [
        "DATABASE_URL",
        "DATABASE_USER",
        "DATABASE_PASSWORD",
        "DATABASE_NAMESPACE",
        "DATABASE_NAME",
        "AWS_S3_BUCKET_NAME",
        "AWS_S3_BUCKET_URL",
        "AWS_ACCESS_KEY",
        "AWS_SECRET_KEY",
        "JWT_SECRET_KEY",
        "JWT_EXPIRE_TIME",
    ];
    for env in env_list {
        getenv(env);
    }
}

pub async fn init_s3_storage() -> Bucket {
    let bucket = Bucket::new_with_path_style(
        getenv("AWS_S3_BUCKET_NAME").as_str(),
        "ap-northeast-2".parse().unwrap(),
        Credentials {
            access_key: Some(getenv("AWS_ACCESS_KEY")),
            secret_key: Some(getenv("AWS_SECRET_KEY")),
            // access_key: Some("root".to_owned()),
            // secret_key: Some("password".to_owned()),
            security_token: None,
            session_token: None,
        },
    )
    .expect("Failed to initiate bucket");

    let (_, code) = bucket.head_object("/").await.expect("Failed to get bucket");
    if code == 404 {
        panic!("Bucket not found");
    }

    bucket
}

pub async fn init_database() -> SurrealClient {
    let client = Surreal::connect::<Ws>(getenv("DATABASE_URL"))
        .await
        .expect("Failed to connect to Surreal database");
    // Sign in as a namespace, database, or root user

    client
        .signin(Root {
            username: getenv("DATABASE_USER").as_str(),
            password: getenv("DATABASE_PASSWORD").as_str(),
        })
        .await
        .expect("Failed to sign in");
    // Select a specific namespace and database
    client
        .use_ns(getenv("DATABASE_NAMESPACE"))
        .use_db(getenv("DATABASE_NAME"))
        .await
        .expect("Failed to select namespace and database");

    client
}

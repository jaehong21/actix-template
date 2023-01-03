extern crate core;

use crate::controller::{photo, user};
use crate::database::Client;
use actix_web::{web, App, HttpServer};
use std::sync::Arc;

mod controller;
mod database;
mod entity;
mod init;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init::init_env();
    let client = init::init_database().await;
    let s3_client = init::init_s3_storage().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Client {
                surreal: Arc::new(client.clone()),
                s3: Arc::new(s3_client.clone()),
            }))
            // [/]
            .service(controller::routes())
            // [/user]
            .service(user::routes())
            // [/photo]
            .service(photo::routes())
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}

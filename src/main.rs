use crate::controller::user;
use crate::database::SurrealDb;
use actix_web::{guard, web, App, HttpServer};
use std::sync::Arc;

mod controller;
mod database;
pub mod entity;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = database::connect().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(SurrealDb {
                client: Arc::new(client.clone()),
            }))
            .service(
                web::scope("/")
                    // .guard(guard::Header("Accept", "*/*"))
                    .route("", web::get().to(controller::index)),
            )
            // [/user]
            .service(user::routes())
    })
    .bind(("127.0.0.1", 9000))?
    .run()
    .await
}

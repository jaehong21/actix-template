mod jwt;
pub mod photo;
pub mod user;

use actix_web::{web, Responder, Scope};
use serde::Serialize;

#[derive(Serialize)]
pub struct JsonMessage {
    pub msg: String,
}

pub fn routes() -> Scope {
    web::scope("/")
        // .guard(guard::Header("Accept", "*/*"))
        .route("", web::get().to(index))
}

async fn index() -> impl Responder {
    "OK"
}

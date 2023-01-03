mod jwt;
pub mod user;

use actix_web::Responder;
use serde::Serialize;

#[derive(Serialize)]
pub struct JsonMessage {
    pub msg: String,
}

pub async fn index() -> impl Responder {
    // req: HttpRequest in parameter
    // println!("{:?}", req);
    "OK"
}

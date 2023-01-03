use crate::controller::JsonMessage;
use actix_multipart::Multipart;
use actix_web::{web, Error, HttpResponse, Scope};
// for .next() and .try_next()
use crate::database::Client;
use crate::utils::upload_s3;
use futures::{StreamExt, TryStreamExt};
use uuid::Uuid;

pub fn routes() -> Scope {
    web::scope("/photo")
        // .guard(guard::Header("Accept", "*/*"))
        .route("", web::post().to(upload))
}

async fn upload(mut payload: Multipart, db: web::Data<Client>) -> Result<HttpResponse, Error> {
    #[allow(unused_assignments)]
    let mut filename = String::new();

    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition();
        filename = match content_type.get_filename() {
            Some(name) => name.to_owned(),
            None => {
                return Ok(HttpResponse::InternalServerError().json(JsonMessage {
                    msg: "No filename provided".to_owned(),
                }))
            }
        };
        filename = format!("{}:{}", Uuid::new_v4(), filename).replace(" ", "_");
        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            // return HttpResponse
            return Ok(upload_s3(db.s3.clone(), filename, &data, "image").await);
        }
    }
    Ok(HttpResponse::InternalServerError().json(JsonMessage {
        msg: "Unknown error".to_owned(),
    }))
}

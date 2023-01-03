use crate::controller::jwt::{sign_jwt, validate_request};
use crate::controller::JsonMessage;
use crate::database::{user, SurrealDb};
use actix_web::{web, HttpRequest, HttpResponse, Responder, Scope};
use serde::{Deserialize, Serialize};

pub fn routes() -> Scope {
    web::scope("/user")
        .route("/register", web::post().to(register))
        .route("/login", web::post().to(login))
        .route("", web::get().to(find_all_user))
        .route("/{id}", web::get().to(find_user))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
}

/// [POST /user/register] creates user
async fn register(body: web::Json<CreateUserRequest>, db: web::Data<SurrealDb>) -> impl Responder {
    match user::create_user(
        db.client.clone(),
        String::from(&body.username),
        String::from(&body.password),
    )
    .await
    {
        Ok(user) => HttpResponse::Created().json(user),
        Err(e) => HttpResponse::InternalServerError().json(e),
    }
}

/// [POST /user/login] creates user
async fn login(body: web::Json<CreateUserRequest>, db: web::Data<SurrealDb>) -> impl Responder {
    let user = match user::find_user_by_username(db.client.clone(), body.username.clone()).await {
        Ok(user) => user,
        Err(e) => {
            return HttpResponse::InternalServerError().json(e);
        }
    };
    return if let Some(user) = user {
        if user.verify_password(&body.password) {
            let token = sign_jwt(user.username);
            HttpResponse::Created().json(JsonMessage { msg: token })
        } else {
            HttpResponse::Unauthorized().json(JsonMessage {
                msg: "Wrong password".to_owned(),
            })
        }
    } else {
        HttpResponse::Unauthorized().json(JsonMessage {
            msg: "User not exist".to_owned(),
        })
    };
}

/// [GET /user] finds all users (limit 100)
async fn find_all_user(db: web::Data<SurrealDb>) -> impl Responder {
    match user::find_all_user(db.client.clone()).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::InternalServerError().json(e),
    }
}

/// [GET /user/{id}] finds all users (limit 100)
async fn find_user(id: web::Path<String>, db: web::Data<SurrealDb>) -> impl Responder {
    match user::find_user_by_id(db.client.clone(), format!("{id}")).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::InternalServerError().json(e),
    }
}

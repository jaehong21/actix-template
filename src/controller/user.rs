use crate::controller::jwt::sign_jwt;
use crate::controller::JsonMessage;
use crate::database::{user, Client};
use actix_web::{web, HttpResponse, Responder, Scope};
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
async fn register(body: web::Json<CreateUserRequest>, db: web::Data<Client>) -> impl Responder {
    match user::create_user(
        db.surreal.clone(),
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
async fn login(body: web::Json<CreateUserRequest>, db: web::Data<Client>) -> impl Responder {
    let user = match user::find_user_by_username(db.surreal.clone(), body.username.clone()).await {
        Ok(user) => user,
        Err(e) => {
            return HttpResponse::InternalServerError().json(e);
        }
    };
    return if let Some(user) = user {
        if user.verify_password(&body.password) {
            let token = sign_jwt(user.username);
            // cookie build
            /*let cookie = Cookie::build("refresh_token", token.clone())
            .path("/")
            // .domain("localhost")
            // .secure(true)
            .http_only(true)
            .expires(
                // add 2weeks to now
                OffsetDateTime::from_unix_timestamp(chrono::Utc::now().timestamp() + 1_209_600)
                    .expect("Cannot parse timestamp to OffsetDateTime"),
            )
            .finish();*/

            HttpResponse::Created()
                // .cookie(cookie)
                .json(JsonMessage { msg: token })
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
async fn find_all_user(db: web::Data<Client>) -> impl Responder {
    match user::find_all_user(db.surreal.clone()).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::InternalServerError().json(e),
    }
}

/// [GET /user/{id}] finds all users (limit 100)
async fn find_user(id: web::Path<String>, db: web::Data<Client>) -> impl Responder {
    match user::find_user_by_id(db.surreal.clone(), format!("{id}")).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::InternalServerError().json(e),
    }
}

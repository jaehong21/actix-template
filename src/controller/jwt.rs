use crate::controller::JsonMessage;
use crate::database::user::find_user_by_id;
use crate::database::SurrealClient;
use crate::utils::getenv;
use actix_web::{HttpRequest, HttpResponse};
use jsonwebtoken::errors::Error as JwtError;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug)]
struct Claims {
    sub: String,
    iat: usize,
    exp: usize,
}

pub async fn validate_request(
    req: HttpRequest,
    client: Arc<SurrealClient>,
) -> Result<String, HttpResponse> {
    match req.headers().get("Authorization") {
        Some(token) => match token.to_str() {
            Ok(token) => {
                let token = token.replace("Bearer ", "");
                match validate_jwt(token) {
                    Ok(sub) => match find_user_by_id(client.clone(), sub).await {
                        Ok(user) => {
                            if let Some(user) = user {
                                Ok(user.username)
                            } else {
                                Err(HttpResponse::Unauthorized().json(JsonMessage {
                                    msg: "User not found".to_string(),
                                }))
                            }
                        }
                        // SurrealDB error
                        Err(e) => Err(HttpResponse::InternalServerError().json(e)),
                    },
                    // JwtError
                    Err(e) => Err(HttpResponse::Unauthorized().json(JsonMessage {
                        msg: format!("{:?}", e),
                    })),
                }
            }
            Err(_) => Err(HttpResponse::InternalServerError().json(JsonMessage {
                msg: "Error parsing header to string".to_string(),
            })),
        },
        None => Err(HttpResponse::InternalServerError().json(JsonMessage {
            msg: "Authorization field not exist".to_string(),
        })),
    }
}

pub fn sign_jwt(sub: String) -> String {
    let key = getenv("JWT_SECRET_KEY");
    let key = key.as_bytes();
    let now = chrono::Utc::now().timestamp() as usize;

    let claims = Claims {
        sub: sub.to_owned(),
        iat: now,
        // add 5min (300 sec) to now
        exp: now + getenv("JWT_EXPIRE_TIME").parse::<usize>().unwrap(),
    };
    let token = jsonwebtoken::encode(&Header::default(), &claims, &EncodingKey::from_secret(key))
        .expect("Failed to sign token");
    token
}

fn validate_jwt(token: String) -> Result<String, JwtError> {
    let key = getenv("JWT_SECRET_KEY");
    let key = key.as_bytes();
    let validation = Validation::new(Algorithm::HS256);
    // let mut validation = Validation::new(Algorithm::HS256);
    // validation.sub = Some(sub.clone());
    match jsonwebtoken::decode::<Claims>(&token, &DecodingKey::from_secret(key), &validation) {
        Ok(data) => Ok(data.claims.sub),
        Err(err) => Err(err),
    }
}
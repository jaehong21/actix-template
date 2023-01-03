use crate::database::{find_all, find_one, SurrealClient};
use crate::entity::user::User;
use std::sync::Arc;
use surrealdb_rs::method::query_response::QueryResponse;
use surrealdb_rs::Error as SurrealError;

pub async fn find_all_user(client: Arc<SurrealClient>) -> Result<Vec<User>, SurrealError> {
    let response = client
        .query(format!("SELECT * FROM user LIMIT {} START 0", i64::MAX,))
        .await;
    find_all::<User>(response)
}

/// find user by id returns null if not exist
pub async fn find_user_by_id(
    client: Arc<SurrealClient>,
    user_id: String,
) -> Result<Option<User>, SurrealError> {
    let response = client
        .query(format!("SELECT * FROM user WHERE id = '{}'", user_id))
        .await;
    find_one_user(response)
}

/// find user by username returns null if not exist
pub async fn find_user_by_username(
    client: Arc<SurrealClient>,
    username: String,
) -> Result<Option<User>, SurrealError> {
    let response = client
        .query(format!(
            "SELECT * FROM user WHERE username = '{}'",
            username
        ))
        .await;
    find_one_user(response)
}

#[allow(dead_code)]
pub async fn find_user_by_limit(
    client: Arc<SurrealClient>,
    limit: usize,
    start: usize,
) -> Result<Vec<User>, SurrealError> {
    let response = client
        .query(format!(
            "SELECT * FROM user LIMIT {} START {}",
            limit, start,
        ))
        .await;
    find_all::<User>(response)
}

#[allow(dead_code)]
pub async fn create_user(
    client: Arc<SurrealClient>,
    username: String,
    password: String,
) -> Result<User, SurrealError> {
    let user: User = User::new(username, password);
    match client.create("user").content(user).await {
        Ok(user) => Ok(user),
        Err(e) => Err(e),
    }
}

fn find_one_user(
    response: Result<QueryResponse, SurrealError>,
) -> Result<Option<User>, SurrealError> {
    return match find_one::<User>(response) {
        Ok(user) => {
            if user.id != None {
                Ok(Some(user))
            } else {
                Ok(None)
            }
        }
        Err(e) => Err(e),
    };
}

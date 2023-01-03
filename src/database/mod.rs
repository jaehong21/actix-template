pub mod user;

use crate::utils::getenv;
use dotenv::dotenv;
use serde::de::DeserializeOwned;
use std::ops::RangeFull;
use std::sync::Arc;
use surrealdb_rs::{
    method::query_response::QueryResponse, net::WsClient, param::Root, protocol::Ws,
    Error as SurrealError, Surreal,
};

pub type SurrealClient = Surreal<WsClient>;

pub struct SurrealDb {
    pub client: Arc<SurrealClient>,
}

pub async fn connect() -> SurrealClient {
    dotenv().ok();
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

/// select first item from query response
fn find_one<T: DeserializeOwned + Default>(
    response: Result<QueryResponse, SurrealError>,
) -> Result<T, SurrealError> {
    match response {
        Ok(response) => match response.get::<T, usize>(0, 0) {
            Ok(item) => Ok(item),
            // QueryResponse Deserialize Error
            Err(e) => Err(e),
        },
        // Query Error
        Err(e) => Err(e),
    }
}

/// select all item from query response
fn find_all<T: DeserializeOwned + Default>(
    response: Result<QueryResponse, SurrealError>,
) -> Result<Vec<T>, SurrealError> {
    match response {
        Ok(response) => match response.get::<Vec<T>, RangeFull>(0, ..) {
            Ok(items) => Ok(items),
            // QueryResponse Deserialize Error
            Err(e) => Err(e),
        },
        // Query Error
        Err(e) => Err(e),
    }
}

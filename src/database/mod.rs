pub mod user;
use s3::Bucket;
use serde::de::DeserializeOwned;
use std::ops::RangeFull;
use std::sync::Arc;
use surrealdb_rs::{
    method::query_response::QueryResponse, net::WsClient, Error as SurrealError, Surreal,
};

pub type SurrealClient = Surreal<WsClient>;

pub struct Client {
    pub surreal: Arc<SurrealClient>,
    pub s3: Arc<Bucket>,
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

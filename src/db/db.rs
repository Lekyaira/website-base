use std::error::Error;
use leptos::prelude::*;
use leptos::logging;
use async_trait::async_trait;
use crate::models::prelude::*;

#[cfg(feature = "ssr")]
#[async_trait]
pub trait Database: Send {
    async fn get_posts(&self) -> Result<Vec<Post>, Box<dyn Error>>;
    async fn get_post(&self, id: i32) -> Result<Post, Box<dyn Error>>;
    async fn create_post(&self, post: Post) -> Result<i32, Box<dyn Error>>;
    async fn update_post(&self, post: Post) -> Result<(), Box<dyn Error>>;
    async fn delete_post(&self, id: i32) -> Result<(), Box<dyn Error>>;
}

#[cfg(feature = "ssr")]
//static DATABASE: crate::mock::mockdb::MockDb = crate::mock::mockdb::MockDb {};
static DATABASE: crate::seaorm::seapost::SeaormDb = crate::seaorm::seapost::SeaormDb {};

#[cfg(feature = "ssr")]
fn parse_error(error: Box<dyn Error>) -> ServerFnError {
    ServerFnError::ServerError(error.to_string())
}

#[server]
pub async fn get_posts() -> Result<Vec<Post>, ServerFnError> {
    DATABASE.get_posts().await.or_else(|e| Err(parse_error(e)))
}

#[server]
pub async fn get_post(id: i32) -> Result<Post, ServerFnError> {
    DATABASE.get_post(id).await.or_else(|e| Err(parse_error(e)))
}

#[server]
pub async fn create_post(post: Post) -> Result<i32, ServerFnError> {
    DATABASE.create_post(post).await.or_else(|e| Err(parse_error(e)))
}

#[server]
pub async fn update_post(post: Post) -> Result<(), ServerFnError> {
    DATABASE.update_post(post).await.or_else(|e| Err(parse_error(e)))
}

#[server]
pub async fn delete_post(id: i32) -> Result<(), ServerFnError> {
    DATABASE.delete_post(id).await.or_else(|e| Err(parse_error(e)))
}

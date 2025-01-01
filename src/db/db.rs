use std::{
    sync::Mutex,
    fmt,
    error::Error,
};
use leptos::prelude::*;
use crate::models::prelude::*;

#[cfg(feature = "ssr")]
pub static DATABASE_BACKING: Mutex<Option<dyn Database>> = Mutex::new(None);

#[derive(Debug)]
pub enum DbError {
    BackingNotSet,
}
impl Error for DbError {}
impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::BackingNotSet => write!(f, "Database backing it not set"),
        }
    }
}

#[cfg(feature = "ssr")]
pub trait Database {
    pub async fn get_posts() -> Result<Vec<Post>, Box<dyn Error>>;
    pub async fn get_post(id: i32) -> Result<Post, Box<dyn Error>>;
    pub async fn create_post(post: Post) -> Result<i32, Box<dyn Error>>;
    pub async fn update_post(post: Post) -> Result<(), Box<dyn Error>>;
    pub async fn delete_post(id: i32) -> Result<(), Box<dyn Error>>;
}

#[server]
pub async fn get_posts() -> Result<Vec<Post>, ServerFnError> {
    match DATABASE_BACKING.lock().unwrap() {
        Some(db) => {
            match db.get_posts() {
                Ok(posts) => posts,
                Err(e) => Err(ServerFnError::ServerError(e.to_string())),
            }
        }
        None => Err(ServerFnError::ServerError(DbError::BackingNotSet.to_string())),

    } 
}

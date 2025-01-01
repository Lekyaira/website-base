use std::{
    sync::{ Mutex, Once },
    mem::MaybeUninit,
    fmt,
    error::Error,
};
use leptos::prelude::*;
use leptos::logging;
use async_trait::async_trait;
use crate::models::prelude::*;

//#[cfg(feature = "ssr")]
//pub static DATABASE_BACKING: RwLock<Option<Box<dyn Database>>> = RwLock::new(None);

#[cfg(feature = "ssr")]
pub struct DbBackingReader {
    //backing: Mutex<Option<Box<dyn Database + fmt::Display>>>,
    pub backing: Mutex<Option<Box<dyn Database>>>,
}

#[cfg(feature = "ssr")]
pub fn db() -> &'static DbBackingReader {
    // Create an uninitialized static.
    static mut SINGLETON: MaybeUninit<DbBackingReader> = MaybeUninit::uninit();
    static ONCE: Once = Once::new();

    unsafe {
        ONCE.call_once(|| {
            // Initialize singleton.
            let singleton = DbBackingReader {
                backing: Mutex::new(None),
            };
            // Store it to the static singleton var.
            SINGLETON.write(singleton);
        });

        // Give out a shared reference to the data, which is safe to use.
        SINGLETON.assume_init_ref()
    }
}

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
#[async_trait]
pub trait Database: Send {
    async fn get_posts(&self) -> Box<dyn Fn() -> Result<Vec<Post>, Box<dyn Error>>>;
    async fn get_post(&self, id: i32) -> Result<Post, Box<dyn Error>>;
    async fn create_post(&self, post: Post) -> Result<i32, Box<dyn Error>>;
    async fn update_post(&self, post: Post) -> Result<(), Box<dyn Error>>;
    async fn delete_post(&self, id: i32) -> Result<(), Box<dyn Error>>;
}
#[cfg(feature = "ssr")]
impl fmt::Debug for dyn Database {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Database")
    }
}

#[server]
pub async fn get_posts() -> Result<Vec<Post>, ServerFnError> {
    //match DATABASE_BACKING.read().unwrap() {
    let backing = db().backing.lock().unwrap();
    logging::log!("{:?}", backing);
    Ok(vec![])
    //let mut posts;
    //match backing {
    //    Some(db) => posts = db.get_posts(),
    //    None => Err(ServerFnError::ServerError(DbError::BackingNotSet.to_string())),
    //} 
    //posts().await;
    //match posts {
    //    Ok(posts) => Ok(posts),
    //    Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    //}
}


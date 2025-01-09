use std::{ error::Error, env };
use async_trait::async_trait;
use sea_orm::*;
use dotenvy::dotenv;
use crate::db::db as DB;
use crate::models::post::Post as Post_Model;
#[cfg(feature = "ssr")]
use crate::entities::{ prelude::*, * };

const DB_NAME: &'static str = "blog";

// Create a SeaORM connection to the database.
// Can return SeaORM or Dotenvy errors on failure.
#[cfg(feature = "ssr")]
async fn connect() -> Result<DatabaseConnection, Box<dyn Error>> {
    // Get the environment variables for database connection.
    dotenv().ok();
    let server_auth = env::var("DATABASE_SERVER_AUTH")?;
    let db_socket = env::var("DATABASE_SOCKET")?;
    // Create connection string.
    let db_url = format!("mysql://{}/{}?socket={}", server_auth, DB_NAME, db_socket);
    // Create the connection or throw an error.
    let db = Database::connect(db_url).await?;
    // Return good connection object.
    Ok(db)
}

#[cfg(feature = "ssr")]
fn post_to_db(post: Post_Model) -> post::ActiveModel {
    post::ActiveModel {
        id: ActiveValue::Set(post.id),
        title: ActiveValue::Set(post.title),
        body: ActiveValue::Set(post.body),
        published: ActiveValue::Set(post.published.into()),
        ..Default::default()
    }
}

#[cfg(feature = "ssr")]
fn db_to_post(model: post::Model) -> Post_Model {
    Post_Model {
        id: model.id,
        title: model.title,
        body: model.body,
        published: model.published != 0,
    }
}

#[cfg(feature = "ssr")]
pub struct SeaormDb;
#[cfg(feature = "ssr")]
#[async_trait]
impl DB::Database for SeaormDb {
    async fn get_posts(&self) -> Result<Vec<Post_Model>, Box<dyn Error>> {
        // Connect to the MySql database.
        let db = connect().await?;
        // Run the query, returning a `Vec` of `Model`.
        let posts: Vec<post::Model> = Post::find()
            .filter(post::Column::Published.eq(true))
            .all(&db)
            .await?;
        // Convert the result into a `Vec` of `Post`.
        let posts = posts
            .iter()
            .map(|p| db_to_post(p.clone()))
            .collect();
        // Return the result.
        Ok(posts)
    }

    async fn get_post(&self, id: i32) -> Result<Post_Model, Box<dyn Error>> {
        // Connect to the MySql database.
        let db = connect().await?;
        // Run the `find_by_id` query.
        let res = Post::find_by_id(id).one(&db).await?;
        // Convert the result into `Post`.
        let res = match res {
            // Decided to throw rather than return an empty `Vec`.
            None => return Err("No results found.".into()), 
            Some(r) => db_to_post(r),
        };
        // Return the result.
        Ok(res)
    }

    async fn create_post(&self, post: Post_Model) -> Result<i32, Box<dyn Error>> {
        // Connect to the MySql database.
        let db = connect().await?;
        // Convert the `Post` data to Seaorm entity.
        let post = post_to_db(post);
        // Perform the insert, capturing the new id.
        let res = Post::insert(post).exec(&db).await?;
        // Return the id.
        Ok(res.last_insert_id)
    }

    async fn update_post(&self, post: Post_Model) -> Result<(), Box<dyn Error>> {
        // Connect to the MySql database.
        let db = connect().await?;
        // Convert the `Post` into `ActiveModel`.
        let post = post_to_db(post);
        // Run the update query.
        let res = post.update(&db).await?;
        // If all went well, return ok.
        Ok(())
    }

    async fn delete_post(&self, id: i32) -> Result<(), Box<dyn Error>> {
        // Connect to the MySql database.
        let db = connect().await?;
        // Make an `ActiveModel` out of `id`.
        let model = post::ActiveModel {
            id: ActiveValue::Set(id),
            ..Default::default()
        };
        // Run the `delete` query.
        model.delete(&db).await?;
        // If all went well, return ok.
        Ok(())
    }
}

use std::error::Error;
use std::fmt;
use async_trait::async_trait;
use crate::db::prelude::*;
use crate::models::prelude::*;

#[cfg(feature = "ssr")]
#[derive(Debug)]
pub struct MockDb;
#[cfg(feature = "ssr")]
impl MockDb {
    pub fn new() -> Self {
        Self {}
    }
}
#[cfg(feature = "ssr")]
#[async_trait]
impl Database for MockDb {
    async fn get_posts(&self) -> Box<dyn Fn() -> Result<Vec<Post>, Box<dyn Error>>> {
        Box::new(|| {
            let posts = vec![
                Post {
                    id: 1,
                    title: "A Test Post".to_string(),
                    body: "This it a test post...".to_string(),
                    published: true,
                },
                Post {
                    id: 2,
                    title: "Another Test Post".to_string(),
                    body: "...What are you going to do about it??".to_string(),
                    published: true,
                },
                Post {
                    id: 3,
                    title: "And Another!".to_string(),
                    body: "Yes.".to_string(),
                    published: true,
                },
            ];

            Ok(posts)
        })
    }
    //async fn get_posts(&self) -> Result<Vec<Post>, Box<dyn Error>> {
    //    let posts = vec![
    //        Post {
    //            id: 1,
    //            title: "A Test Post".to_string(),
    //            body: "This it a test post...".to_string(),
    //            published: true,
    //        },
    //        Post {
    //            id: 2,
    //            title: "Another Test Post".to_string(),
    //            body: "...What are you going to do about it??".to_string(),
    //            published: true,
    //        },
    //        Post {
    //            id: 3,
    //            title: "And Another!".to_string(),
    //            body: "Yes.".to_string(),
    //            published: true,
    //        },
    //    ];
    //
    //    Ok(posts)
    //}

    async fn get_post(&self, id: i32) -> Result<Post, Box<dyn Error>> {
        let post = Post {
            id: 42,
            title: "A Post".to_string(),
            body: "Yes, this is a post.".to_string(),
            published: true,
        };

        Ok(post)
    }

    async fn create_post(&self, post: Post) -> Result<i32, Box<dyn Error>> {
        Ok(43)
    }

    async fn update_post(&self, post: Post) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    async fn delete_post(&self, id: i32) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

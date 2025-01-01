use crate::db::prelude::*;
use crate::models::prelude::*;

#[cfg(feature = "ssr")]
pub struct MockDb;
#[cfg(feature = "ssr")]
impl MockDb {
    pub fn new() -> Self {}
}
#[cfg(feature = "ssr")]
impl Database for MockDb {
    pub async fn get_posts() -> Result<Vec<Post>, Box<dyn Error>> {
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
    }

    pub async fn get_post(id: i32) -> Result<Post, Box<dyn Error>> {
        let post = Post {
            id: 42,
            title: "A Post".to_string(),
            body: "Yes, this is a post.".to_string(),
            published: true,
        };

        Ok(post)
    }

    pub async fn create_post(post: Post) -> Result<i32, Box<dyn Error>> {
        Ok(43)
    }

    pub async fn update_post(post: Post) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub async fn delete_post(id: i32) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

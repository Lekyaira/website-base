use serde::{Serialize, Deserialize};

#[derive(Clone, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}


use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct BlogCard {
    pub id: i32,
    pub posted_by: String,
    pub title: String,
    pub summary: String,
}

#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct BlogPost {
    pub id: usize,
    pub posted_by: String,
    pub title: String,
    pub content: String,
}


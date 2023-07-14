use serde::Deserialize;

use crate::models::article::Status;

#[derive(Debug, Clone, Deserialize)]
pub struct CreateArticle {
    pub title: String,
    pub image: Option<i32>,
    pub content: String,
    pub status: Status,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Image(String);

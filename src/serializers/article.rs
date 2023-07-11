use serde::{Deserialize, Serialize};

use crate::models::article::Status;

#[derive(Debug, Deserialize)]
pub struct CreateArticle {
    pub title: String,
    pub slug: String,
    pub image: Option<String>,
    pub content: String,
    pub status: Status,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCategory {
    pub title: String,
    pub description: Option<String>,
}

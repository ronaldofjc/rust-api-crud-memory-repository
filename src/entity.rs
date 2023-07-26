use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BookRequest {
    pub title: Option<String>,
    pub author: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Book {
    pub id: Uuid,
    pub title: String,
    pub author: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorData {
    pub message: String
}

impl ErrorData {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}
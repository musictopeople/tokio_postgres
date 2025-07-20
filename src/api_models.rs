use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct PostRequest {
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Serialize, Clone)]
pub struct Response {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct PostRequest {
    pub(crate) title: String,
    pub(crate) body: String,
    pub(crate) published: bool,
}

#[derive(Serialize)]
pub struct Response {
    pub(crate) id: i32,
    pub(crate) title: String,
    pub(crate) body: String,
    pub(crate) published: bool,
}

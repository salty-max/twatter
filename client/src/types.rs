use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct DtoPostForm {
    pub text: String,
}

#[derive(Deserialize)]
pub struct DtoCreatePostSuccess {
    pub id: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct DtoPost {
    pub id: i32,
    pub text: String,
    pub likes: i32,
    pub reply_count: i32,
}

use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct FrontMatter {
    pub title: String,
    pub tags: Vec<String>,
}

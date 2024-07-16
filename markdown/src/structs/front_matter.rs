use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, PartialEq)]
pub struct FrontMatter {
    pub title: String,
    pub tags: Vec<String>,
}

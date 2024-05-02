use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct FrontMatter {
    title: String,
    tags: Vec<String>,
}

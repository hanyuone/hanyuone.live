use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, PartialEq)]
pub struct FrontMatter {
    pub title: String,
    pub tags: Vec<String>,
}

#[derive(Clone, Deserialize, Serialize, PartialEq)]
pub struct PostRenderData {
    pub read_time: Duration,
}

#[derive(Clone, Deserialize, Serialize, PartialEq)]
pub struct BlogMetadata {
    pub front_matter: FrontMatter,
    pub post_render: PostRenderData,
}

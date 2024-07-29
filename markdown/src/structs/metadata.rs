use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RawFrontMatter {
    pub title: String,
    pub description: String,
    pub image: String,
    pub publish_date: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FrontMatter {
    pub title: String,
    pub description: String,
    pub image: String,
    pub publish_date: NaiveDateTime,
    pub tags: Vec<String>,
}

impl From<RawFrontMatter> for FrontMatter {
    fn from(value: RawFrontMatter) -> Self {
        let parsed = NaiveDateTime::parse_from_str(&value.publish_date, "%Y-%m-%d %H:%M").unwrap();

        Self {
            title: value.title,
            description: value.description,
            image: value.image,
            publish_date: parsed,
            tags: value.tags,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostTranslateData {
    pub read_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BlogMetadata {
    pub front_matter: FrontMatter,
    pub post_translate: PostTranslateData,
}

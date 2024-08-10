// use chrono::NaiveDateTime;
use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Archive, Serialize, Deserialize, serde::Deserialize)]
#[archive(check_bytes)]
pub struct FrontMatter {
    pub title: String,
    pub description: String,
    pub image: String,
    pub publish_date: String,
    pub tags: Vec<String>,
}

/*
#[derive(Debug, Clone, Archive, Serialize, Deserialize, PartialEq)]
#[archive(check_bytes)]
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
*/

#[derive(Debug, Clone, Archive, Serialize, Deserialize, PartialEq)]
#[archive(check_bytes)]
pub struct PostTranslateData {
    pub words: usize,
}

#[derive(Debug, Clone, Archive, Serialize, Deserialize, PartialEq)]
#[archive(check_bytes)]
pub struct BlogMetadata {
    pub front_matter: FrontMatter,
    pub post_translate: PostTranslateData,
}

use chrono::{NaiveDateTime, TimeDelta};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DurationSeconds};

#[derive(Deserialize)]
pub struct RawFrontMatter {
    pub title: String,
    pub publish_date: String,
    pub tags: Vec<String>,
}

#[derive(Clone, Deserialize, Serialize, PartialEq)]
pub struct FrontMatter {
    pub title: String,
    pub publish_date: NaiveDateTime,
    pub tags: Vec<String>,
}

impl From<RawFrontMatter> for FrontMatter {
    fn from(value: RawFrontMatter) -> Self {
        let parsed = NaiveDateTime::parse_from_str(&value.publish_date, "%Y-%m-%d %H:%M").unwrap();

        Self {
            title: value.title,
            publish_date: parsed,
            tags: value.tags,
        }
    }
}

#[serde_as]
#[derive(Clone, Deserialize, Serialize, PartialEq)]
pub struct PostRenderData {
    #[serde_as(as = "DurationSeconds<i64>")]
    pub read_time: TimeDelta,
}

#[derive(Clone, Deserialize, Serialize, PartialEq)]
pub struct BlogMetadata {
    pub front_matter: FrontMatter,
    pub post_render: PostRenderData,
}

use macros::generate_blog_id;

use enum_iterator;
use serde;

use crate::front_matter::FrontMatter;

generate_blog_id!("content/blog");

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct BlogCard {
    pub id: BlogId,
    pub front_matter: FrontMatter,
}

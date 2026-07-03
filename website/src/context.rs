use std::collections::HashMap;

use markdown::structs::{blog::BlogId, metadata::BlogMetadata, tag::TagId};

const POSTS_IN_PAGE: usize = 10;

#[derive(Clone)]
pub struct BlogContext {
    pub content: HashMap<BlogId, BlogMetadata>,
}

impl PartialEq for BlogContext {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl BlogContext {
    pub fn new(bytes: &str) -> Self {
        let content = ron::from_str::<HashMap<BlogId, BlogMetadata>>(bytes).unwrap();
        Self { content }
    }

    pub fn get(&self, id: &BlogId) -> Option<&BlogMetadata> {
        self.content.get(id)
    }

    pub fn get_all(&self) -> Vec<Vec<(BlogId, BlogMetadata)>> {
        let mut blogs = self.content.iter().collect::<Vec<_>>();

        blogs.sort_by(|(_, a), (_, b)| {
            b.front_matter
                .publish_date
                .cmp(&a.front_matter.publish_date)
        });

        blogs
            .chunks(POSTS_IN_PAGE)
            .map(|page| {
                page.iter()
                    .map(|(id, metadata)| (**id, (*metadata).clone()))
                    .collect()
            })
            .collect()
    }

    pub fn get_with_tag(&self, tag: TagId) -> Vec<Vec<(BlogId, BlogMetadata)>> {
        let mut blogs = self
            .content
            .iter()
            .filter(|(_, metadata)| metadata.front_matter.tags.contains(&tag.to_string()))
            .collect::<Vec<_>>();

        blogs.sort_by(|(_, a), (_, b)| {
            b.front_matter
                .publish_date
                .cmp(&a.front_matter.publish_date)
        });

        blogs
            .chunks(POSTS_IN_PAGE)
            .map(|page| {
                page.iter()
                    .map(|(id, metadata)| (**id, (*metadata).clone()))
                    .collect()
            })
            .collect()
    }
}

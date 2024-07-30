use std::{cell::RefCell, rc::Rc};

use markdown::structs::{blog::BlogId, metadata::BlogMetadata};
use rkyv::Deserialize;
use yew::Html;

#[derive(Default)]
pub struct HeadContext {
    pub content: Rc<RefCell<Html>>,
}

impl PartialEq for HeadContext {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl Clone for HeadContext {
    fn clone(&self) -> Self {
        Self {
            content: self.content.clone(),
        }
    }
}

#[derive(Clone)]
pub struct BlogContext {
    pub content: Vec<(BlogId, BlogMetadata)>,
}

impl PartialEq for BlogContext {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl BlogContext {
    pub fn new(bytes: &[u8]) -> Self {
        let archived = rkyv::check_archived_root::<Vec<(BlogId, BlogMetadata)>>(bytes).expect("Archived properly");
        let content = archived.deserialize(&mut rkyv::Infallible).unwrap();

        Self {
            content,
        }
    }

    pub fn get(&self, id: &BlogId) -> Option<&BlogMetadata> {
        self.content.iter()
            .find(|(target_id, _)| id == target_id)
            .map(|(_, metadata)| metadata)
    }
}

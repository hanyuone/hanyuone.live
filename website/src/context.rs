use std::{cell::RefCell, collections::HashMap, rc::Rc};

use markdown::structs::{blog::BlogId, metadata::BlogMetadata};
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
    pub content: HashMap<BlogId, BlogMetadata>,
}

impl PartialEq for BlogContext {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl BlogContext {
    pub fn new(bytes: &str) -> Self {
        let content = ron::from_str::<HashMap<BlogId, BlogMetadata>>(bytes);
        Self { content: content.unwrap() }
    }

    pub fn get(&self, id: &BlogId) -> Option<&BlogMetadata> {
        self.content.get(id)
    }
}

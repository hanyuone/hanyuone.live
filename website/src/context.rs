use std::{cell::RefCell, collections::HashMap, rc::Rc};

use markdown::structs::{blog::BlogId, metadata::BlogMetadata};
use yew::Html;

// FIXME: split BlogContext and TagContext into separate provide-context components, see
// https://github.com/BlakeRain/blakerain.com/blob/main/src/model.rs

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
        let content = ron::from_str::<HashMap<BlogId, BlogMetadata>>(bytes).unwrap();
        Self { content }
    }

    pub fn get(&self, id: &BlogId) -> Option<&BlogMetadata> {
        self.content.get(id)
    }
}

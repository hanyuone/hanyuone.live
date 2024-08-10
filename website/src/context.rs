use std::{cell::RefCell, collections::HashMap, rc::Rc};

use markdown::structs::{blog::BlogId, metadata::BlogMetadata};
use serde::Deserialize;
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
    pub fn new(bytes: &[u8]) -> Self {
        let content = rkyv::from_bytes::<Vec<(BlogId, BlogMetadata)>>(bytes)
            .unwrap()
            .into_iter()
            .collect::<HashMap<BlogId, BlogMetadata>>();

        Self { content }
    }

    pub fn get(&self, id: &BlogId) -> Option<&BlogMetadata> {
        self.content.get(id)
    }
}

#[derive(Clone, Deserialize)]
struct YamlTag {
    name: String,
    colour: String,
}

#[derive(Clone)]
pub struct TagContext {
    pub content: HashMap<String, String>,
}

impl PartialEq for TagContext {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl TagContext {
    pub fn new(yaml: &str) -> Self {
        let content = serde_yml::from_str::<Vec<YamlTag>>(yaml)
            .unwrap()
            .into_iter()
            .map(|yaml_tag| (yaml_tag.name, yaml_tag.colour))
            .collect::<HashMap<_, _>>();

        Self { content }
    }

    pub fn get(&self, tag: &str) -> Option<String> {
        self.content.get(tag).cloned()
    }
}

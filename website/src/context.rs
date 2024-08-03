use std::{cell::RefCell, collections::HashMap, rc::Rc};

use markdown::structs::{blog::BlogId, metadata::BlogMetadata};
use serde::Deserialize;
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
        let content = ron::from_str::<HashMap<BlogId, BlogMetadata>>(bytes).unwrap();
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

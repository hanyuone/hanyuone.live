use std::{cell::RefCell, rc::Rc};

use markdown::structs::blog::BlogCard;
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
    pub content: Vec<BlogCard>,
}

impl PartialEq for BlogContext {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl BlogContext {
    pub fn new(bytes: &[u8]) -> Self {
        let content = postcard::from_bytes::<Vec<BlogCard>>(bytes).unwrap();
        Self { content }
    }
}

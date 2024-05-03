use std::{cell::RefCell, rc::Rc};

use gloo_net::http::Request;
use markdown::blog::BlogCard;
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
    pub async fn new() -> Self {
        let raw_content = Request::get("/public/blog/blog_cards")
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let content = postcard::from_bytes::<Vec<BlogCard>>(raw_content.as_bytes()).unwrap();

        Self { content }
    }
}

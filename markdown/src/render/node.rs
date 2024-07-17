use std::fmt::Display;

use pulldown_cmark::HeadingLevel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum RenderNode {
    Text(String),
    Element(RenderElement),
}

#[derive(Serialize, Deserialize, PartialEq)]
pub enum RenderTag {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    P,
}

impl Display for RenderTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let as_str = match *self {
            RenderTag::H1 => "h1",
            RenderTag::H2 => "h2",
            RenderTag::H3 => "h3",
            RenderTag::H4 => "h4",
            RenderTag::H5 => "h5",
            RenderTag::H6 => "h6",
            RenderTag::P => "p",
        };

        write!(f, "{}", as_str)
    }
}

impl From<HeadingLevel> for RenderTag {
    fn from(value: HeadingLevel) -> Self {
        match value {
            HeadingLevel::H1 => RenderTag::H1,
            HeadingLevel::H2 => RenderTag::H2,
            HeadingLevel::H3 => RenderTag::H3,
            HeadingLevel::H4 => RenderTag::H4,
            HeadingLevel::H5 => RenderTag::H5,
            HeadingLevel::H6 => RenderTag::H6,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum AttributeName {
    Class,
    Id,
}

impl From<AttributeName> for &'static str {
    fn from(value: AttributeName) -> Self {
        match value {
            AttributeName::Class => "class",
            AttributeName::Id => "id",
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Attribute {
    pub key: AttributeName,
    pub value: String,
}

#[derive(Serialize, Deserialize)]
pub struct RenderElement {
    pub tag: RenderTag,
    pub attributes: Vec<Attribute>,
    pub children: Vec<RenderNode>,
}

impl RenderElement {
    pub fn new(tag: RenderTag) -> Self {
        Self {
            tag,
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn add_attribute(&mut self, key: AttributeName, value: String) {
        self.attributes.push(Attribute { key, value })
    }

    pub fn add_child(&mut self, child: RenderNode) {
        self.children.push(child)
    }
}

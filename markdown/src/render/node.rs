use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum RenderNode {
    Text(String),
    Element(RenderElement),
}

#[derive(Serialize, Deserialize, PartialEq)]
pub enum RenderTag {
    P,
}

impl Display for RenderTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            RenderTag::P => write!(f, "p"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum AttributeName {
    Class,
    Id,
}

impl AttributeName {
    pub fn as_str(&self) -> &'static str {
        match self {
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
}

use std::fmt::Display;

use pulldown_cmark::HeadingLevel;
use serde::{Deserialize, Serialize};

use super::node::RenderNode;

#[derive(Serialize, Deserialize, PartialEq)]
pub enum ElementTag {
    // HTML tags
    Div,
    Em,
    FigCaption,
    Figure,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Img,
    P,
    Strong,
}

impl Display for ElementTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let as_str = match *self {
            // HTML tags
            Self::Div => "div",
            Self::Em => "em",
            Self::FigCaption => "figcaption",
            Self::Figure => "figure",
            Self::H1 => "h1",
            Self::H2 => "h2",
            Self::H3 => "h3",
            Self::H4 => "h4",
            Self::H5 => "h5",
            Self::H6 => "h6",
            Self::Img => "img",
            Self::P => "p",
            Self::Strong => "strong",
        };

        write!(f, "{}", as_str)
    }
}

impl From<HeadingLevel> for ElementTag {
    fn from(value: HeadingLevel) -> Self {
        match value {
            HeadingLevel::H1 => ElementTag::H1,
            HeadingLevel::H2 => ElementTag::H2,
            HeadingLevel::H3 => ElementTag::H3,
            HeadingLevel::H4 => ElementTag::H4,
            HeadingLevel::H5 => ElementTag::H5,
            HeadingLevel::H6 => ElementTag::H6,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum AttributeName {
    Alt,
    Class,
    Id,
    Src,
    Title,
}

impl From<AttributeName> for &'static str {
    fn from(value: AttributeName) -> Self {
        match value {
            AttributeName::Alt => "alt",
            AttributeName::Class => "class",
            AttributeName::Id => "id",
            AttributeName::Src => "src",
            AttributeName::Title => "title",
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
    pub tag: ElementTag,
    pub attributes: Vec<Attribute>,
    pub children: Vec<RenderNode>,
}

impl RenderElement {
    pub fn new(tag: ElementTag) -> Self {
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

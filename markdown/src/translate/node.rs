use std::fmt::Display;

use pulldown_cmark::HeadingLevel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum RenderIcon {
    Note,
    Tip,
    Important,
    Warning,
    Caution,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub enum RenderTag {
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
    // AST-specific tags
    Callout,
}

impl Display for RenderTag {
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
            // AST-specific tags (for debugging purposes)
            Self::Callout => "!callout",
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

#[derive(Serialize, Deserialize)]
pub enum RenderNode {
    Text(String),
    Icon(RenderIcon),
    Element(RenderElement),
}

impl From<String> for RenderNode {
    fn from(value: String) -> Self {
        RenderNode::Text(value)
    }
}

impl From<RenderIcon> for RenderNode {
    fn from(value: RenderIcon) -> Self {
        RenderNode::Icon(value)
    }
}

impl From<RenderElement> for RenderNode {
    fn from(value: RenderElement) -> Self {
        RenderNode::Element(value)
    }
}

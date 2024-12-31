use std::fmt::Display;

use pulldown_cmark::HeadingLevel;
use serde::{Deserialize, Serialize};

use super::node::RenderNode;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ElementTag {
    A,
    BlockQuote,
    Br,
    Code,
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
    Hr,
    Img,
    Li,
    Ol,
    P,
    Strong,
    Sup,
    Table,
    Tbody,
    Td,
    Th,
    Thead,
    Tr,
    Ul,
}

impl Display for ElementTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let as_str = match *self {
            Self::A => "a",
            Self::BlockQuote => "blockquote",
            Self::Br => "br",
            Self::Code => "code",
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
            Self::Hr => "hr",
            Self::Img => "img",
            Self::Li => "li",
            Self::Ol => "ol",
            Self::P => "p",
            Self::Strong => "strong",
            Self::Sup => "sup",
            Self::Table => "table",
            Self::Tbody => "tbody",
            Self::Td => "td",
            Self::Th => "th",
            Self::Thead => "thead",
            Self::Tr => "tr",
            Self::Ul => "ul",
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

#[derive(Debug, Serialize, Deserialize)]
pub enum AttributeName {
    Align,
    Alt,
    Class,
    Colspan,
    Href,
    Id,
    Rowspan,
    Src,
    Start,
    Title,
}

impl From<AttributeName> for &'static str {
    fn from(value: AttributeName) -> Self {
        match value {
            AttributeName::Align => "align",
            AttributeName::Alt => "alt",
            AttributeName::Class => "class",
            AttributeName::Colspan => "colspan",
            AttributeName::Href => "href",
            AttributeName::Id => "id",
            AttributeName::Rowspan => "rowspan",
            AttributeName::Src => "src",
            AttributeName::Start => "start",
            AttributeName::Title => "title",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attribute {
    pub key: AttributeName,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
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

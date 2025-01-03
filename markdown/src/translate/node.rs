use std::fmt::Display;

use pulldown_cmark::BlockQuoteKind;
use serde::{Deserialize, Serialize};

use super::element::{ElementTag, RenderElement};

// HTML

#[derive(Debug, Serialize, Deserialize)]
pub struct RenderHtml(pub String);

// ICONS

#[derive(Debug, Serialize, Deserialize)]
pub enum RenderIcon {
    Note,
    Tip,
    Important,
    Warning,
    Caution,
}

// CALLOUTS

#[derive(Debug, Serialize, Deserialize)]
pub enum CalloutKind {
    Note,
    Tip,
    Important,
    Warning,
    Caution,
}

impl From<BlockQuoteKind> for CalloutKind {
    fn from(value: BlockQuoteKind) -> Self {
        match value {
            BlockQuoteKind::Note => CalloutKind::Note,
            BlockQuoteKind::Tip => CalloutKind::Tip,
            BlockQuoteKind::Important => CalloutKind::Important,
            BlockQuoteKind::Warning => CalloutKind::Warning,
            BlockQuoteKind::Caution => CalloutKind::Caution,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RenderCallout {
    pub kind: CalloutKind,
    pub children: Vec<RenderNode>,
}

impl RenderCallout {
    pub fn new(kind: CalloutKind) -> Self {
        Self {
            kind,
            children: vec![],
        }
    }

    pub fn add_child(&mut self, child: RenderNode) {
        self.children.push(child)
    }
}

// OVERARCHING NODE DATA TYPE

#[derive(Debug, Serialize, Deserialize)]
pub enum RenderNode {
    Text(String),
    Html(RenderHtml),
    Icon(RenderIcon),
    Element(RenderElement),
    // Separate member because we want them to appear outside of <article>
    Callout(RenderCallout),
}

impl From<String> for RenderNode {
    fn from(value: String) -> Self {
        RenderNode::Text(value)
    }
}

impl From<RenderHtml> for RenderNode {
    fn from(value: RenderHtml) -> Self {
        RenderNode::Html(value)
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

impl From<RenderCallout> for RenderNode {
    fn from(value: RenderCallout) -> Self {
        RenderNode::Callout(value)
    }
}

// OVERARCHING TAG DATA TYPE

#[derive(Debug, Clone)]
pub enum RenderTag {
    Element(ElementTag),
    Callout,
    Html,
}

impl Display for RenderTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted = match self {
            RenderTag::Element(tag) => &tag.to_string(),
            RenderTag::Callout => "callout",
            RenderTag::Html => "html",
        };

        write!(f, "{}", formatted)
    }
}

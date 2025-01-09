use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::{
    container::callout::Callout,
    element::{ElementTag, RenderElement},
};

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

// OVERARCHING NODE DATA TYPE

#[derive(Debug, Serialize, Deserialize)]
pub enum RenderNode {
    Text(String),
    Html(RenderHtml),
    Icon(RenderIcon),
    Element(RenderElement),
    // Separate member because we want them to appear outside of <article>
    Callout(Callout),
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

impl From<Callout> for RenderNode {
    fn from(value: Callout) -> Self {
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

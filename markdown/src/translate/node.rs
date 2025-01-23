use serde::{Deserialize, Serialize};

use super::{complex::code_block::CodeBlock, container::callout::Callout, element::RenderElement};

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
    Element(RenderElement),
    // Unique node types, but within <article>
    Text(String),
    Html(RenderHtml),
    Icon(RenderIcon),
    CodeBlock(CodeBlock),
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

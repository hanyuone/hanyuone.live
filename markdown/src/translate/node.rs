use serde::{Deserialize, Serialize};

use super::element::{ElementTag, RenderElement};

#[derive(Serialize, Deserialize)]
pub enum RenderIcon {
    Note,
    Tip,
    Important,
    Warning,
    Caution,
}

#[derive(Serialize, Deserialize)]
pub enum CalloutKind {
    Note,
    Tip,
    Important,
    Warning,
    Caution,
}

#[derive(Serialize, Deserialize)]
pub struct RenderCallout {
    pub kind: CalloutKind,
    pub children: Vec<RenderNode>,
}

#[derive(Serialize, Deserialize)]
pub enum RenderNode {
    Text(String),
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

enum RenderTag {
    Element(ElementTag),
    Callout,
}

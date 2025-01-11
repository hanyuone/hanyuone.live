use std::fmt::Display;

use callout::Callout;

use super::{element::{ElementTag, RenderElement}, node::RenderNode};

pub mod callout;

pub enum Container {
    Element(RenderElement),
    Callout(Callout),
}

impl Container {
    pub fn add_child(&mut self, child: RenderNode) {
        match self {
            Container::Element(element) => element.add_child(child),
            Container::Callout(callout) => callout.add_child(child),
        }
    }
}

impl From<Container> for RenderNode {
    fn from(value: Container) -> Self {
        match value {
            Container::Element(element) => element.into(),
            Container::Callout(callout) => callout.into(),
        }
    }
}

// OVERARCHING TAG DATA TYPE

#[derive(Debug, Clone)]
pub enum ContainerTag {
    Element(ElementTag),
    Callout,
}

impl Display for ContainerTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted = match self {
            ContainerTag::Element(tag) => &tag.to_string(),
            ContainerTag::Callout => "callout",
        };

        write!(f, "{}", formatted)
    }
}

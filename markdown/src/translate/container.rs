use std::fmt::Display;

use callout::Callout;
use table::Table;

use super::{
    element::{ElementTag, RenderElement},
    node::RenderNode,
};

pub mod callout;
pub mod table;

pub enum Container {
    Element(RenderElement),
    Callout(Callout),
    Table(Table),
}

impl Container {
    pub fn add_child(&mut self, child: RenderNode) {
        match self {
            Container::Element(element) => element.add_child(child),
            Container::Callout(callout) => callout.add_child(child),
            Container::Table(table) => table.add_child(child),
        }
    }
}

impl From<Container> for RenderNode {
    fn from(value: Container) -> Self {
        match value {
            Container::Element(element) => element.into(),
            Container::Callout(callout) => callout.into(),
            Container::Table(table) => table.into(),
        }
    }
}

// OVERARCHING TAG DATA TYPE

#[derive(Debug, Clone)]
pub enum ContainerTag {
    Element(ElementTag),
    Callout,
    Table,
}

impl Display for ContainerTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted = match self {
            ContainerTag::Element(tag) => &tag.to_string(),
            ContainerTag::Callout => "callout",
            ContainerTag::Table => "table",
        };

        write!(f, "{}", formatted)
    }
}

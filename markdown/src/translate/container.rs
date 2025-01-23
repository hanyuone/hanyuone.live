//! All components that "contain" other components inside it.

use std::fmt::Display;

use callout::Callout;
use code_block::CodeBlock;
use table::Table;

use super::{
    element::{ElementTag, RenderElement},
    node::RenderNode,
};

pub mod callout;
pub mod code_block;
pub mod table;

/// Overarching data structure for all containers. We need to use an
/// enum here because `Translator` stores a list of containers in a stack,
/// so we need to know the size of our values at compile time.
/// 
/// A trait was considered for containers, but we also need to be able to
/// convert `Container`s into `RenderNode`s by taking `self`, which Rust
/// doesn't allow for memory safety reasons.
pub enum Container {
    Element(RenderElement),
    Callout(Callout),
    CodeBlock(CodeBlock),
    Table(Table),
}

impl Container {
    pub fn add_child(&mut self, child: RenderNode) {
        match self {
            Container::Element(element) => element.add_child(child),
            Container::Callout(callout) => callout.add_child(child),
            Container::CodeBlock(code_block) => code_block.add_child(child),
            Container::Table(table) => table.add_child(child),
        }
    }
}

impl From<Container> for RenderNode {
    fn from(value: Container) -> Self {
        match value {
            Container::Element(element) => element.into(),
            Container::Callout(callout) => callout.into(),
            Container::CodeBlock(code_block) => code_block.into(),
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

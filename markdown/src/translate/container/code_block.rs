use serde::{Deserialize, Serialize};

use crate::translate::node::RenderNode;

use super::Container;

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeBlock {
    language: Option<String>,
    contents: String,
}

impl CodeBlock {
    pub fn new(language: Option<String>) -> Self {
        Self {
            language,
            contents: String::new(),
        }
    }

    pub fn add_child(&mut self, child: RenderNode) {
        let RenderNode::Text(contents) = child else {
            unreachable!();
        };

        self.contents = contents;
    }
}

impl From<CodeBlock> for Container {
    fn from(value: CodeBlock) -> Self {
        todo!()
    }
}

impl From<CodeBlock> for RenderNode {
    fn from(value: CodeBlock) -> Self {
        todo!()
    }
}

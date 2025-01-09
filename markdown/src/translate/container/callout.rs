use pulldown_cmark::BlockQuoteKind;
use serde::{Deserialize, Serialize};

use crate::translate::node::RenderNode;

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
pub struct Callout {
    pub kind: CalloutKind,
    pub children: Vec<RenderNode>,
}

impl Callout {
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

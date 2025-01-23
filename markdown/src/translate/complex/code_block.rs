use serde::{Deserialize, Serialize};

use crate::translate::{error::TranslateError, node::RenderNode};

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

    pub fn add_child(&mut self, child: RenderNode) -> Result<(), TranslateError> {
        let RenderNode::Text(contents) = child else {
            return Err(TranslateError::RawHtmlError);
        };

        self.contents = contents;
        Ok(())
    }
}

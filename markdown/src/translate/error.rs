use std::fmt::Display;

use super::{element::ElementTag, node::RenderTag};

#[derive(Debug, Clone)]
pub enum TranslateError {
    ElementError {
        expected: ElementTag,
        result: Option<ElementTag>,
    },
    NoMatchError {
        tags: Vec<RenderTag>,
    },
    CalloutError,
    RawHtmlError,
    TableMergeError,
}

impl Display for TranslateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_str = match self {
            Self::ElementError { expected, result } => match result {
                Some(result) => format!("Expected <{}>, got <{}>", expected, result),
                None => format!("Expected <{}>", expected),
            },
            Self::NoMatchError { tags } => {
                let tags_str = tags
                    .iter()
                    .map(|tag| tag.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");

                format!("None of the tags {} matched", tags_str)
            }
            Self::CalloutError => "Expected callout".to_string(),
            Self::RawHtmlError => "Expected raw HTML".to_string(),
            Self::TableMergeError => "Invalid merge command".to_string(),
        };

        write!(f, "Markdown translation error: {}", error_str)
    }
}

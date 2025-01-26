use std::fmt::Display;

use super::container::ContainerTag;

#[derive(Debug)]
pub enum TranslateError {
    NoMatchError {
        expected: ContainerTag,
        result: ContainerTag,
    },
    NoMatchAnyError {
        expected: Vec<ContainerTag>,
        result: ContainerTag,
    },
    CalloutError,
    FootnoteError {
        name: String,
    },
    TableMergeError,
}

impl Display for TranslateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_str = match self {
            Self::NoMatchError { expected, result } => {
                format!("Expected <{}>, got <{}>", expected, result)
            }
            Self::NoMatchAnyError { expected, result } => {
                let tags_str = expected
                    .iter()
                    .map(|tag| format!("<{}>", tag))
                    .collect::<Vec<_>>()
                    .join(", ");

                format!("Expected any of [{}], got {}", tags_str, result)
            }
            Self::CalloutError => "Expected callout".to_string(),
            Self::FootnoteError { name } => format!("Footnote \"{}\" does not exist", name),
            Self::TableMergeError => "Invalid merge command".to_string(),
        };

        write!(f, "Markdown translation error: {}", error_str)
    }
}

pub mod complex;
pub mod container;
pub mod element;
pub mod error;
pub mod node;
pub mod translator;

use error::TranslateError;
use pulldown_cmark::{Options, Parser};
use translator::{TranslateOutput, Translator};

use crate::structs::metadata::PostTranslateData;

pub struct TranslateOutputBytes {
    pub bytes: String,
    pub post_translate: PostTranslateData,
}

pub fn to_bytestring(raw: &str) -> Result<TranslateOutputBytes, TranslateError> {
    let parser = Parser::new_ext(raw, Options::all());
    let translator = Translator::new(parser);
    let TranslateOutput {
        nodes,
        post_translate,
    } = translator.run()?;

    Ok(TranslateOutputBytes {
        bytes: ron::to_string(&nodes).expect("encoded nodes into bytestring"),
        post_translate,
    })
}

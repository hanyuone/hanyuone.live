pub mod complex;
pub mod element;
pub mod error;
pub mod node;
pub mod translator;

use pulldown_cmark::{Options, Parser};
use translator::{TranslateOutput, Translator};

use crate::structs::metadata::PostTranslateData;

pub struct TranslateOutputBytes {
    pub bytes: String,
    pub post_translate: PostTranslateData,
}

pub fn to_bytestring(raw: &str) -> TranslateOutputBytes {
    let parser = Parser::new_ext(raw, Options::all());
    let translator = Translator::new(parser);
    let TranslateOutput {
        nodes,
        post_translate,
    } = translator.run();

    TranslateOutputBytes {
        bytes: ron::to_string(&nodes).expect("encoded nodes into bytestring"),
        post_translate,
    }
}

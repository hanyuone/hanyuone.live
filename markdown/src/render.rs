pub mod node;
pub mod renderer;

use std::time::Duration;

use pulldown_cmark::{Options, Parser};
use renderer::Renderer;

pub struct RenderInfo {
    pub bytes: Vec<u8>,
    pub read_time: Duration,
}

pub fn to_bytestring(raw: &str) -> RenderInfo {
    let parser = Parser::new_ext(raw, Options::all());
    let renderer = Renderer::new(parser);
    let nodes = renderer.run();

    RenderInfo {
        bytes: postcard::to_stdvec(&nodes).expect("encoded nodes into bytestring"),
        read_time: Duration::ZERO,
    }
}

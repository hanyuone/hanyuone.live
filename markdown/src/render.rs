pub mod node;
pub mod renderer;

use pulldown_cmark::{Options, Parser};
use renderer::{RenderOutput, Renderer};

use crate::structs::metadata::PostRenderData;

pub struct RenderOutputBytes {
    pub bytes: Vec<u8>,
    pub post_render: PostRenderData,
}

pub fn to_bytestring(raw: &str) -> RenderOutputBytes {
    let parser = Parser::new_ext(raw, Options::all());
    let renderer = Renderer::new(parser);
    let RenderOutput { nodes, post_render } = renderer.run();

    RenderOutputBytes {
        bytes: postcard::to_stdvec(&nodes).expect("encoded nodes into bytestring"),
        post_render,
    }
}

use std::marker::PhantomData;

use chrono::TimeDelta;
use pulldown_cmark::{Event, Tag, TagEnd};

use crate::structs::metadata::PostRenderData;

use super::node::{AttributeName, RenderElement, RenderNode, RenderTag};

pub struct RenderOutput {
    pub nodes: Vec<RenderNode>,
    pub post_render: PostRenderData,
}

/// Helper struct used for converting Markdown events (generated via `pulldown_cmark`)
/// into a simplified virtual DOM that can easily be converted to work with Yew.
pub struct Renderer<'a, I> {
    tokens: I,
    output: Vec<RenderNode>,
    stack: Vec<RenderElement>,
    post_render: PostRenderData,
    phantom: PhantomData<&'a I>,
}

impl<'a, I> Renderer<'a, I>
where
    I: Iterator<Item = Event<'a>>,
{
    pub fn new(tokens: I) -> Self {
        Self {
            tokens,
            output: vec![],
            stack: vec![],
            post_render: PostRenderData {
                read_time: TimeDelta::zero(),
            },
            phantom: PhantomData,
        }
    }

    fn output(&mut self, node: RenderNode) {
        if let Some(top) = self.stack.last_mut() {
            top.add_child(node);
        } else {
            self.output.push(node);
        }
    }

    fn enter(&mut self, element: RenderElement) {
        self.stack.push(element);
    }

    fn leave(&mut self, tag: RenderTag) {
        let Some(top) = self.stack.pop() else {
            panic!("Stack underflow");
        };

        assert!(
            top.tag == tag,
            "Expected to pop <{}>, found <{}>",
            tag,
            top.tag
        );

        self.output(RenderNode::Element(top))
    }

    /// Consumes the next HTML element in our Markdown text and returns it as a single-lined string.
    fn capture_next_as_text(&mut self) -> String {
        let mut nest_level = 0;
        let mut captured = String::new();

        for event in self.tokens.by_ref() {
            match event {
                Event::Start(_) => nest_level += 1,
                Event::End(_) => {
                    nest_level -= 1;

                    if nest_level == 0 {
                        break;
                    }
                }
                Event::Html(text) | Event::Code(text) | Event::Text(text) => {
                    captured.push_str(&text)
                }
                Event::SoftBreak | Event::HardBreak | Event::Rule => captured.push(' '),
                _ => todo!(),
            }
        }

        captured
    }

    fn run_start(&mut self, tag: Tag) {
        match tag {
            // Text styles
            Tag::Paragraph => self.enter(RenderElement::new(RenderTag::P)),
            Tag::Emphasis => self.enter(RenderElement::new(RenderTag::Em)),
            Tag::Strong => self.enter(RenderElement::new(RenderTag::Strong)),
            // Headings
            Tag::Heading {
                level, id, classes, ..
            } => {
                let mut element = RenderElement::new(level.into());

                if let Some(id) = id {
                    element.add_attribute(AttributeName::Id, id.into_string());
                }

                if !classes.is_empty() {
                    let classes_string = classes.join(" ");
                    element.add_attribute(AttributeName::Class, classes_string);
                }

                self.enter(element)
            }
            // Images and links
            Tag::Image {
                dest_url,
                title,
                id,
                ..
            } => {
                // Alt text - used for both the figure caption and in <img /> itself
                let alt = self.capture_next_as_text();

                // Images are converted to <figure /> under the hood, so that we can support captions
                let mut element = RenderElement::new(RenderTag::Figure);
                element.add_attribute(AttributeName::Id, id.into_string());
                element.add_attribute(AttributeName::Title, title.into_string());

                let mut img = RenderElement::new(RenderTag::Img);
                img.add_attribute(AttributeName::Src, dest_url.into_string());
                img.add_attribute(AttributeName::Alt, alt.clone());
                element.add_child(RenderNode::Element(img));

                let mut figcaption = RenderElement::new(RenderTag::FigCaption);
                figcaption.add_child(RenderNode::Text(alt));
                element.add_child(RenderNode::Element(figcaption));

                // Cannot place <figure /> in <p>, so we must get rid of it on the stack and put it back later
                let p = if let Some(RenderElement {
                    tag: RenderTag::P, ..
                }) = self.stack.last()
                {
                    self.stack.pop()
                } else {
                    None
                };

                self.output(RenderNode::Element(element));

                if let Some(p) = p {
                    self.enter(p);
                }
            }
            _ => todo!(),
        }
    }

    fn run_end(&mut self, tag: TagEnd) {
        match tag {
            TagEnd::Paragraph => self.leave(RenderTag::P),
            TagEnd::Emphasis => self.leave(RenderTag::Em),
            TagEnd::Strong => self.leave(RenderTag::Strong),
            TagEnd::Heading(level) => self.leave(level.into()),
            // We already generated the image in `start` (it's self-contained), so do nothing
            TagEnd::Image => {}
            _ => todo!(),
        }
    }

    fn run_token(&mut self, token: Event<'a>) {
        match token {
            Event::Start(tag) => self.run_start(tag),
            Event::End(tag) => self.run_end(tag),
            Event::Text(text) => {
                let words = text.split(' ').count();
                self.post_render.read_time += TimeDelta::seconds((words as i64) / 200);

                let node = RenderNode::Text(text.to_string());
                self.output(node)
            }
            _ => todo!(),
        }
    }

    pub fn run(mut self) -> RenderOutput {
        while let Some(token) = self.tokens.next() {
            self.run_token(token);
        }

        RenderOutput {
            nodes: self.output,
            post_render: self.post_render,
        }
    }
}

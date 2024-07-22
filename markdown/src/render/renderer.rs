use std::marker::PhantomData;

use chrono::TimeDelta;
use pulldown_cmark::{Event, Tag, TagEnd};

use crate::structs::metadata::PostRenderData;

use super::node::{AttributeName, RenderElement, RenderNode, RenderTag};

pub struct RenderOutput {
    pub nodes: Vec<RenderNode>,
    pub post_render: PostRenderData,
}

pub struct Renderer<'a, I> {
    tokens: I,
    output: Vec<RenderNode>,
    stack: Vec<RenderElement>,
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

    fn run_start(&mut self, tag: Tag) {
        match tag {
            Tag::Paragraph => self.enter(RenderElement::new(RenderTag::P)),
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
            _ => {}
        }
    }

    fn run_end(&mut self, tag: TagEnd) {
        match tag {
            TagEnd::Paragraph => self.leave(RenderTag::P),
            TagEnd::Heading(level) => self.leave(level.into()),
            _ => {}
        }
    }

    fn run_token(&mut self, token: Event<'a>) {
        match token {
            Event::Start(tag) => self.run_start(tag),
            Event::End(tag) => self.run_end(tag),
            Event::Text(text) => {
                let node = RenderNode::Text(text.to_string());
                self.output(node);
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
            post_render: PostRenderData {
                read_time: TimeDelta::zero(),
            },
        }
    }
}

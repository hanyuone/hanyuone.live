use std::marker::PhantomData;

use pulldown_cmark::{Event, Tag, TagEnd};

use super::node::{RenderElement, RenderNode, RenderTag};

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

        if let Some(new_top) = self.stack.last_mut() {
            new_top.children.push(RenderNode::Element(top));
        } else {
            self.output.push(RenderNode::Element(top));
        }
    }

    fn run_start(&mut self, tag: Tag) {
        match tag {
            Tag::Paragraph => self.enter(RenderElement::new(RenderTag::P)),
            _ => {},
        }
    }

    fn run_end(&mut self, tag: TagEnd) {
        match tag {
            TagEnd::Paragraph => self.leave(RenderTag::P),
            _ => {},
        }
    }

    fn run_token(&mut self, token: Event<'a>) {
        match token {
            Event::Start(tag) => self.run_start(tag),
            Event::End(tag) => self.run_end(tag),
            Event::Text(text) => {
                let node = RenderNode::Text(text.to_string());
                self.output.push(node);
            }
            _ => todo!(),
        }
    }

    pub fn run(mut self) -> Vec<RenderNode> {
        while let Some(token) = self.tokens.next() {
            self.run_token(token);
        }

        self.output
    }
}

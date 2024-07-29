use std::marker::PhantomData;

use chrono::TimeDelta;
use pulldown_cmark::{BlockQuoteKind, Event, Tag, TagEnd};

use crate::structs::metadata::PostTranslateData;

use super::{element::{AttributeName, ElementTag, RenderElement}, node::{RenderIcon, RenderNode, RenderTag}};

pub struct TranslateOutput {
    pub nodes: Vec<RenderNode>,
    pub post_translate: PostTranslateData,
}

struct CalloutMetadata {
    class: &'static str,
    title: &'static str,
    icon: RenderIcon,
}

fn get_metadata(kind: BlockQuoteKind) -> CalloutMetadata {
    match kind {
        BlockQuoteKind::Note => CalloutMetadata {
            class: "callout-note",
            title: "Note",
            icon: RenderIcon::Note,
        },
        BlockQuoteKind::Tip => CalloutMetadata {
            class: "callout-tip",
            title: "Tip",
            icon: RenderIcon::Tip,
        },
        BlockQuoteKind::Important => CalloutMetadata {
            class: "callout-important",
            title: "Important",
            icon: RenderIcon::Important,
        },
        BlockQuoteKind::Warning => CalloutMetadata {
            class: "callout-warning",
            title: "Warning",
            icon: RenderIcon::Warning,
        },
        BlockQuoteKind::Caution => CalloutMetadata {
            class: "callout-caution",
            title: "Caution",
            icon: RenderIcon::Caution,
        },
    }
}

/// Helper struct used for converting Markdown events (generated via `pulldown_cmark`)
/// into a simplified virtual DOM that can easily be converted to work with Yew.
pub struct Translator<'a, I> {
    tokens: I,
    output: Vec<RenderNode>,
    stack: Vec<RenderElement>,
    post_translate: PostTranslateData,
    phantom: PhantomData<&'a I>,
}

impl<'a, I> Translator<'a, I>
where
    I: Iterator<Item = Event<'a>>,
{
    pub fn new(tokens: I) -> Self {
        Self {
            tokens,
            output: vec![],
            stack: vec![],
            post_translate: PostTranslateData {
                read_time: TimeDelta::zero(),
            },
            phantom: PhantomData,
        }
    }

    fn output<N>(&mut self, node: N)
    where
        N: Into<RenderNode>,
    {
        if let Some(top) = self.stack.last_mut() {
            top.add_child(node.into());
        } else {
            self.output.push(node.into());
        }
    }

    fn enter(&mut self, element: RenderElement) {
        self.stack.push(element);
    }

    fn leave(&mut self, tag: RenderTag) {
        let Some(top) = self.stack.pop() else {
            panic!("Stack underflow");
        };

        match tag {
            RenderTag::Element(etag) => {
                assert!(
                    top.tag == etag,
                    "Expected to pop <{}>, found <{}>",
                    etag,
                    top.tag
                );        
            },
            RenderTag::Callout => todo!(),
        }
        
        self.output(top)
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

    fn generate_callout(&mut self, kind: BlockQuoteKind) {
        // TODO: style MD callouts properly (consider moving to FE?)
        let mut callout = RenderElement::new(ElementTag::Div);

        let metadata = get_metadata(kind);

        // Add class for colour background
        callout.add_attribute(AttributeName::Class, metadata.class.to_string());

        // Add icon and title
        let mut heading = RenderElement::new(ElementTag::Div);
        heading.add_attribute(AttributeName::Class, "flex flex-row".to_string());

        let mut title = RenderElement::new(ElementTag::Strong);
        title.add_child(RenderNode::Text(metadata.title.to_string()));

        heading.add_child(RenderNode::Icon(metadata.icon));
        heading.add_child(RenderNode::Element(title));

        callout.add_child(RenderNode::Element(heading));

        self.enter(callout);
    }

    fn run_start(&mut self, tag: Tag) {
        match tag {
            // Text styles
            Tag::Paragraph => self.enter(RenderElement::new(ElementTag::P)),
            Tag::Emphasis => self.enter(RenderElement::new(ElementTag::Em)),
            Tag::Strong => self.enter(RenderElement::new(ElementTag::Strong)),
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
            // Blockquotes and callouts
            Tag::BlockQuote(kind) => match kind {
                Some(kind) => self.generate_callout(kind),
                None => {
                    let mut blockquote = RenderElement::new(ElementTag::Div);
                    blockquote.add_attribute(AttributeName::Class, "blockquote".to_string());
                    self.enter(blockquote);
                }
            },
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
                let mut element = RenderElement::new(ElementTag::Figure);
                element.add_attribute(AttributeName::Id, id.into_string());
                element.add_attribute(AttributeName::Title, title.into_string());

                let mut img = RenderElement::new(ElementTag::Img);
                img.add_attribute(AttributeName::Src, dest_url.into_string());
                img.add_attribute(AttributeName::Alt, alt.clone());
                element.add_child(RenderNode::Element(img));

                let mut figcaption = RenderElement::new(ElementTag::FigCaption);
                figcaption.add_child(RenderNode::Text(alt));
                element.add_child(RenderNode::Element(figcaption));

                // Cannot place <figure /> in <p>, so we must get rid of it on the stack and put it back later
                let p = if let Some(RenderElement {
                    tag: ElementTag::P, ..
                }) = self.stack.last()
                {
                    self.stack.pop()
                } else {
                    None
                };

                self.output(element);

                if let Some(p) = p {
                    self.enter(p);
                }
            }
            _ => todo!(),
        }
    }

    fn run_end(&mut self, tag: TagEnd) {
        match tag {
            TagEnd::Paragraph => self.leave(RenderTag::Element(ElementTag::P)),
            TagEnd::Emphasis => self.leave(RenderTag::Element(ElementTag::Em)),
            TagEnd::Strong => self.leave(RenderTag::Element(ElementTag::Strong)),
            TagEnd::Heading(level) => self.leave(RenderTag::Element(level.into())),
            // Blockquotes are always rendered as divs
            TagEnd::BlockQuote => self.leave(RenderTag::Element(ElementTag::Div)),
            // We already generated the image in `start` (it's self-contained), so do nothing
            TagEnd::Image => {}
            _ => todo!(),
        }
    }

    fn run_token(&mut self, token: Event<'a>) {
        match token {
            Event::Text(text) => {
                let words = text.split(' ').count();
                self.post_translate.read_time += TimeDelta::seconds((words as i64) / 200);

                self.output(text.to_string())
            }
            Event::SoftBreak => self.output("\n".to_string()),
            Event::Start(tag) => self.run_start(tag),
            Event::End(tag) => self.run_end(tag),
            _ => todo!(),
        }
    }

    pub fn run(mut self) -> TranslateOutput {
        while let Some(token) = self.tokens.next() {
            self.run_token(token);
        }

        TranslateOutput {
            nodes: self.output,
            post_translate: self.post_translate,
        }
    }
}

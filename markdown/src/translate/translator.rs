use std::collections::HashMap;

use pulldown_cmark::{BlockQuoteKind, CowStr, Event, Tag, TagEnd};

use crate::structs::metadata::PostTranslateData;

use super::{
    element::{AttributeName, ElementTag, RenderElement},
    error::TranslateError,
    node::{RenderCallout, RenderNode, RenderTag},
};

pub struct TranslateOutput {
    pub nodes: Vec<RenderNode>,
    pub post_translate: PostTranslateData,
}

/// Helper struct used for converting Markdown events (generated via `pulldown_cmark`)
/// into a simplified virtual DOM that can easily be converted to work with Yew.
pub struct Translator<'a, I> {
    tokens: I,
    output: Vec<RenderNode>,
    stack: Vec<RenderNode>,
    post_translate: PostTranslateData,
    footnotes: HashMap<CowStr<'a>, usize>,
}

impl<'a, I> Translator<'a, I>
where
    I: Iterator<Item = Event<'a>>,
{
    /// Creates a new translator, loaded with a list of tokens provided
    /// by `pulldown_cmark`.
    pub fn new(tokens: I) -> Self {
        Self {
            tokens,
            output: vec![],
            stack: vec![],
            post_translate: PostTranslateData { words: 0 },
            footnotes: HashMap::new(),
        }
    }

    fn get_footnote_index(&mut self, name: CowStr<'a>) -> usize {
        let next = self.footnotes.len() + 1;
        *self.footnotes.entry(name).or_insert(next)
    }

    /// Adds a new node either into the topmost container, or directly
    /// into the output.
    fn output<N>(&mut self, node: N)
    where
        N: Into<RenderNode>,
    {
        if let Some(top) = self.stack.last_mut() {
            match top {
                RenderNode::Element(element) => element.add_child(node.into()),
                RenderNode::Callout(callout) => callout.add_child(node.into()),
                _ => unreachable!("Only containers should be put onto stack"),
            }
        } else {
            self.output.push(node.into());
        }
    }

    /// Enters into the current container.
    fn enter<N>(&mut self, element: N)
    where
        N: Into<RenderNode>,
    {
        self.stack.push(element.into());
    }

    fn check_top(&self, top: &RenderNode, tag: RenderTag) -> Result<(), TranslateError> {
        match tag {
            // Match element with element
            RenderTag::Element(etag) => {
                let RenderNode::Element(element) = top else {
                    return Err(TranslateError::ElementError {
                        expected: etag,
                        result: None,
                    });
                };

                if element.tag != etag {
                    return Err(TranslateError::ElementError {
                        expected: etag,
                        result: Some(element.tag),
                    });
                }

                Ok(())
            }
            // Match callout with callout
            RenderTag::Callout => {
                let RenderNode::Callout(_) = top else {
                    return Err(TranslateError::CalloutError);
                };

                Ok(())
            }
        }
    }

    /// Leaves the container, checking that there's nothing wrong
    /// with our entering/leaving process.
    fn leave(&mut self, tag: RenderTag) -> Result<(), TranslateError> {
        let Some(top) = self.stack.pop() else {
            panic!("Stack underflow");
        };

        self.check_top(&top, tag)?;
        self.output(top);
        Ok(())
    }

    fn leave_any(&mut self, tags: Vec<RenderTag>) -> Result<(), TranslateError> {
        let Some(top) = self.stack.pop() else {
            panic!("Stack underflow");
        };

        for tag in tags.clone() {
            let result = self.check_top(&top, tag);

            if let Ok(()) = result {
                self.output(top);
                return Ok(());
            }
        }

        Err(TranslateError::NoMatchError { tags })
    }

    /// Consumes the next HTML element in our Markdown text and returns it as
    /// a single-lined string.
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
        let callout = RenderCallout::new(kind.into());
        self.enter(callout);
    }

    fn generate_image(&mut self, dest_url: CowStr, title: CowStr, id: CowStr) {
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
        let p = if let Some(RenderNode::Element(RenderElement {
            tag: ElementTag::P, ..
        })) = self.stack.last()
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

    fn run_start(&mut self, tag: Tag<'a>) {
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
                    let mut blockquote = RenderElement::new(ElementTag::BlockQuote);
                    blockquote.add_attribute(AttributeName::Class, "blockquote".to_string());
                    self.enter(blockquote);
                }
            },
            // Links and images
            Tag::Link {
                dest_url, title, ..
            } => {
                let mut a = RenderElement::new(ElementTag::A);
                a.add_attribute(AttributeName::Title, title.to_string());
                a.add_attribute(AttributeName::Href, dest_url.to_string());
                self.enter(a);
            }
            Tag::Image {
                dest_url,
                title,
                id,
                ..
            } => self.generate_image(dest_url, title, id),
            // Lists
            Tag::List(_) => self.enter(RenderElement::new(ElementTag::Ul)),
            Tag::Item => self.enter(RenderElement::new(ElementTag::Li)),
            // Footnotes
            Tag::FootnoteDefinition(name) => {
                let mut footnote = RenderElement::new(ElementTag::Div);
                footnote.add_attribute(AttributeName::Id, name.to_string());

                let index = self.get_footnote_index(name);
                let mut p_index = RenderElement::new(ElementTag::P);
                p_index.add_child(RenderNode::Text(format!("{}: ", index)));
                footnote.add_child(RenderNode::Element(p_index));

                self.enter(footnote);
            }
            _ => todo!(),
        }
    }

    // Called whenever we're trying to exit a footnote. Attaches the footnote
    // index onto the rest of the footnote itself, if possible.
    fn attach_footnote_index(&mut self) {
        // We know the top item of the stack (if a stack exists) or the output
        // *must* be our footnote
        let footnote_node = if self.stack.is_empty() {
            self.output.last_mut().unwrap()
        } else {
            self.stack.last_mut().unwrap()
        };

        let RenderNode::Element(ref mut footnote) = footnote_node else {
            unreachable!();
        };

        let children = &mut footnote.children;
        // Reverse because we're removing the first two elements and potentially
        // adding them back later, popping is an O(1) operation
        children.reverse();

        // Footnote element must contain the footnote index itself
        let RenderNode::Element(mut index) = children.pop().unwrap() else {
            unreachable!();
        };

        // Guards to ensure we're pushing to an element with <p> as first tag
        let Some(first_content) = children.pop() else {
            children.push(index.into());
            children.reverse();
            return;
        };

        let RenderNode::Element(mut first_content) = first_content else {
            children.push(first_content);
            children.push(index.into());
            children.reverse();
            return;
        };

        if first_content.tag != ElementTag::P {
            children.push(first_content.into());
            children.push(index.into());
            children.reverse();
            return;
        }

        // Attach footnote index text to <p>
        let index_text = index.children.pop().unwrap();
        first_content.children.insert(0, index_text);

        children.push(first_content.into());
        children.reverse();
    }

    fn run_end(&mut self, tag: TagEnd) {
        let result = match tag {
            // Text and decorations
            TagEnd::Paragraph => self.leave(RenderTag::Element(ElementTag::P)),
            TagEnd::Emphasis => self.leave(RenderTag::Element(ElementTag::Em)),
            TagEnd::Strong => self.leave(RenderTag::Element(ElementTag::Strong)),
            TagEnd::Heading(level) => self.leave(RenderTag::Element(level.into())),
            // Blockquotes are always rendered as divs
            TagEnd::BlockQuote => self.leave_any(vec![
                RenderTag::Element(ElementTag::BlockQuote),
                RenderTag::Callout,
            ]),
            // Links and images
            TagEnd::Link => self.leave(RenderTag::Element(ElementTag::A)),
            // We already generated the image in `start` (it's self-contained), so do nothing
            TagEnd::Image => Ok(()),
            // Lists
            TagEnd::List(_) => self.leave(RenderTag::Element(ElementTag::Ul)),
            TagEnd::Item => self.leave(RenderTag::Element(ElementTag::Li)),
            TagEnd::FootnoteDefinition => {
                self.leave(RenderTag::Element(ElementTag::Div))
                    .and_then(|_| {
                        self.attach_footnote_index();
                        Ok(())
                    })
            }
            _ => todo!(),
        };

        if let Err(e) = result {
            panic!("{}", e);
        }
    }

    fn run_token(&mut self, token: Event<'a>) {
        match token {
            Event::Text(text) => {
                let words = text.split(' ').count();
                self.post_translate.words += words;

                self.output(text.to_string())
            }
            Event::SoftBreak => self.output("\n".to_string()),
            Event::HardBreak => {
                self.output(RenderElement::new(ElementTag::Br));
            }
            Event::Start(tag) => self.run_start(tag),
            Event::End(tag) => self.run_end(tag),
            Event::FootnoteReference(name) => {
                let mut sup = RenderElement::new(ElementTag::Sup);
                let mut anchor = RenderElement::new(ElementTag::A);
                anchor.add_attribute(AttributeName::Href, format!("#{name}"));

                let index = self.get_footnote_index(name);
                anchor.add_child(RenderNode::Text(index.to_string()));

                sup.add_child(RenderNode::Element(anchor));
                self.output(sup);
            }
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

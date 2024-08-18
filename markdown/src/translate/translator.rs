use std::collections::HashMap;

use pulldown_cmark::{BlockQuoteKind, CowStr, Event, Tag, TagEnd};

use crate::structs::metadata::PostTranslateData;

use super::{
    element::{AttributeName, ElementTag, RenderElement},
    error::TranslateError,
    node::{RenderCallout, RenderNode, RenderTag},
};

struct Footnotes<'a> {
    index: usize,
    mapping: HashMap<CowStr<'a>, (usize, RenderElement)>,
}

impl<'a> Footnotes<'a> {
    pub fn new() -> Self {
        Self {
            index: 1,
            mapping: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: CowStr<'a>, element: RenderElement) {
        self.mapping.insert(name, (self.index, element));
        self.index += 1;
    }

    pub fn get_index(&self, name: CowStr<'a>) -> Option<usize> {
        self.mapping.get(&name).map(|tup| tup.0)
    }

    pub fn as_nodes(self) -> Vec<RenderNode> {
        let mut sorted_footnotes = self.mapping
            .into_iter()
            .map(|(name, (index, element))| (index, name, element))
            .collect::<Vec<_>>();

        sorted_footnotes.sort_by_key(|(index, _, _)| *index);

        sorted_footnotes.into_iter()
            .map(|(index, name, mut element)| {
                let mut footnote = RenderElement::new(ElementTag::Div);
                footnote.add_attribute(AttributeName::Id, format!("footnote_{}", name));

                let children = &mut element.children;

                // Add <p>{index}: </p> at beginning of each footnote
                let index_text = format!("{}: ", index);

                // We know that the first child has to be a render element
                let RenderNode::Element(first_element) = children.first_mut().unwrap() else {
                    unreachable!()
                };

                if first_element.tag == ElementTag::P {
                    first_element.children.insert(0, RenderNode::Text(index_text));
                } else {
                    let mut index_element = RenderElement::new(ElementTag::P);
                    index_element.add_child(index_text.into());
                    children.insert(0, index_element.into());
                }

                // Add return button at end of each footnote
                let mut return_button = RenderElement::new(ElementTag::A);
                return_button.add_attribute(AttributeName::Href, format!("anchor_{}", name));
                return_button.add_child("↩️".to_string().into());

                // We know that the last child has to be a render element
                let RenderNode::Element(last_element) = children.last_mut().unwrap() else {
                    unreachable!()
                };

                if last_element.tag == ElementTag::P {
                    // Add space
                    if let RenderNode::Text(space_adder) = last_element.children.last_mut().unwrap() {
                        space_adder.insert(0, ' ');
                    } else {
                        last_element.add_child(" ".to_string().into());
                    }

                    last_element.add_child(return_button.into());
                } else {
                    let mut return_element = RenderElement::new(ElementTag::P);
                    return_element.add_child(return_button.into());
                    element.add_child(return_element.into());
                }
                
                footnote.add_child(element.into());
                footnote.into()
            })
            .collect::<Vec<_>>()

    }
}

pub struct TranslateOutput {
    pub nodes: Vec<RenderNode>,
    pub post_translate: PostTranslateData,
}

// TODO for footnotes:
// 1. Render footnotes separately, always include at very bottom of file
// 2. Return button at end of footnote
// 3. Hover tooltip for footnote

/// Helper struct used for converting Markdown events (generated via `pulldown_cmark`)
/// into a simplified virtual DOM that can easily be converted to work with Yew.
pub struct Translator<'a, I> {
    tokens: I,
    output: Vec<RenderNode>,
    stack: Vec<RenderNode>,
    footnotes: Footnotes<'a>,
    _footnotes: HashMap<CowStr<'a>, usize>,
    is_footnote: bool,
    post_translate: PostTranslateData,
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
            footnotes: Footnotes::new(),
            _footnotes: HashMap::new(),
            is_footnote: false,
            post_translate: PostTranslateData { words: 0 },
        }
    }

    //// HELPER FUNCTIONS

    fn get_footnote_index(&mut self, name: CowStr<'a>) -> usize {
        let next = self._footnotes.len() + 1;
        *self._footnotes.entry(name).or_insert(next)
    }

    //// TRANSLATION FUNCTIONS

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

    /// Called whenever we encounter the *first* element after a footnote. This edgecase
    /// needs to be checked because we don't want the index `<p>` have a line separator with
    /// the footnote contents if the node we're adding is also a `<p>` element.
    fn enter_footnote<N>(&mut self, node: N)
    where
        N: Into<RenderNode>,
    {
        // We know that we have a footnote on the top of stack
        let RenderNode::Element(footnote_element) = self.stack.last_mut().unwrap() else {
            unreachable!()
        };

        // We only want to do footnote manipulation if the element we're adding is another <p>
        let node = node.into();

        let RenderNode::Element(mut added_element) = node else {
            footnote_element.add_child(node);
            return;
        };

        if added_element.tag != ElementTag::P {
            footnote_element.add_child(added_element.into());
            return;
        }

        // We know that the footnote on the top of stack has one element in it so far,
        // in the format <p>{index}: </p>. Wipe it from the existing footnote element using pop()
        let RenderNode::Element(mut index_element) = footnote_element.children.pop().unwrap()
        else {
            unreachable!()
        };

        let RenderNode::Text(index_text) = index_element.children.pop().unwrap() else {
            unreachable!()
        };

        // The text is now part of the <p> we're adding, as if it never existed in the
        // footnote in the first place
        added_element
            .children
            .insert(0, RenderNode::Text(index_text));

        self.stack.push(added_element.into());
    }

    /// Enters into the current container.
    fn enter<N>(&mut self, node: N)
    where
        N: Into<RenderNode>,
    {
        if self.is_footnote {
            self.enter_footnote(node);
            self.is_footnote = false;
            return;
        }

        self.stack.push(node.into());
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

                    if nest_level <= 0 {
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

    // TODO: add codeblocks w/ syntax highlighting
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
            Tag::List(start) => match start {
                Some(start) => {
                    let mut ol = RenderElement::new(ElementTag::Ol);
                    ol.add_attribute(AttributeName::Start, start.to_string());
                    self.enter(ol)
                }
                None => self.enter(RenderElement::new(ElementTag::Ul)),
            },
            Tag::Item => self.enter(RenderElement::new(ElementTag::Li)),
            // Footnotes
            Tag::FootnoteDefinition(name) => {
                let mut footnote = RenderElement::new(ElementTag::Div);
                footnote.add_attribute(AttributeName::Id, format!("footnote_{}", name));

                let index = self.get_footnote_index(name);
                let mut p_index = RenderElement::new(ElementTag::P);
                p_index.add_child(RenderNode::Text(format!("{}: ", index)));
                footnote.add_child(RenderNode::Element(p_index));

                self.enter(footnote);
                
                // We want to make sure the footnote flag triggers on the first item we add
                // *after* we enter the footnote
                self.is_footnote = true;
            }
            _ => todo!(),
        }
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
            TagEnd::List(is_ordered) => {
                if is_ordered {
                    self.leave(RenderTag::Element(ElementTag::Ol))
                } else {
                    self.leave(RenderTag::Element(ElementTag::Ul))
                }
            }
            TagEnd::Item => self.leave(RenderTag::Element(ElementTag::Li)),
            TagEnd::FootnoteDefinition => self.leave(RenderTag::Element(ElementTag::Div)),
            _ => todo!(),
        };

        if let Err(e) = result {
            panic!("{}", e);
        }
    }

    fn run_token(&mut self, token: Event<'a>) {
        match token {
            // Text
            Event::Text(text) => {
                let words = text.split(' ').count();
                self.post_translate.words += words;

                self.output(text.to_string())
            }
            Event::Code(text) => {
                let mut code = RenderElement::new(ElementTag::Code);
                code.add_child(RenderNode::Text(text.to_string()));

                self.output(code)
            }
            // Line breaks
            Event::SoftBreak => self.output("\n".to_string()),
            Event::HardBreak => {
                self.output(RenderElement::new(ElementTag::Br));
            }
            Event::Rule => {
                self.output(RenderElement::new(ElementTag::Hr));
            }
            // Starting and ending more complex elements
            Event::Start(tag) => self.run_start(tag),
            Event::End(tag) => self.run_end(tag),
            // Inline footnote references
            Event::FootnoteReference(name) => {
                let mut sup = RenderElement::new(ElementTag::Sup);
                let mut anchor = RenderElement::new(ElementTag::A);
                anchor.add_attribute(AttributeName::Href, format!("#footnote_{name}"));

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

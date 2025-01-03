use pulldown_cmark::{BlockQuoteKind, CowStr, Event, Tag, TagEnd};

use crate::structs::metadata::PostTranslateData;

use super::{
    complex::{footnotes::Footnotes, table::Table},
    element::{AttributeName, ElementTag, RenderElement},
    error::TranslateError,
    node::{RenderCallout, RenderHtml, RenderNode, RenderTag},
};

pub struct TranslateOutput {
    pub nodes: Vec<RenderNode>,
    pub post_translate: PostTranslateData,
}

/// Helper struct used for converting Markdown events (generated via `pulldown_cmark`)
/// into a simplified virtual DOM that can easily be converted to work with Yew.
pub struct Translator<'a, I> {
    // Basic variables
    tokens: I,
    output: Vec<RenderNode>,
    stack: Vec<RenderNode>,
    // Footnote variables
    footnotes: Footnotes<'a>,
    current_footnote: Option<CowStr<'a>>,
    // Table variables
    table: Option<Table>,
    in_cell: bool,
    cell_output: Vec<RenderNode>,
    // After translation
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
            // Basic functionality
            tokens,
            output: vec![],
            stack: vec![],
            // Footnote variables
            footnotes: Footnotes::new(),
            current_footnote: None,
            // Table variables
            table: None,
            in_cell: false,
            cell_output: vec![],
            // After translation
            post_translate: PostTranslateData { words: 0 },
        }
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
        } else if self.in_cell {
            self.cell_output.push(node.into());
        } else {
            self.output.push(node.into());
        }
    }

    /// Enters into the current container.
    fn enter<N>(&mut self, node: N)
    where
        N: Into<RenderNode>,
    {
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
            RenderTag::Html => {
                let RenderNode::Html(_) = top else {
                    return Err(TranslateError::RawHtmlError);
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

    fn leave_footnote(&mut self) -> Result<(), TranslateError> {
        let Some(top) = self.stack.pop() else {
            panic!("Stack underflow");
        };

        let RenderNode::Element(top_element) = top else {
            unreachable!()
        };

        self.footnotes.insert(
            self.current_footnote
                .clone()
                .expect("Should be inside a footnote right now"),
            top_element,
        );
        self.current_footnote = None;
        Ok(())
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
            // === Raw HTML ===
            Tag::HtmlBlock => {}

            // === Text styles ===
            Tag::Paragraph => self.enter(RenderElement::new(ElementTag::P)),
            Tag::Emphasis => self.enter(RenderElement::new(ElementTag::Em)),
            Tag::Strong => self.enter(RenderElement::new(ElementTag::Strong)),

            // === Headings ===
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

            // === Blockquotes and callouts ===
            Tag::BlockQuote(kind) => match kind {
                Some(kind) => self.generate_callout(kind),
                None => {
                    let mut blockquote = RenderElement::new(ElementTag::BlockQuote);
                    blockquote.add_attribute(AttributeName::Class, "blockquote".to_string());
                    self.enter(blockquote);
                }
            },

            // === Links and images ===
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

            // === Lists ===
            Tag::List(start) => match start {
                Some(start) => {
                    let mut ol = RenderElement::new(ElementTag::Ol);
                    ol.add_attribute(AttributeName::Start, start.to_string());
                    self.enter(ol)
                }
                None => self.enter(RenderElement::new(ElementTag::Ul)),
            },
            Tag::Item => self.enter(RenderElement::new(ElementTag::Li)),

            // === Tables ===
            Tag::Table(alignment) => self.table = Some(Table::new(alignment)),
            Tag::TableHead => {
                self.table.as_mut().unwrap().is_head = true;
                self.table.as_mut().unwrap().add_row();
            }
            Tag::TableRow => self.table.as_mut().unwrap().add_row(),
            Tag::TableCell => self.in_cell = true,

            // === Footnotes ===
            Tag::FootnoteDefinition(name) => {
                self.current_footnote = Some(name);
                self.enter(RenderElement::new(ElementTag::Div))
            }

            _ => todo!(),
        }
    }

    fn run_end(&mut self, tag: TagEnd) {
        let result = match tag {
            // === Raw HTML ===
            TagEnd::HtmlBlock => Ok(()),

            // === Text and decorations ===
            TagEnd::Paragraph => self.leave(RenderTag::Element(ElementTag::P)),
            TagEnd::Emphasis => self.leave(RenderTag::Element(ElementTag::Em)),
            TagEnd::Strong => self.leave(RenderTag::Element(ElementTag::Strong)),
            TagEnd::Heading(level) => self.leave(RenderTag::Element(level.into())),

            // === Blockquotes ===
            // Always rendered as divs
            TagEnd::BlockQuote => self.leave_any(vec![
                RenderTag::Element(ElementTag::BlockQuote),
                RenderTag::Callout,
            ]),

            // === Links and images ===
            TagEnd::Link => self.leave(RenderTag::Element(ElementTag::A)),
            // We already generated the image in `start` (it's self-contained), so do nothing
            TagEnd::Image => Ok(()),

            // === Lists ===
            TagEnd::List(is_ordered) => {
                if is_ordered {
                    self.leave(RenderTag::Element(ElementTag::Ol))
                } else {
                    self.leave(RenderTag::Element(ElementTag::Ul))
                }
            }
            TagEnd::Item => self.leave(RenderTag::Element(ElementTag::Li)),

            // === Tables ===
            TagEnd::Table => {
                let table_node = self.table.take().unwrap().to_node();
                self.output(table_node);

                Ok(())
            }
            TagEnd::TableHead => {
                self.table.as_mut().unwrap().is_head = false;
                Ok(())
            }
            TagEnd::TableRow => Ok(()),
            TagEnd::TableCell => {
                let cell = std::mem::take(&mut self.cell_output);
                let add_command = self.table.as_mut().unwrap().add_contents(cell);

                if let Err(err) = add_command {
                    Err(err)
                } else {
                    self.in_cell = false;
                    Ok(())
                }
            }

            // === Footnotes ===
            TagEnd::FootnoteDefinition => self.leave_footnote(),

            _ => todo!(),
        };

        if let Err(e) = result {
            panic!("{}", e);
        }
    }

    fn run_token(&mut self, token: Event<'a>) {
        match token {
            // === Raw HTML ===
            Event::Html(html) => self.output(RenderHtml(html.to_string())),
            Event::InlineHtml(html) => self.output(RenderHtml(html.to_string())),

            // === Text ===
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

            // === Line breaks ===
            Event::SoftBreak => self.output("\n".to_string()),
            Event::HardBreak => {
                self.output(RenderElement::new(ElementTag::Br));
            }
            Event::Rule => {
                self.output(RenderElement::new(ElementTag::Hr));
            }

            // === Complex elements ===
            Event::Start(tag) => self.run_start(tag),
            Event::End(tag) => self.run_end(tag),

            // === Footnotes ===
            Event::FootnoteReference(name) => {
                self.footnotes.add_index(name.clone());

                let mut sup = RenderElement::new(ElementTag::Sup);
                sup.add_attribute(AttributeName::Id, format!("anchor_{name}"));

                let mut anchor = RenderElement::new(ElementTag::A);
                anchor.add_attribute(AttributeName::Href, format!("#footnote_{name}"));

                let index = self.footnotes.get_index(name).unwrap();
                anchor.add_child(format!("{index}").into());

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

        let mut nodes = self.output;

        let footnote_nodes = self.footnotes.to_nodes();
        nodes.extend(footnote_nodes);

        TranslateOutput {
            nodes,
            post_translate: self.post_translate,
        }
    }
}

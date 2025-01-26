use std::sync::LazyLock;

use pulldown_cmark::{BlockQuoteKind, CowStr, Event, Tag, TagEnd};
use syntect::parsing::SyntaxSet;

use crate::structs::metadata::PostTranslateData;

use super::{
    complex::footnotes::Footnotes,
    container::{callout::Callout, table::Table, Container, ContainerTag},
    element::{AttributeName, ElementTag, RenderElement},
    error::TranslateError,
    node::{RenderHtml, RenderNode},
};

static SYNTAX_SET: LazyLock<SyntaxSet> = LazyLock::new(|| SyntaxSet::load_defaults_newlines());

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
    stack: Vec<Container>,
    // Footnote variables
    footnotes: Footnotes<'a>,
    current_footnote: Option<CowStr<'a>>,
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
            // After translation
            post_translate: PostTranslateData { words: 0 },
        }
    }

    //// OUTPUT FUNCTIONS

    /// Adds a new node either into the topmost container, or directly
    /// into the output.
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

    //// STACK FUNCTIONS

    /// Enters into the current container.
    fn enter<C>(&mut self, container: C)
    where
        C: Into<Container>,
    {
        self.stack.push(container.into());
    }

    fn check_top(&self, top: &Container, tag: ContainerTag) -> Result<(), TranslateError> {
        match tag {
            // Match element with element
            ContainerTag::Element(etag) => {
                let Container::Element(element) = top else {
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
            ContainerTag::Callout => {
                let Container::Callout(_) = top else {
                    return Err(TranslateError::CalloutError);
                };

                Ok(())
            }
            ContainerTag::Table => {
                let Container::Table(_) = top else {
                    return Err(TranslateError::TableMergeError);
                };

                Ok(())
            }
        }
    }

    /// "Leaves" the container - i.e. declares that the container has no
    /// more child elements and moves it to `self.output`.
    ///
    /// # Panics
    ///
    /// Panics if the stack is empty.
    ///
    /// # Errors
    ///
    /// This function will return an error if the tag provided doesn't match
    /// with the container on top of the stack.
    fn leave(&mut self, tag: ContainerTag) -> Result<(), TranslateError> {
        let Some(top) = self.stack.pop() else {
            panic!("Stack underflow");
        };

        self.check_top(&top, tag)?;
        self.output(top);

        Ok(())
    }

    /// "Leaves" the container satisfying any of `tags`.
    ///
    /// # Panics
    ///
    /// Panics if the stack is empty.
    ///
    /// # Errors
    ///
    /// This function will return an error if none of the tags provided match
    /// the container on top of the stack.
    fn leave_any(&mut self, tags: Vec<ContainerTag>) -> Result<(), TranslateError> {
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

    /// Leaves the current footnote, adding it to `self.footnotes` for
    /// post-traversal processing.
    ///
    /// # Panics
    ///
    /// Panics if the stack is empty.
    fn leave_footnote(&mut self) {
        let Some(top) = self.stack.pop() else {
            panic!("Stack underflow");
        };

        let Container::Element(top_element) = top else {
            // By the way `pulldown_cmark` processes footnotes,
            // the top container *has* to be a `RenderElement` that
            // contains all the relevant information.
            unreachable!()
        };

        self.footnotes.insert(
            self.current_footnote
                .clone()
                .expect("Should be inside a footnote right now"),
            top_element,
        );
        self.current_footnote = None;
    }

    //// HELPER FUNCTIONS

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
        let callout = Callout::new(kind.into());
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
        let p = if let Some(Container::Element(RenderElement {
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

    /// Fetches the table which is at the top of the stack. Should only be called
    /// when the translator interacts with a `Table` token of any kind, when the
    /// top element of the stack is guaranteed to be a table.
    ///
    /// # Panics
    ///
    /// Panics if the top element is not a table, which should never happen in the
    /// correct conditions.
    fn get_top_table(&mut self) -> &mut Table {
        let table = self.stack.last_mut();

        let Some(Container::Table(table)) = table else {
            panic!("Top item in translator stack is not a table");
        };

        table
    }

    //// TRANSLATOR LOOP

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

            // === Code blocks
            Tag::CodeBlock(kind) => {
                let language = match kind {
                    pulldown_cmark::CodeBlockKind::Indented => None,
                    pulldown_cmark::CodeBlockKind::Fenced(language) => {
                        let syntax = SYNTAX_SET.find_syntax_by_name(&language);
                        syntax.map(|syntax| syntax.name.clone())
                    }
                };

                
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
            Tag::Table(alignment) => self.enter(Table::new(alignment)),
            Tag::TableHead => {
                let table = self.get_top_table();
                table.is_head = true;
                table.add_row();
            }
            Tag::TableRow => {
                let table = self.get_top_table();
                table.add_row();
            }
            Tag::TableCell => {}

            // === Footnotes ===
            Tag::FootnoteDefinition(name) => {
                self.current_footnote = Some(name);
                self.enter(RenderElement::new(ElementTag::Div))
            }

            _ => todo!(),
        }
    }

    fn run_end(&mut self, tag: TagEnd) -> Result<(), TranslateError> {
        match tag {
            // === Raw HTML ===
            TagEnd::HtmlBlock => {}

            // === Text and decorations ===
            TagEnd::Paragraph => self.leave(ContainerTag::Element(ElementTag::P))?,
            TagEnd::Emphasis => self.leave(ContainerTag::Element(ElementTag::Em))?,
            TagEnd::Strong => self.leave(ContainerTag::Element(ElementTag::Strong))?,
            TagEnd::Heading(level) => self.leave(ContainerTag::Element(level.into()))?,

            // === Blockquotes ===
            // Always rendered as divs
            TagEnd::BlockQuote => self.leave_any(vec![
                ContainerTag::Element(ElementTag::BlockQuote),
                ContainerTag::Callout,
            ])?,

            // === Links and images ===
            TagEnd::Link => self.leave(ContainerTag::Element(ElementTag::A))?,
            // We already generated the image in `start` (it's self-contained), so do nothing
            TagEnd::Image => {}

            // === Lists ===
            TagEnd::List(is_ordered) => {
                if is_ordered {
                    self.leave(ContainerTag::Element(ElementTag::Ol))?
                } else {
                    self.leave(ContainerTag::Element(ElementTag::Ul))?
                }
            }
            TagEnd::Item => self.leave(ContainerTag::Element(ElementTag::Li))?,

            // === Tables ===
            TagEnd::Table => self.leave(ContainerTag::Table)?,
            TagEnd::TableHead => {
                let table = self.get_top_table();
                table.is_head = false;
            }
            TagEnd::TableRow => {}
            TagEnd::TableCell => {
                let table = self.get_top_table();
                table.create_cell()?
            }

            // === Footnotes ===
            TagEnd::FootnoteDefinition => self.leave_footnote(),

            _ => todo!(),
        }

        Ok(())
    }

    fn run_token(&mut self, token: Event<'a>) -> Result<(), TranslateError> {
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
            Event::End(tag) => self.run_end(tag)?,

            // === Footnotes ===
            Event::FootnoteReference(name) => {
                self.footnotes.add_index(name.clone());

                let mut sup = RenderElement::new(ElementTag::Sup);
                sup.add_attribute(AttributeName::Id, format!("anchor_{name}"));

                let mut anchor = RenderElement::new(ElementTag::A);
                anchor.add_attribute(AttributeName::Href, format!("#footnote_{name}"));

                let index = match self.footnotes.get_index(name.clone()) {
                    Some(index) => index,
                    None => {
                        return Err(TranslateError::FootnoteError {
                            name: name.to_string(),
                        })
                    }
                };

                anchor.add_child(format!("{index}").into());

                sup.add_child(RenderNode::Element(anchor));
                self.output(sup);
            }

            _ => todo!(),
        }

        Ok(())
    }

    pub fn run(mut self) -> Result<TranslateOutput, TranslateError> {
        while let Some(token) = self.tokens.next() {
            self.run_token(token)?;
        }

        let mut nodes = self.output;

        let footnote_nodes = self.footnotes.to_nodes();
        nodes.extend(footnote_nodes);

        Ok(TranslateOutput {
            nodes,
            post_translate: self.post_translate,
        })
    }
}

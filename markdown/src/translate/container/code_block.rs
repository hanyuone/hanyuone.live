use syntect::{easy::HighlightLines, highlighting::Color, parsing::SyntaxReference, util::LinesWithEndings};

use crate::translate::{
    element::{AttributeName, ElementTag, RenderElement}, error::TranslateError, highlight::{SYNTAX_SET, THEME_SET}, node::RenderNode
};

use super::Container;

pub struct CodeBlock<'a> {
    language: Option<&'a SyntaxReference>,
    contents: String,
}

impl<'a> CodeBlock<'a> {
    pub fn new(language: Option<&'a SyntaxReference>) -> Self {
        Self {
            language,
            contents: String::new(),
        }
    }

    pub fn add_child(&mut self, child: RenderNode) {
        let RenderNode::Text(contents) = child else {
            unreachable!();
        };

        self.contents = contents;
    }
}

impl<'a> TryFrom<CodeBlock<'a>> for RenderNode {
    type Error = TranslateError;

    fn try_from(value: CodeBlock) -> Result<Self, Self::Error> {
        let Some(language) = value.language else {
            let mut unstyled = RenderElement::new(ElementTag::Pre);
            
            let mut code = RenderElement::new(ElementTag::Code);
            code.add_child(Self::Text(value.contents));

            unstyled.add_child(code.into());
            return Ok(unstyled.into());
        };

        let theme = &THEME_SET.themes["base16-ocean.dark"];
        let mut highlighter = HighlightLines::new(language, theme);

        let token_lines = LinesWithEndings::from(&value.contents)
            .map(|line| highlighter.highlight_line(line, &SYNTAX_SET))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| TranslateError::CodeHighlightError)?;

        let mut styled = RenderElement::new(ElementTag::Pre);
        let mut code = RenderElement::new(ElementTag::Code);

        for line in token_lines {
            for (style, contents) in line {
                if contents.trim().is_empty() {
                    let text = RenderNode::Text(contents.to_string());
                    code.add_child(text);
                    continue;
                }
                
                let mut span = RenderElement::new(ElementTag::Span);

                let Color { r, g, b, .. } = style.foreground;
                let colour_text = format!("color: rgb({r}, {g}, {b})");
                println!("Colour text: {}", colour_text);

                span.add_attribute(AttributeName::Style, colour_text);
                span.add_child(Self::Text(contents.to_string()));

                code.add_child(span.into());
            }
        }

        styled.add_child(code.into());
        Ok(styled.into())
    }
}

impl<'a> From<CodeBlock<'a>> for Container<'a> {
    fn from(value: CodeBlock<'a>) -> Self {
        Self::CodeBlock(value)
    }
}

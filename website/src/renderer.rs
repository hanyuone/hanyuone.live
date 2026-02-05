use std::mem;

use icondata_core::IconData;
use leptos::{
    html::{
        a, blockquote, br, code, div, em, figcaption, figure, h1, h2, h3, h4, h5, h6, hr, img, li,
        ol, p, pre, script, span, strong, sup, table, tbody, td, th, thead, tr, ul,
    },
    prelude::*,
};
use leptos_icons::Icon;
use markdown::translate::{
    container,
    element::{ElementTag, RenderElement},
    node::{RenderIcon, RenderNode},
};

use crate::components::blog_post::callout::{Callout, CalloutData};

/// Given a predefined `ElementTag` and a list of children, creates
/// a Leptos `AnyView` object. Necessary because of Leptos's HTML
/// element type system - we cannot dynamically generate elements based
/// on tags.
fn create_view(tag: ElementTag, children: Vec<AnyView>) -> AnyView {
    match tag {
        ElementTag::A => a().child(children).into_any(),
        ElementTag::BlockQuote => blockquote().child(children).into_any(),
        ElementTag::Br => br().into_any(),
        ElementTag::Code => code().child(children).into_any(),
        ElementTag::Div => div().child(children).into_any(),
        ElementTag::Em => em().child(children).into_any(),
        ElementTag::FigCaption => figcaption().child(children).into_any(),
        ElementTag::Figure => figure().child(children).into_any(),
        ElementTag::H1 => h1().child(children).into_any(),
        ElementTag::H2 => h2().child(children).into_any(),
        ElementTag::H3 => h3().child(children).into_any(),
        ElementTag::H4 => h4().child(children).into_any(),
        ElementTag::H5 => h5().child(children).into_any(),
        ElementTag::H6 => h6().child(children).into_any(),
        ElementTag::Hr => hr().into_any(),
        ElementTag::Img => img().into_any(),
        ElementTag::Li => li().child(children).into_any(),
        ElementTag::Ol => ol().child(children).into_any(),
        ElementTag::P => p().child(children).into_any(),
        ElementTag::Pre => pre().child(children).into_any(),
        ElementTag::Script => script().child(children).into_any(),
        ElementTag::Span => span().child(children).into_any(),
        ElementTag::Strong => strong().child(children).into_any(),
        ElementTag::Sup => sup().child(children).into_any(),
        ElementTag::Table => table().child(children).into_any(),
        ElementTag::Tbody => tbody().child(children).into_any(),
        ElementTag::Td => td().child(children).into_any(),
        ElementTag::Th => th().child(children).into_any(),
        ElementTag::Thead => thead().child(children).into_any(),
        ElementTag::Tr => tr().child(children).into_any(),
        ElementTag::Ul => ul().child(children).into_any(),
    }
}

/// Converts a `RenderIcon` enum value to its corresponding icon.
fn to_icon_data(icon: RenderIcon) -> &'static IconData {
    match icon {
        RenderIcon::Note => icondata::BsInfoCircleFill,
        RenderIcon::Tip => icondata::BsLightbulbFill,
        RenderIcon::Important => icondata::BsCheckCircleFill,
        RenderIcon::Warning => icondata::BsExclamationTriangleFill,
        RenderIcon::Caution => icondata::BsXOctagonFill,
    }
}

pub struct Renderer {
    prose: Vec<AnyView>,
    output: Vec<AnyView>,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            prose: vec![],
            output: vec![],
        }
    }

    fn move_prose(&mut self) {
        if self.prose.is_empty() {
            return;
        }

        let prose_view = view! {
            <article class="prose dark:prose-invert">
                {mem::take(&mut self.prose)}
            </article>
        };

        self.output.push(prose_view.into_any());
    }

    fn to_view(node: RenderNode) -> AnyView {
        match node {
            RenderNode::Text(text) => view! { {text.clone()} }.into_any(),
            RenderNode::Html(html) => view! {
                <div inner_html={html.0} />
            }
            .into_any(),
            RenderNode::Icon(icon) => view! {
                <Icon icon={to_icon_data(icon)} />
            }
            .into_any(),
            RenderNode::Element(RenderElement {
                tag,
                attributes,
                children,
            }) => {
                let children_view = children
                    .into_iter()
                    .map(Renderer::to_view)
                    .collect::<Vec<_>>();

                attributes
                    .into_iter()
                    .fold(create_view(tag, children_view), |acc, attr| {
                        acc.attr(attr.key.to_string(), attr.value).into_any()
                    })
            }
            RenderNode::Callout(container::callout::Callout { kind, children }) => {
                let CalloutData {
                    colour,
                    icon,
                    title,
                } = kind.into();
                let children = Renderer::new().run(children);

                view! {
                    <Callout colour icon title>
                        {children}
                    </Callout>
                }
                .into_any()
            }
        }
    }

    fn render_node(&mut self, node: RenderNode) {
        let should_unprose = matches!(&node, RenderNode::Callout(_));
        let render_html = Renderer::to_view(node);

        if should_unprose {
            self.move_prose();
            self.output.push(render_html);
        } else {
            self.prose.push(render_html);
        }
    }

    pub fn run(mut self, nodes: Vec<RenderNode>) -> Vec<AnyView> {
        for node in nodes {
            self.render_node(node);
        }

        self.move_prose();

        self.output
    }
}

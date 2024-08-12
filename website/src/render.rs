use markdown::translate::{
    element::RenderElement,
    node::{RenderCallout, RenderIcon, RenderNode},
};
use yew::{
    html,
    virtual_dom::{VTag, VText},
    Html,
};
use yew_icons::{Icon, IconId};

use crate::components::blog_post::callout::Callout;

fn to_icon_id(icon: RenderIcon) -> IconId {
    match icon {
        RenderIcon::Note => IconId::BootstrapInfoCircleFill,
        RenderIcon::Tip => IconId::BootstrapLightbulbFill,
        RenderIcon::Important => IconId::BootstrapCheckCircleFill,
        RenderIcon::Warning => IconId::BootstrapExclamationTriangleFill,
        RenderIcon::Caution => IconId::BootstrapXOctagonFill,
    }
}

pub struct Renderer {
    prose: Vec<Html>,
    output: Vec<Html>,
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

        let prose = html! {
            <article class="prose dark:prose-invert">
                {self.prose.clone()}
            </article>
        };

        self.output.push(prose);
        self.prose.clear();
    }

    fn to_html(node: RenderNode) -> Html {
        match node {
            RenderNode::Text(text) => VText::new(text.clone()).into(),
            RenderNode::Icon(icon) => html! {
                <Icon icon_id={to_icon_id(icon)} />
            },
            RenderNode::Element(RenderElement {
                tag,
                attributes,
                children,
            }) => {
                let mut tag = VTag::new(tag.to_string());
    
                for attribute in attributes {
                    tag.add_attribute(attribute.key.into(), attribute.value.clone());
                }
    
                for child in children {
                    tag.add_child(Renderer::to_html(child));
                }
    
                tag.into()
            },
            RenderNode::Callout(RenderCallout {
                kind,
                children,
            }) => {
                let props = kind.into();
                let children = Renderer::new().run(children);

                html! {
                    <Callout ..props>
                        {children}
                    </Callout>
                }
            },
        }
    }

    fn render_node(&mut self, node: RenderNode) {
        let should_unprose = matches!(&node, RenderNode::Callout(_));
        let render_html = Renderer::to_html(node);

        if should_unprose {
            self.move_prose();
            self.output.push(render_html);
        } else {
            self.prose.push(render_html);
        }
    }

    pub fn run(mut self, nodes: Vec<RenderNode>) -> Vec<Html> {
        for node in nodes {
            self.render_node(node);
        }

        self.move_prose();

        self.output
    }
}

use markdown::render::node::{RenderElement, RenderIcon, RenderNode};
use yew::{
    html, virtual_dom::{VTag, VText}, Html
};
use yew_icons::{Icon, IconId};

fn to_icon_id(icon: RenderIcon) -> IconId {
    match icon {
        RenderIcon::Note => IconId::BootstrapInfoCircleFill,
        RenderIcon::Tip => IconId::BootstrapLightbulbFill,
        RenderIcon::Important => IconId::BootstrapCheckCircleFill,
        RenderIcon::Warning => IconId::BootstrapExclamationTriangleFill,
        RenderIcon::Caution => IconId::BootstrapXOctagonFill,
    }
}

pub fn to_html(node: RenderNode) -> Html {
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
                tag.add_child(to_html(child));
            }

            tag.into()
        }
    }
}

use markdown::render::node::{RenderElement, RenderNode};
use yew::{
    virtual_dom::{VTag, VText},
    Html,
};

pub fn to_html(node: &RenderNode) -> Html {
    match node {
        RenderNode::Text(text) => VText::new(text.clone()).into(),
        RenderNode::Element(RenderElement {
            tag,
            attributes,
            children,
        }) => {
            let mut tag = VTag::new(tag.to_string());

            for attribute in attributes {
                tag.add_attribute(attribute.key.as_str(), attribute.value.clone());
            }

            for child in children {
                tag.add_child(to_html(&child));
            }

            tag.into()
        }
    }
}

use std::{cell::RefCell, rc::Rc};

use web_sys::HtmlHeadElement;
use yew::{
    create_portal, function_component, html, use_context, use_state, Children, Html, Properties,
};
use yew_hooks::use_effect_once;

#[derive(Properties, PartialEq)]
pub struct HeadProps {
    #[prop_or_default]
    pub children: Children,
}

/// Removes any tags that exist between `head-ssg-before` and `head-ssg-after`
/// defined in `index.html`, essentially acting as a manual hydration of
/// tags that need to be refreshed in `head` when we switch pages.
/// 
/// Only applies to server-side rendering - the client injects custom `head` tags
/// into the bottom anyway, not within the two `script` tags.
fn clean_head_ssg(head: &HtmlHeadElement) {
    let mut node = head.first_element_child();
    let mut in_ssg_section = false;

    // Iterate over nodes until we delete SSG elements in <head>
    while let Some(child) = node {
        node = child.next_element_sibling();

        let is_script = child.tag_name() == "SCRIPT";

        if is_script && child.id() == "head-ssg-before" {
            in_ssg_section = true;
            continue;
        }

        if in_ssg_section {
            if is_script && child.id() == "head-ssg-after" {
                break;
            }

            child.remove();
        }
    }
}

#[function_component(Head)]
pub fn head(props: &HeadProps) -> Html {
    let head = use_state(|| None::<HtmlHeadElement>);

    if let Some(head_ctx) = use_context::<HeadContext>() {
        let _ = head_ctx.content.replace(html! {
            <>{props.children.clone()}</>
        });
    }

    {
        let head = head.clone();
        use_effect_once(move || {
            let head_el = gloo::utils::head();

            // Cleans the `head` tag - only for SSR, should have no effect
            // on client
            clean_head_ssg(&head_el);
            // Sets the head to our actual head element
            head.set(Some(head_el));

            || ()
        })
    }

    // Client-side rendering - inject portal into location
    let portal = if let Some(head) = &*head {
        html! {
            create_portal(
                html! { <>{props.children.clone()}</> },
                head.clone().into()
            )
        }
    } else {
        html! {}
    };

    html! { <div>{portal}</div> }
}

#[derive(Default)]
pub struct HeadContext {
    content: Rc<RefCell<Html>>,
}

impl PartialEq for HeadContext {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl Clone for HeadContext {
    fn clone(&self) -> Self {
        Self {
            content: self.content.clone(),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct HeadRenderProps {
    pub context: HeadContext,
}

#[function_component(HeadRender)]
pub fn head_render(props: &HeadRenderProps) -> Html {
    props.context.content.borrow().clone()
}

use yew::{function_component, html, Children, Html, Properties};

use crate::components::header::Header;

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    html! {
        <>
            <Header />
            {props.children.clone()}
        </>
    }
}

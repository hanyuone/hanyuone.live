use yew::{function_component, html, Children, Html, Properties};

#[derive(Properties, PartialEq)]
struct CalloutProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Callout)]
fn callout(props: &CalloutProps) -> Html {
    html! {}
}

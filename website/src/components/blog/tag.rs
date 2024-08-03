use yew::{classes, function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct TagProps {
    pub name: String,
    pub colour: String,
}

#[function_component(Tag)]
pub fn tag(props: &TagProps) -> Html {
    html! {
        <div class={classes!("mr-0.5", format!("bg-{}", props.colour))}>
            {props.name.clone()}
        </div>
    }
}

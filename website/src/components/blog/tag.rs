use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct TagProps {
    pub name: String,
    pub colour: String,
}

#[function_component(Tag)]
pub fn tag(props: &TagProps) -> Html {
    html! {
        <div class={format!("rounded-sm mr-1 px-1 transition bg-{0}/50 hover:bg-{0}", props.colour)}>
            {props.name.clone()}
        </div>
    }
}

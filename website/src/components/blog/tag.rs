use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct TagProps {
    pub name: String,
}

#[function_component(Tag)]
pub fn tag(props: &TagProps) -> Html {
    html! {
        <div class="bg-secondary inline-block">
            {props.name.clone()}
        </div>
    }
}

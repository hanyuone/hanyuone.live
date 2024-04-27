use yew::{classes, function_component, html, Html};

#[function_component(Page)]
pub fn page() -> Html {
    html! {
        <h1 class={classes!("font-bold", "text-3xl", "text-center")}>{"Home"}</h1>
    }
}

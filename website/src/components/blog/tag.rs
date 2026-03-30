use leptos::prelude::*;
use leptos_router::components::A;
use markdown::structs::tag::TagId;

#[component]
pub fn Tag(name: String, colour: String) -> impl IntoView {
    let href = format!("tag/{}", name.parse::<TagId>().unwrap());

    view! {
        <A href=href.clone() {..} class="inline-flex">
            <div class={format!("rounded-sm mr-1 px-1 transition bg-{colour}/50 hover:bg-{colour}")}>
                {name.clone()}
            </div>
        </A>
    }
}

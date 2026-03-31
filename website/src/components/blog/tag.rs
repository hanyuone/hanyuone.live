use leptos::prelude::*;
use markdown::structs::tag::TagId;

#[component]
pub fn Tag(name: String, colour: String) -> impl IntoView {
    let href = format!("tag/{}", name.parse::<TagId>().unwrap());

    view! {
        // Cannot use Leptos's `<A>` here, since `Tag` can be in
        // multiple locations at once
        <a href=href.clone() class="inline-flex">
            <div class={format!("rounded-sm mr-1 px-1 transition bg-{colour}/50 hover:bg-{colour}")}>
                {name.clone()}
            </div>
        </a>
    }
}

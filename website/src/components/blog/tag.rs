use leptos::prelude::*;
use markdown::structs::tag::TagId;

use crate::ROOT;

#[component]
pub fn Tag(name: String, colour: String) -> impl IntoView {
    let root = ROOT.unwrap_or("");
    let href = format!("{root}/tag/{}", name.parse::<TagId>().unwrap());

    view! {
        <a href=href class="inline-flex">
            <div class={format!("rounded-sm mr-1 px-1 transition bg-{colour}/50 hover:bg-{colour}")}>
                {name.clone()}
            </div>
        </a>
    }
}

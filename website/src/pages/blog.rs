use leptos::prelude::*;

use crate::{components::blog::post_display::PostDisplay, context::BlogContext};

#[component]
pub fn BlogPage() -> impl IntoView {
    let context = use_context::<BlogContext>().unwrap();
    let pages = context.get_all();

    view! {
        {/* Have `PageDisplay` take *pre-rendered* individual pages instead */}
        <PostDisplay pages />
    }
}

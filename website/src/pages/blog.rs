use leptos::prelude::*;
use leptos_router::components::A;

use crate::app::list_slugs;

#[component]
pub fn BlogPage() -> impl IntoView {
    // load the posts
    let posts = Resource::new(|| (), |_| list_slugs());
    let posts = move || {
        posts
            .get()
            .map(|n| n.unwrap_or_default())
            .unwrap_or_default()
    };

    view! {
        <h1>"My Great Blog"</h1>
        <Suspense fallback=move || view! { <p>"Loading posts..."</p> }>
            <ul>
                <For each=posts key=|post| post.clone() let:post>
                    <li>
                        <A href={post.clone()}>{post.clone()}</A>
                    </li>
                </For>
            </ul>
        </Suspense>
    }
}

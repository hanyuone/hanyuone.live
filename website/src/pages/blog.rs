use leptos::prelude::*;

use crate::{
    components::blog::{card::BlogCard, item::BlogItem},
    context::BlogContext,
};

#[component]
pub fn BlogPage() -> impl IntoView {
    let context = use_context::<BlogContext>().unwrap();
    let sorted = context.get_sorted();

    if sorted.is_empty() {
        view! { <p>"No blogs found!"</p> }.into_any()
    } else {
        let mut sorted_iter = sorted.into_iter();
        let (first_id, first_metadata) = sorted_iter.next().unwrap();

        view! {
            <BlogCard
                id={*first_id}
                metadata={first_metadata.clone()} />
            <div>
                {
                    sorted_iter
                        .map(|(id, metadata)| view! {
                            <BlogItem
                                id={*id}
                                metadata={metadata.clone()} />
                        })
                        .collect::<Vec<_>>()
                }
            </div>
        }
        .into_any()
    }
}

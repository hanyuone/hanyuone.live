use leptos::prelude::*;
use leptos_icons::Icon;
use markdown::structs::{blog::BlogId, metadata::BlogMetadata};

use crate::components::blog::{card::BlogCard, item::BlogItem};

#[component]
pub fn PostList(posts: Vec<(BlogId, BlogMetadata)>) -> impl IntoView {
    let (first_id, first_metadata) = &posts[0];
    let rest = &posts[1..];

    view! {
        <BlogCard
            id={*first_id}
            metadata={first_metadata} />
        <div>
            {
                rest
                    .into_iter()
                    .map(|(id, metadata)| view! {
                        <BlogItem
                            id={*id}
                            metadata={metadata} />
                    })
                    .collect::<Vec<_>>()
            }
        </div>
    }
}

#[island]
pub fn PostDisplay(pages: Vec<Vec<(BlogId, BlogMetadata)>>) -> impl IntoView {
    let n_pages = pages.len();

    // `page` is always between a valid index of `page_chunks`
    let (page_index, set_page_index) = signal::<usize>(0);

    let decrement = move |_ev| {
        set_page_index.update(|page_index| {
            if *page_index > 0 {
                *page_index -= 1;
            }
        })
    };

    let increment = move |_ev| {
        set_page_index.update(|page_index| {
            if *page_index < n_pages - 1 {
                *page_index += 1;
            }
        })
    };

    view! {
        <div class="flex flex-col">
            {
                let pages = pages.clone();

                move || {
                    let post_list = pages[page_index.get()]
                        .iter()
                        .map(|(id, metadata)| (*id, metadata.clone()))
                        .collect::<Vec<_>>();

                    view! {
                        <PostList posts={post_list} />
                    }
                }
            }
            <div class="flex flex-row justify-center">
                <button class="px-2 border-[1px] border-solid rounded-md hover:bg-gray transition-colors" on:click=decrement>
                    <Icon icon={icondata::BsArrowLeft} />
                </button>
                <p class="p-2">{move || format!("Page {} / {n_pages}", page_index.get() + 1)}</p>
                <button class="px-2 border-[1px] border-solid rounded-md hover:bg-gray transition-colors" on:click=increment>
                    <Icon icon={icondata::BsArrowRight} />
                </button>
            </div>
        </div>
    }
}

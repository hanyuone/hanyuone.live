use leptos::{ev::Targeted, prelude::*};
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
                    .iter()
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

    let page_input_change = move |ev: Targeted<web_sys::Event, web_sys::HtmlInputElement>| {
        let value = ev.target().value();
        let parsed_page = value.parse::<usize>().unwrap_or(1);

        set_page_index.update(|page_index| {
            *page_index = if parsed_page == 0 {
                0
            } else if parsed_page > n_pages {
                n_pages - 1
            } else {
                parsed_page - 1
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
                <p class="p-2">"Page "</p>
                <input
                    class="[appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"
                    type="number"
                    inputmode="numeric"
                    size={n_pages.checked_ilog10().unwrap() + 1}
                    on:change:target=page_input_change
                    prop:value={move || page_index.get() + 1} />
                <p class="p-2">{format!(" / {n_pages}")}</p>
                <button class="px-2 border-[1px] border-solid rounded-md hover:bg-gray transition-colors" on:click=increment>
                    <Icon icon={icondata::BsArrowRight} />
                </button>
            </div>
        </div>
    }
}

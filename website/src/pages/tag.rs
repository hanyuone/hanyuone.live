use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::{hooks::use_params, params::Params};
use markdown::structs::tag::{TagId, TagMetadata};

use crate::{
    components::blog::{card::BlogCard, item::BlogItem},
    context::BlogContext,
};

#[derive(Params, PartialEq)]
pub struct TagParams {
    // TODO: convert to Option<TagId>
    tag_id: Option<String>,
}

#[component]
pub fn TagPage() -> impl IntoView {
    let params = use_params::<TagParams>();
    let tag_id = params
        .read()
        .as_ref()
        .ok()
        .and_then(|params| params.tag_id.clone().map(|id| id.parse::<TagId>().unwrap()))
        .unwrap();
    let tag_metadata: TagMetadata = tag_id.clone().into();

    let blog_context = use_context::<BlogContext>().unwrap();
    let sorted_blogs = blog_context.get_sorted();
    let mut tag_blogs = sorted_blogs
        .into_iter()
        .filter(|(_, metadata)| metadata.front_matter.tags.contains(&tag_id.to_string()));
    let first_blog = tag_blogs.next();

    view! {
        <Title text={tag_id.to_string()} />
        <div class="flex flex-col">
            <a>
                <div class="flex">
                    <div class={format!("grow-0 rounded-sm px-2 transition bg-{0}/50 hover:bg-{0}", tag_metadata.colour)}>
                        <h2 class="font-bold text-2xl">{tag_id.to_string()}</h2>
                    </div>
                </div>
            </a>
            <div>{tag_metadata.description}</div>
        </div>
        {
            if let Some((first_id, first_metadata)) = first_blog {
                view! {
                    <BlogCard
                        id={*first_id}
                        metadata={first_metadata.clone()} />
                    <div>
                        {
                            tag_blogs
                                .map(|(id, metadata)| {
                                    view! {
                                        <BlogItem
                                            id={*id}
                                            metadata={metadata.clone()} />
                                    }
                                })
                                .collect::<Vec<_>>()
                        }
                    </div>
                }.into_any()
            } else {
                view! { <p>{"No blogs found!"}</p> }.into_any()
            }
        }
    }
}

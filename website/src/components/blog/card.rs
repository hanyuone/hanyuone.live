use leptos::prelude::*;
use leptos_router::components::A;
use markdown::structs::{
    blog::BlogId,
    metadata::BlogMetadata,
    tag::{TagId, TagMetadata},
};

use crate::components::blog::{tag::Tag, to_read_time};

#[component]
pub fn blog_card(id: BlogId, metadata: BlogMetadata) -> impl IntoView {
    let BlogMetadata {
        front_matter,
        post_translate,
    } = metadata;

    view! {
        <div class="flex flex-col md:flex-row hover:bg-gray">
            <div class="flex-col w-full md:basis-1/4 p-4">
                <img
                    src={front_matter.image.clone()}
                    class="w-full aspect-video object-cover" />
            </div>
            <div class="flex flex-col md:basis-3/4 p-4">
                <A href=id.to_string()>
                    <h2 class="font-bold text-2xl hover:underline">{front_matter.title.clone()}</h2>
                </A>
                <p class="flex grow">{front_matter.description.clone()}</p>
                <div class="inline">
                    <span class="text-gray-500">{front_matter.publish_date.format("%d %b %Y").to_string()}</span>
                    <span class="px-1 text-white">{"·"}</span>
                    <span class="text-gray-500">{to_read_time(post_translate.words)}</span>
                    <span class="px-1 text-white">{"·"}</span>
                    {
                        front_matter.tags
                            .iter()
                            .map(|tag_name| {
                                let colour = tag_name.parse::<TagId>()
                                    .map(|tag_id| {
                                        let TagMetadata { colour, .. } = tag_id.into();
                                        colour
                                    })
                                    .unwrap_or("green".to_string());

                                view! {
                                    <Tag
                                        name={tag_name.clone()}
                                        colour={colour} />
                                }
                            })
                            .collect::<Vec<_>>()
                    }
                </div>
            </div>
        </div>
    }
}

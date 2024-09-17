use markdown::structs::{
    blog::BlogId,
    metadata::BlogMetadata,
    tag::{TagId, TagMetadata},
};
use yew::{function_component, html, html_nested, Html, Properties};
use yew_router::components::Link;

use crate::{
    components::blog::{tag::Tag, to_read_time},
    pages::Route,
};

#[derive(Properties, PartialEq)]
pub struct BlogCardProps {
    pub id: BlogId,
    pub metadata: BlogMetadata,
}

#[function_component(BlogCard)]
pub fn blog_card(props: &BlogCardProps) -> Html {
    let BlogMetadata {
        front_matter,
        post_translate,
    } = &props.metadata;

    html! {
        <div class="flex flex-row hover:bg-gray">
            <div class="flex-col basis-1/4 p-4">
                <img
                    src={front_matter.image.clone()}
                    class="aspect-video object-cover" />
            </div>
            <div class="flex flex-col basis-3/4 p-4">
                <Link<Route> to={Route::BlogPost { blog_id: props.id }}>
                    <h2 class="font-bold text-2xl hover:underline">{&front_matter.title}</h2>
                </Link<Route>>
                <p class="flex grow">{&front_matter.description}</p>
                <div class="flex flex-row">
                    <span class="text-gray-500">{&front_matter.publish_date.format("%d %b %Y").to_string()}</span>
                    <span class="px-0.5 text-white">{"·"}</span>
                    <span class="text-gray-500">{&to_read_time(post_translate.words)}</span>
                    <span class="px-0.5 text-white">{"·"}</span>
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

                                html_nested! {
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

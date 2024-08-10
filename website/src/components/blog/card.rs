use markdown::structs::{blog::BlogId, metadata::BlogMetadata};
use yew::{function_component, html, html_nested, use_context, Html, Properties};
use yew_router::components::Link;

use crate::{
    components::blog::{tag::Tag, to_read_time},
    context::TagContext,
    pages::Route,
};

#[derive(Properties, PartialEq)]
pub struct BlogCardProps {
    pub id: BlogId,
    pub metadata: BlogMetadata,
}

#[function_component(BlogCard)]
pub fn blog_card(props: &BlogCardProps) -> Html {
    let tag_context = use_context::<TagContext>().unwrap();

    let BlogMetadata {
        front_matter,
        post_translate,
    } = &props.metadata;

    html! {
        <Link<Route> to={Route::BlogPost { blog_id: props.id }}>
            <div class="flex flex-row hover:bg-gray">
                <div class="flex-col basis-1/4 p-4">
                    <img
                        src={front_matter.image.clone()}
                        class="aspect-video object-cover" />
                </div>
                <div class="flex flex-col basis-3/4 p-4">
                    <h2 class="font-bold text-2xl">{&front_matter.title}</h2>
                    <p class="flex grow">{&front_matter.description}</p>
                    <div class="flex flex-row">
                        <span class="text-gray-500">{&front_matter.publish_date}</span>
                        <span class="px-0.5 text-white">{"·"}</span>
                        <span class="text-gray-500">{&to_read_time(post_translate.words)}</span>
                        <span class="px-0.5 text-white">{"·"}</span>
                        {
                            front_matter.tags
                                .iter()
                                .map(|tag_name| html_nested! {
                                    <Tag
                                        name={tag_name.clone()}
                                        colour={tag_context.get(tag_name).unwrap_or("green".to_string())} />
                                })
                                .collect::<Vec<_>>()
                        }
                    </div>
                </div>
            </div>
        </Link<Route>>
    }
}

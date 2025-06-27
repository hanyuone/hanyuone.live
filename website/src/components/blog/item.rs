use std::str::FromStr;

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

#[derive(PartialEq, Properties)]
pub struct BlogItemProps {
    pub id: BlogId,
    pub metadata: BlogMetadata,
}

#[function_component(BlogItem)]
pub fn blog_item(props: &BlogItemProps) -> Html {
    let BlogMetadata {
        front_matter,
        post_translate,
    } = &props.metadata;

    html! {
        <div class="flex-col p-4 border-t-[1px] border-white hover:bg-gray">
            <Link<Route> to={Route::BlogPost { blog_id: props.id }}>
                <h3 class="font-bold text-xl hover:underline">{&front_matter.title}</h3>
            </Link<Route>>
            <div class="inline">
                <span class="text-gray-500">{&front_matter.publish_date.format("%d %b %Y").to_string()}</span>
                <span class="px-1 text-white">{"·"}</span>
                <span class="text-gray-500">{&to_read_time(post_translate.words)}</span>
                <span class="px-1 text-white">{"·"}</span>
                {
                    front_matter.tags
                        .iter()
                        .map(|tag_name| {
                            let colour = TagId::from_str(tag_name)
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
    }
}

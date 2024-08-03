use markdown::structs::{blog::BlogId, metadata::BlogMetadata};
use yew::{function_component, html, html_nested, use_context, Html, Properties};
use yew_router::components::Link;

use crate::{
    components::blog::{tag::Tag, to_read_time},
    context::TagContext,
    pages::Route,
};

#[derive(PartialEq, Properties)]
pub struct BlogItemProps {
    pub id: BlogId,
    pub metadata: BlogMetadata,
}

#[function_component(BlogItem)]
pub fn blog_item(props: &BlogItemProps) -> Html {
    let tag_context = use_context::<TagContext>().unwrap();

    let BlogMetadata {
        front_matter,
        post_translate,
    } = &props.metadata;

    html! {
        <Link<Route> to={Route::BlogPost { blog_id: props.id }}>
            <div class="flex-col border-t-[1px] border-white hover:bg-gray">
                <h3 class="font-bold text-xl">{&front_matter.title}</h3>
                <div class="flex flex-row">
                    <span class="text-gray-500">{&front_matter.publish_date.format("%d %b %Y").to_string()}</span>
                    <span class="px-0.5 text-white">{" · "}</span>
                    <span class="text-gray-500">{&to_read_time(post_translate.words)}</span>
                    <span class="px-0.5 text-white">{" · "}</span>
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
        </Link<Route>>
    }
}

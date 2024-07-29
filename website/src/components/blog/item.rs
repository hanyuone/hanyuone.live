use markdown::structs::{blog::BlogId, metadata::BlogMetadata};
use yew::{function_component, html, html_nested, Html, Properties};
use yew_router::components::Link;

use crate::{components::blog::tag::Tag, pages::Route};

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
        <Link<Route> to={Route::BlogPost { blog_id: props.id }}>
            <div class="flex-col border-t-[1px] border-white hover:bg-gray">
                <h3 class="font-bold text-xl">{&front_matter.title}</h3>
                <div>
                    <span class="text-gray-500">{&front_matter.publish_date.format("%d %b %Y").to_string()}</span>
                    <span class="text-white">{" · "}</span>
                    <span class="text-gray-500">{&post_translate.read_time}</span>
                    <span class="text-white">{" · "}</span>
                    {
                        front_matter.tags.iter()
                            .map(|tag_name| html_nested! {
                                <Tag
                                    name={tag_name.clone()} />
                            })
                            .collect::<Vec<_>>()
                    }
                </div>
            </div>
        </Link<Route>>
    }
}

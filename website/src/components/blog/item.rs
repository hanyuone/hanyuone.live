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
    let front_matter = &props.metadata.front_matter;

    html! {
        <Link<Route> to={Route::BlogPost { blog_id: props.id }}>
            <div class="flex-col border-t-[1px] border-white hover:bg-black-light">
                <h3>{&front_matter.title}</h3>
                <div>
                    <span class="text-gray-500">{&front_matter.publish_date.format("%d %b %Y").to_string()}</span>
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

use chrono::TimeDelta;
use markdown::structs::{blog::BlogId, metadata::BlogMetadata};
use yew::{function_component, html, html_nested, Html, Properties};
use yew_router::components::Link;

use crate::{components::blog::tag::Tag, pages::Route};

#[derive(Properties, PartialEq)]
pub struct BlogCardProps {
    pub id: BlogId,
    pub metadata: BlogMetadata,
}

fn to_readable(time_delta: &TimeDelta) -> String {
    let seconds = time_delta.num_seconds();

    if seconds < 60 {
        return "<1 min".to_string();
    }

    let minutes = time_delta.num_minutes();

    if minutes < 60 {
        return format!("{} min", minutes);
    }

    "long read".to_string()
}

#[function_component(BlogCard)]
pub fn blog_card(props: &BlogCardProps) -> Html {
    let BlogMetadata {
        front_matter,
        post_render,
    } = &props.metadata;

    html! {
        <Link<Route> to={Route::BlogPost { blog_id: props.id }}>
            <div class="flex flex-row hover:bg-black-light">
                <div class="flex-col basis-1/4 p-4">
                    <img
                        src={front_matter.image.clone()}
                        class="aspect-video object-cover" />
                </div>
                <div class="flex flex-col basis-3/4 p-4">
                    <h2 class="text-xl">{&front_matter.title}</h2>
                    <div>
                        <span class="text-gray-500">{&front_matter.publish_date.format("%d %b %Y").to_string()}</span>
                        <span class="text-white">{" · "}</span>
                        <span class="text-gray-500">{to_readable(&post_render.read_time)}</span>
                        <span class="text-white">{" · "}</span>
                        {
                            front_matter.tags
                                .iter()
                                .map(|tag_name| html_nested! {
                                    <Tag
                                        name={tag_name.clone()} />
                                })
                                .collect::<Vec<_>>()
                        }
                    </div>
                </div>
            </div>
        </Link<Route>>
    }
}

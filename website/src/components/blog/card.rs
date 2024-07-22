use markdown::structs::{blog::BlogId, metadata::BlogMetadata};
use yew::{function_component, html, html_nested, Html, Properties};
use yew_router::components::Link;

use crate::{components::blog::tag::Tag, pages::Route};

#[derive(Properties, PartialEq)]
pub struct BlogCardProps {
    pub id: BlogId,
    pub metadata: BlogMetadata,
}

#[function_component(BlogCard)]
pub fn blog_card(props: &BlogCardProps) -> Html {
    let front_matter = &props.metadata.front_matter;

    html! {
        <Link<Route> to={Route::BlogPost { blog_id: props.id }}>
            <div class="flex-row hover:bg-black-light">
                <img />
                <div class="flex-col">
                    <h2 class="text-xl">{&front_matter.title}</h2>
                    <div>
                        <span class="text-gray-500">
                            {
                                format!(
                                    "{} · ",
                                    &front_matter.publish_date.format("%d %b %Y").to_string(),
                                )
                            }
                        </span>
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

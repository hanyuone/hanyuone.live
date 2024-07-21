use markdown::structs::{blog::BlogId, metadata::BlogMetadata};
use yew::{function_component, html, html_nested, Html, Properties};
use yew_router::components::Link;

use crate::{components::blog::tag::Tag, pages::Route};

#[derive(Properties, PartialEq)]
pub struct CardProps {
    pub id: BlogId,
    pub metadata: BlogMetadata,
}

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    let front_matter = &props.metadata.front_matter;

    html! {
        <div class="rounded-sm bg-blue">
            <Link<Route> to={Route::BlogPost { blog_id: props.id }}>
                {front_matter.title.clone()}
            </Link<Route>>
            <br />
            <div>
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
    }
}

use markdown::{blog::BlogId, front_matter::FrontMatter};
use yew::{function_component, html, html_nested, Html, Properties};
use yew_router::components::Link;

use crate::{components::blog::tag::Tag, pages::Route};

#[derive(Properties, PartialEq)]
pub struct CardProps {
    pub id: BlogId,
    pub front_matter: FrontMatter,
}

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    html! {
        <div class="rounded-sm bg-primary-light">
            <Link<Route> to={Route::BlogPost { blog_id: props.id }}>
                {props.front_matter.title.clone()}
            </Link<Route>>
            <br />
            <div>
            {
                props.front_matter.tags
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

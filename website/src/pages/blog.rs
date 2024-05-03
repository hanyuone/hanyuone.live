use yew::{function_component, html, html_nested, use_context, Html};
use yew_router::components::Link;

use crate::{context::BlogContext, pages::Route};

#[function_component(Page)]
pub fn page() -> Html {
    let blog_context = use_context::<BlogContext>().unwrap();

    html! {
        <ul>
        {
            blog_context.content.into_iter()
                .map(|card| html_nested! {
                    <li>
                        <Link<Route> to={Route::BlogPost { blog_id: card.id }}>
                            {card.front_matter.title}
                        </Link<Route>>
                    </li>
                })
                .collect::<Vec<_>>()
        }
        </ul>
    }
}

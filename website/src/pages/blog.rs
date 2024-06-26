use yew::{function_component, html, html_nested, use_context, Html};

use crate::{components::blog::card::Card, context::BlogContext};

#[function_component(Page)]
pub fn page() -> Html {
    let blog_context = use_context::<BlogContext>().unwrap();

    html! {
        <div class="m-4 grid grid-cols-3 gap-4">
        {
            blog_context.content.into_iter()
                .map(|card| html_nested! {
                    <Card
                        id={card.id}
                        front_matter={card.front_matter} />
                })
                .collect::<Vec<_>>()
        }
        </div>
    }
}

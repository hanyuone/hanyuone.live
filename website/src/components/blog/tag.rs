use markdown::structs::tag::TagId;
use yew::{classes, function_component, html, Html, Properties};
use yew_router::prelude::Link;

use crate::pages::Route;

#[derive(Properties, PartialEq)]
pub struct TagProps {
    pub name: String,
    pub colour: String,
}

#[function_component(Tag)]
pub fn tag(props: &TagProps) -> Html {
    html! {
        <Link<Route> to={Route::Tag { tag_id: props.name.parse::<TagId>().unwrap() }} classes={classes!("inline-flex")}>
            <div class={format!("rounded-sm mr-1 px-1 transition bg-{0}/50 hover:bg-{0}", props.colour)}>
                {props.name.clone()}
            </div>
        </Link<Route>>
    }
}

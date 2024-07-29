use yew::{classes, function_component, html, Children, Html, Properties};
use yew_icons::{Icon, IconId};

#[derive(Properties, PartialEq)]
struct CalloutProps {
    colour: String,
    icon: IconId,
    title: String,
    #[prop_or_default]
    children: Children,
}

#[function_component(Callout)]
fn callout(props: &CalloutProps) -> Html {
    html! {
        <div class={props.colour.clone()}>
            <div class="flex flex-row">
                <Icon icon_id={props.icon} />
                <p class="font-bold">{props.title.clone()}</p>
            </div>
            <article class="prose dark:prose-invert">
                {props.children.clone()}
            </article>
        </div>
    }
}

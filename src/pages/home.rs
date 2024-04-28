use yew::{classes, function_component, html, Html};

use crate::components::head::Head;

#[function_component(Page)]
pub fn page() -> Html {
    html! {
        <>
            <Head>
                <title>{"Hanyuan's Website"}</title>
            </Head>
            <h1 class={classes!("font-bold", "text-3xl", "text-center")}>{"Home"}</h1>
        </>
    }
}

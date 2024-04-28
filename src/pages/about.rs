use yew::{function_component, html, Html};

use crate::components::head::Head;

#[function_component(Page)]
pub fn page() -> Html {
    html! {
        <>
            <Head>
                <title>{"About | Hanyuan's Website"}</title>
            </Head>
            <h1>{"About"}</h1>
        </>
    }
}

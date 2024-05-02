use std::path::PathBuf;

use gloo_net::http::Request;
use markdown::blog_id::BlogId;
use yew::{function_component, html, use_state, Html, Properties};
use yew_hooks::use_effect_once;

use crate::components::head::Head;

#[derive(PartialEq, Properties)]
pub struct BlogProps {
    pub blog_id: BlogId,
}

#[function_component(Page)]
pub fn page(props: &BlogProps) -> Html {
    let contents = use_state(String::new);

    {
        let contents = contents.clone();
        let url = format!("/public/blog/{}.md", props.blog_id);

        use_effect_once(|| {
            wasm_bindgen_futures::spawn_local(async move {
                // We can safely unwrap here, because we're guaranteed that the
                // file exists when we build our MD files in the first place
                let raw_contents = Request::get(&url)
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();

                contents.set(raw_contents);
            });

            || ()
        });
    }

    html! {
        <>
            <Head>
                <title>{"Blog | Hanyuan's Website"}</title>
            </Head>
            <code>{(*contents).clone()}</code>
        </>
    }
}

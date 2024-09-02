use gloo_net::http::Request;
use markdown::{structs::blog::BlogId, translate::node::RenderNode};
use yew::{function_component, html, use_context, use_state, Html, Properties, UseStateHandle};
use yew_hooks::use_effect_once;

use crate::{components::head::Head, context::BlogContext, render::Renderer};

#[derive(PartialEq, Properties)]
pub struct BlogPostProps {
    pub blog_id: BlogId,
}

#[function_component(Page)]
pub fn page(props: &BlogPostProps) -> Html {
    let blog_context = use_context::<BlogContext>().unwrap();
    let content: UseStateHandle<String> = use_state(String::new);

    {
        let content = content.clone();
        let url = format!("/public/blog/{}.ron", props.blog_id);

        use_effect_once(|| {
            wasm_bindgen_futures::spawn_local(async move {
                // We can safely unwrap here, because we're guaranteed that the
                // file exists when we build our MD files in the first place
                let raw_content = Request::get(&url)
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();

                content.set(raw_content);
            });

            || ()
        });
    }

    let title = &blog_context.content[&props.blog_id].front_matter.title;
    let nodes = ron::from_str::<Vec<RenderNode>>(&content).unwrap_or_default();

    html! {
        <>
            <Head>
                <title>{format!("{} | Hanyuan's Website", title)}</title>
            </Head>
            {Renderer::new().run(nodes)}
        </>
    }
}

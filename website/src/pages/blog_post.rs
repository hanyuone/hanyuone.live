use gloo_net::http::Request;
use markdown::{render::node::RenderNode, structs::blog::BlogId};
use yew::{function_component, html, use_context, use_state, Html, Properties, UseStateHandle};
use yew_hooks::use_effect_once;

use crate::{components::head::Head, context::BlogContext, render::to_html};

#[derive(PartialEq, Properties)]
pub struct BlogProps {
    pub blog_id: BlogId,
}

#[function_component(Page)]
pub fn page(props: &BlogProps) -> Html {
    let blog_context = use_context::<BlogContext>().unwrap();
    let content: UseStateHandle<Vec<u8>> = use_state(Vec::new);

    {
        let content = content.clone();
        let url = format!("/public/blog/{}", props.blog_id);

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

                content.set(raw_content.as_bytes().to_vec());
            });

            || ()
        });
    }

    let title = &blog_context.content[&props.blog_id].front_matter.title;
    let nodes = postcard::from_bytes::<Vec<RenderNode>>(&content).unwrap_or_default();

    html! {
        <>
            <Head>
                <title>{format!("{} | Hanyuan's Website", title)}</title>
            </Head>
            <article class="prose dark:prose-invert prose-callouts">
                {
                    nodes.into_iter()
                        .map(|node| {
                            to_html(node)
                        })
                        .collect::<Vec<_>>()
                }
            </article>
        </>
    }
}

use leptos::prelude::*;
use leptos_router::{hooks::use_params, params::Params};
use markdown::translate::node::RenderNode;

#[derive(Params, PartialEq)]
pub struct BlogPostParams {
    slug: Option<String>,
}

#[component]
pub fn BlogPostPage() -> impl IntoView {
    let params = use_params::<BlogPostParams>();
    let slug = move || {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.slug.clone())
            .unwrap_or_default()
    };

    let post_contents = Resource::new(slug, get_blog_post_ron);

    view! {
        <h4>{slug}</h4>
        <Suspense fallback=move || view! { <p>"Loading post..."</p> }>
            {move || Suspend::new(async move {
                let renderer = crate::renderer::Renderer::new();

                let post_contents = post_contents.await;
                let post_ron = ron::from_str::<Vec<RenderNode>>(&post_contents.unwrap())
                    .unwrap();

                view! {
                    <div>
                        {renderer.run(post_ron)}
                    </div>
                }
            })}
        </Suspense>
    }
}

#[server]
async fn get_blog_post_ron(slug: String) -> Result<String, ServerFnError> {
    use tokio::fs;

    let raw_post_ron = fs::read(&format!("./blogs/parsed/{slug}.ron")).await?;
    let post_ron_str = String::from_utf8(raw_post_ron)?;
    Ok(post_ron_str)
}

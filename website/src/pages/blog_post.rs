use leptos::prelude::*;
use leptos_router::{hooks::use_params, params::Params};

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

    let slug_contents = Resource::new(slug, get_blog_post_ron);

    view! {
        <h4>{slug}</h4>
        <Suspense fallback=move || view! { <p>"Loading post..."</p> }>
            <article class="prose dark:prose-invert">
                <p>{slug_contents}</p>
            </article>
        </Suspense>
    }
}

#[server]
async fn get_blog_post_ron(slug: String) -> Result<String, ServerFnError> {
    use tokio::fs;

    let raw_post_ron = fs::read(&format!("./blogs/parsed/{slug}.ron")).await?;
    let post_ron = String::from_utf8(raw_post_ron)?;
    Ok(post_ron)
}

use chrono::TimeDelta;
use leptos::prelude::*;
use leptos_router::{hooks::use_params, params::Params};
use markdown::{structs::metadata::BlogMetadata, translate::node::RenderNode};

#[derive(Params, PartialEq)]
pub struct BlogPostParams {
    slug: Option<String>,
}

pub fn to_read_time(words: usize) -> String {
    let time_delta = TimeDelta::seconds((words / 3) as i64);
    let seconds = time_delta.num_seconds();

    if seconds < 60 {
        return "<1 min".to_string();
    }

    let minutes = time_delta.num_minutes();

    if minutes < 60 {
        return format!("{} min", minutes);
    }

    "long read".to_string()
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

    let metadata = Resource::new(slug, get_metadata);
    let post_contents = Resource::new(slug, get_blog_post_ron);

    view! {
        <Suspense fallback=move || view! { <p>"Loading metadata..."</p> }>
            {move || Suspend::new(async move {
                let BlogMetadata { front_matter, post_translate } = metadata.await.unwrap();

                view! {
                    <div class="flex flex-col p-4 content-center text-center border-b-[1px]">
                        <h2 class="font-bold text-2xl underline">{front_matter.title}</h2>
                        <p>
                            <span class="text-gray-500">{front_matter.publish_date.format("%d %b %Y").to_string()}</span>
                            <span class="px-1 text-white">{"·"}</span>
                            <span class="text-gray-500">{to_read_time(post_translate.words)}</span>
                        </p>
                    </div>
                }
            })}
        </Suspense>
        <Suspense fallback=move || view! { <p>"Loading post..."</p> }>
            {move || Suspend::new(async move {
                let renderer = crate::renderer::Renderer::new();

                let post_contents = post_contents.await;
                let post_ron = ron::from_str::<Vec<RenderNode>>(&post_contents.unwrap())
                    .unwrap_or_default();

                view! {
                    <div class="flex flex-col py-4 items-center">
                        {renderer.run(post_ron)}
                    </div>
                }
            })}
        </Suspense>
    }
}

#[server]
async fn get_metadata(slug: String) -> Result<BlogMetadata, ServerFnError> {
    use std::{collections::HashMap, str::FromStr};

    use markdown::structs::blog::BlogId;
    use tokio::fs;

    let raw_blog_map = fs::read("./blogs/blog_map.ron").await?;
    let blog_map =
        ron::from_str::<HashMap<BlogId, BlogMetadata>>(&String::from_utf8(raw_blog_map).unwrap())?;

    let slug_id = BlogId::from_str(&slug).map_err(|e| ServerFnError::new(e))?;

    Ok(blog_map.get(&slug_id).unwrap().clone())
}

#[server]
async fn get_blog_post_ron(slug: String) -> Result<String, ServerFnError> {
    use tokio::fs;

    let raw_post_ron = fs::read(&format!("./blogs/parsed/{slug}.ron")).await?;
    let post_ron_str = String::from_utf8(raw_post_ron)?;

    Ok(post_ron_str)
}

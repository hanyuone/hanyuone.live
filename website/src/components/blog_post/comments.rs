use icondata as i;
use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_use::use_document;
use serde::{Deserialize, Serialize};

use crate::{pages::blog_post::BlogSlug, COMMENTS_URL};

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    id: i32,
    username: String,
    avatar_url: String,
}

#[island]
fn OAuth() -> impl IntoView {
    let document = use_document();

    let login_url = match document.url() {
        Some(url) => Some(format!(
            "{COMMENTS_URL}/auth/login?return_url={}",
            url.unwrap()
        )),
        None => None,
    };

    view! {
        <div class="flex flex-col content-center">
            <a class="bg-white text-black p-2 rounded" href={login_url}>
                <div class="flex flex-row justify-center content-center">
                    <div class="p-1">
                        <Icon icon={i::BsGithub} height="1.1em" />
                    </div>
                    <p>"Login using GitHub"</p>
                </div>
            </a>
        </div>
    }
}

#[derive(Clone)]
struct CommentData {
    id: String,
    contents: String,
}

#[component]
fn Comment(#[prop(into)] data: CommentData) -> impl IntoView {
    view! {
        <p>{data.contents}</p>
    }
}

async fn get_comments(slug: Option<BlogSlug>) -> Vec<CommentData> {
    vec![]
}

#[island]
fn CommentList() -> impl IntoView {
    let slug = use_context::<BlogSlug>();
    let comments = LocalResource::new(move || get_comments(slug.clone()));

    view! {
        <div>
            <Suspense fallback=move || view! { <p>"Loading comments..."</p> }>
                {move || Suspend::new(async move {
                    let comments = comments.await;

                    if comments.is_empty() {
                        view! { <p>"No comments"</p> }.into_any()
                    } else {
                        view! {
                            <For
                                each=move || comments.clone()
                                key=|comment| comment.id.clone()
                                let(data)>
                                <Comment data />
                            </For>
                        }.into_any()
                    }
                })}
            </Suspense>
        </div>
    }
}

#[component]
fn CommentWidget(#[prop(into)] user: User) -> impl IntoView {
    view! {
        <div class="flex flex-col">
            // Posting comments
            <form>
                <div class="flex flex-col">
                    // Text input for posting comment
                    <textarea
                        name="comment"
                        placeholder="Enter your thoughts here..."
                        class="bg-white text-black text-start rounded p-2 h-24" />
                    // User avatar, post comment button
                    <div class="flex flex-row mt-2">
                        <div class="flex flex-row items-center">
                            <img src={user.avatar_url} class="p-1 h-10" />
                            <p class="p-1">{user.username}</p>
                            <div class="p-1">
                                <Icon icon={i::BsGithub} height="1.1em" />
                            </div>
                        </div>
                        <input
                            type="submit"
                            value="Post"
                            class="ml-auto px-4 py-1 rounded transition bg-blue/50 hover:bg-blue" />
                    </div>
                </div>
            </form>
            // List of existing comments
            <CommentList />
        </div>
    }
}

async fn get_profile() -> Result<User, Error> {
    #[cfg(target_arch = "wasm32")]
    {
        use reqwest::Client;

        let url = format!("{COMMENTS_URL}/profile");

        let client = Client::new();
        let user = client
            .get(url)
            .fetch_credentials_include()
            .send()
            .await?
            .json::<User>()
            .await?;

        Ok(user)
    }

    #[cfg(not(target_arch = "wasm32"))]
    Err("Only supported on WASM mode".into())
}

#[island]
pub fn CommentToggle() -> impl IntoView {
    let auth_state = LocalResource::new(get_profile);

    view! {
        <Suspense fallback=move || view! { <p>"Loading..."</p> }>
            {move || Suspend::new(async move {
                let auth_state = auth_state.await;

                match auth_state {
                    Ok(ref user) => view! {
                        <CommentWidget user={user.clone()} />
                    }.into_any(),
                    Err(_) => view! { <OAuth /> }.into_any(),
                }
            })}
        </Suspense>
    }
}

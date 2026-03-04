use icondata as i;
use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_use::use_document;
use serde::{Deserialize, Serialize};

use crate::COMMENTS_URL;

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

#[island]
pub fn Comments() -> impl IntoView {
    let auth_state = LocalResource::new(get_profile);

    view! {
        <Suspense fallback=move || view! { <div>{"Loading..."}</div> }>
            {move || Suspend::new(async move {
                let auth_state = auth_state.await;

                match auth_state {
                    Ok(ref user) => {
                        view! {
                            <p>{format!("Signed in as user {}", user.username)}</p>
                        }.into_any()
                    }
                    Err(_) => view! { <OAuth /> }.into_any(),
                }
            })}
        </Suspense>
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

use icondata as i;
use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_router::hooks::use_location;
use serde::{Deserialize, Serialize};

use crate::{COMMENTS_URL, WEBSITE_URL};

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    id: i32,
    username: String,
    avatar_url: String,
}

#[island]
fn OAuth() -> impl IntoView {
    let route = use_location().pathname.get();

    let return_url = format!("{WEBSITE_URL}{route}");
    let login_url = format!("{COMMENTS_URL}/auth/login?return_url={return_url}");

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

#[component]
pub fn Comments() -> impl IntoView {
    let auth_state = OnceResource::new(get_profile());

    view! {
        <Suspense fallback=move || view! { <div>{"Loading..."}</div> }>
            {move || Suspend::new(async move {
                let auth_state = auth_state.await;

                match auth_state {
                    Ok(ref user) => {
                        view! { <p>{format!("Signed in as user {}", user.username)}</p> }.into_any()
                    }
                    Err(_) => view! { <OAuth /> }.into_any(),
                }
            })}
        </Suspense>
    }
}

#[server]
async fn get_profile() -> Result<User, ServerFnError> {
    let url = format!("{COMMENTS_URL}/profile");

    let user = reqwest::get(url)
        .await?
        .json::<User>()
        .await?;

    Ok(user)
}

use serde::Deserialize;
use yew::{function_component, html, suspense::use_future, Html, HtmlResult, Properties, Suspense};
use yew_icons::{Icon, IconId};
use yew_router::{hooks::use_route, Routable};

use crate::pages::Route;

#[derive(Deserialize)]
pub struct User {
    id: i32,
    username: String,
    avatar_url: String,
}

async fn authenticate() -> Option<User> {
    reqwest::get("https://comments.hanyuone.live/profile")
        .await
        .ok()?
        .json::<User>()
        .await
        .ok()
}

#[function_component(OAuth)]
fn o_auth() -> Html {
    let route = use_route::<Route>();

    let return_url = format!("https://hanyuone.live/{}", route.unwrap().to_path());
    let login_url = format!("https://comments.hanyuone.live/auth/login?return_url={return_url}");

    html! {
        <div class="flex flex-col content-center">
            <a class="bg-white text-black p-2 rounded" href={login_url}>
                <div class="flex flex-row justify-center content-center">
                    <Icon icon_id={IconId::BootstrapGithub} height="1.1em" />
                    <p>
                        {"Login using GitHub"}
                    </p>
                </div>
            </a>
        </div>
    }
}

#[function_component(AuthComments)]
fn auth_comments() -> HtmlResult {
    let auth_state = use_future(authenticate)?;

    let contents = match *auth_state {
        Some(ref user) => html! { <p>{format!("Signed in as user {}", user.username)}</p> },
        None => html! { <OAuth /> },
    };

    Ok(contents)
}

#[derive(Properties, PartialEq)]
pub struct CommentsProps {}

#[function_component(Comments)]
pub fn comments(props: &CommentsProps) -> Html {
    let fallback = html! { <div>{"Loading..."}</div> };

    html! {
        <Suspense fallback={fallback}>
            <AuthComments />
        </Suspense>
    }
}

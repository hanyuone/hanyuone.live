use gloo::utils::window;
use gloo_net::http::Request;
use serde::Deserialize;
use yew::{function_component, html, suspense::use_future, Html, HtmlResult, Properties, Suspense};
use yew_icons::{Icon, IconId};

#[derive(Deserialize)]
pub struct User {
    id: i32,
    username: String,
    avatar_url: String,
}

async fn authenticate() -> Option<User> {
    let comments_auth = Request::get("http://comments.hanyuone.live/profile")
        .send()
        .await
        .ok()?;

    if comments_auth.ok() {
        let github_user = comments_auth.json::<User>().await.unwrap();
        Some(github_user)
    } else {
        None
    }
}

#[function_component(OAuth)]
fn o_auth() -> Html {
    let location = window().location();
    let return_url = location.href().unwrap();

    let login_url = format!("http://comments.hanyuone.live/auth/login?return_url={return_url}");

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

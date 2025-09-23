use reqwest::Client;
use serde::Deserialize;
#[cfg(any(feature = "hydration", feature = "static"))]
use yew::use_prepared_state;
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
    let auth_url = format!("{}/profile", env!("COMMENTS_URL"));

    let client = Client::new();

    let request_builder = client.get(auth_url);
    #[cfg(target_arch = "wasm32")]
    let request_builder = request_builder.fetch_credentials_include();

    request_builder
        .send()
        .await
        .ok()?
        .json::<User>()
        .await
        .ok()
}

#[function_component(OAuth)]
fn o_auth() -> Html {
    let route = use_route::<Route>();

    let return_url = format!("{}{}", env!("WEBSITE_URL"), route.unwrap().to_path());
    let login_url = format!("{}/auth/login?return_url={return_url}", env!("COMMENTS_URL"));

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
    #[cfg(not(any(feature = "hydration", feature = "static")))]
    let auth_state = use_future(authenticate)?;
    #[cfg(any(feature = "hydration", feature = "static"))]
    let auth_state = use_prepared_state!(
        async move |_| -> Option<User> {
            authenticate().await
        },
        ()
    )?.unwrap();

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

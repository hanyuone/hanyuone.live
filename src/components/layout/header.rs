use yew::{function_component, html, Html};
use yew_icons::{Icon, IconId};
use yew_router::prelude::Link;

use crate::pages::Route;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <nav>
            <div class="bg-primary flex flex-row px-4">
                // Home page link
                <div class="flex">
                    <Link<Route> classes="px-6 py-4 text-white" to={Route::Home}>
                        {"Hanyuan's Blog"}
                    </Link<Route>>
                </div>
                // Icons on right-hand side
                <div class="flex ml-auto items-center">
                    <a href="https://github.com/hanyuone">
                        <Icon icon_id={IconId::BootstrapGithub} height="1.1em" />
                    </a>
                </div>
            </div>
        </nav>
    }
}

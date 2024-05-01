use yew::{function_component, html, Html};
use yew_icons::{Icon, IconId};
use yew_router::prelude::Link;

use crate::pages::Route;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <nav>
            <div class="flex flex-row mx-4 border-b-[1px] border-white">
                // Home page link
                <div class="flex px-4 py-4">
                    <Link<Route> classes="font-mono" to={Route::Home}>
                        {"hanyuone.live"}
                    </Link<Route>>
                </div>
                <div class="flex px-4">
                    <Link<Route> classes="px-4 py-4 text-white transition-colors hover:bg-secondary" to={Route::About}>
                        {"About"}
                    </Link<Route>>
                    <a
                        href="/public/resume.pdf"
                        class="px-4 py-4 text-white transition-colors hover:bg-secondary">
                        {"Resume"}
                    </a>
                </div>
                // Icons on right-hand side
                <div class="flex px-4 py-4 ml-auto items-center">
                    <a
                        href="https://github.com/hanyuone"
                        class="text-neutral-200 hover:text-neutral-50">
                        <Icon icon_id={IconId::BootstrapGithub} height="1.1em" />
                    </a>
                </div>
            </div>
        </nav>
    }
}

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
                // Other pages
                <div class="flex px-4">
                    <Link<Route> classes="px-4 py-4 text-white hover:bg-green transition-colors" to={Route::About}>
                        {"About"}
                    </Link<Route>>
                    <Link<Route> classes="px-4 py-4 text-white hover:bg-green transition-colors" to={Route::Blog}>
                        {"Blog"}
                    </Link<Route>>
                    <a
                        href="/public/resume.pdf"
                        class="px-4 py-4 text-white hover:bg-green transition-colors">
                        {"Resume"}
                    </a>
                </div>
                // Icons on right-hand side
                <div class="flex px-4 py-4 ml-auto items-center">
                    <a
                        href="https://github.com/hanyuone"
                        class="px-2 text-neutral-200 hover:text-neutral-50 transition-colors">
                        <Icon icon_id={IconId::BootstrapGithub} height="1.1em" />
                    </a>
                    <a
                        href="https://linkedin.com/in/hanyuan-li"
                        class="px-2 text-neutral-200 hover:text-neutral-50 transition-colors">
                        <Icon icon_id={IconId::BootstrapLinkedin} height="1.1em" />
                    </a>
                    <a
                        href="mailto:work@hanyuone.live"
                        class="px-2 text-neutral-200 hover:text-neutral-50 transition-colors">
                        <Icon icon_id={IconId::LucideMail} height="1.1em" />
                    </a>
                </div>
            </div>
        </nav>
    }
}

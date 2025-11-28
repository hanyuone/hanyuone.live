use yew::{classes, function_component, html, use_state, Callback, Html};
use yew_icons::{Icon, IconId};
use yew_router::prelude::Link;

use crate::pages::Route;

#[function_component(Header)]
pub fn header() -> Html {
    let hamburger_open = use_state(|| false);

    let mut hamburger_classes = classes!(
        "absolute",
        "top-14",
        "w-full",
        "px-4",
        "bg-black",
        "flex",
        "flex-col",
        "transition",
        "duration-200",
        "ease-in-out",
        "text-right",
    );

    let open_classes = if *hamburger_open {
        classes!("opacity-100", "translate-y-0")
    } else {
        classes!("opacity-0", "translate-y-1")
    };

    hamburger_classes.extend(open_classes);

    html! {
        <nav>
            // Mobile view
            <div class="md:hidden flex flex-row mx-4 border-b-[1px] border-white">
                <div class="flex">
                    <Link<Route> classes="px-4 py-4 font-mono hover:bg-gray transition-colors" to={Route::Home}>
                        {"hanyuone.live"}
                    </Link<Route>>
                </div>
                <div class="flex px-4 py-4 ml-auto items-center">
                    <button class="text-neutral-200" onclick={
                        let hamburger_open = hamburger_open.clone();
                        Callback::from(move |_| {
                            hamburger_open.set(!(*hamburger_open));
                        })
                    }>
                        <Icon icon_id={IconId::LucideMenu} height="1.1em" />
                    </button>
                </div>
            </div>
            // Mobile hamburger menu
            <div class={hamburger_classes}>
                <Link<Route> classes="w-full py-2 text-white hover:bg-blue transition-colors" to={Route::Blog}>
                    {"Blog"}
                </Link<Route>>
                <a
                    href="/public/resume.pdf"
                    class="w-full py-2 text-white hover:bg-blue transition-colors">
                    {"Resume"}
                </a>
                <div class="flex items-center justify-end">
                    <a
                        href="https://github.com/hanyuone"
                        class="pr-4 py-2 text-neutral-200 hover:text-purple transition-colors">
                        <Icon icon_id={IconId::BootstrapGithub} height="1.1em" />
                    </a>
                    <a
                        href="https://linkedin.com/in/hanyuan-li"
                        class="pr-4 py-2 text-neutral-200 hover:text-blue transition-colors">
                        <Icon icon_id={IconId::BootstrapLinkedin} height="1.1em" />
                    </a>
                    <a
                        href="mailto:work@hanyuone.live"
                        class="py-2 text-neutral-200 hover:text-yellow transition-colors">
                        <Icon icon_id={IconId::LucideMail} height="1.1em" />
                    </a>
                </div>
            </div>
            // Desktop view
            <div class="hidden md:flex flex-row mx-4 border-b-[1px] border-white">
                // Home page link
                <div class="flex">
                    <Link<Route> classes="px-4 py-4 font-mono hover:bg-gray transition-colors" to={Route::Home}>
                        {"hanyuone.live"}
                    </Link<Route>>
                </div>
                // Other pages
                <div class="flex px-4">
                    <Link<Route> classes="px-4 py-4 text-white hover:bg-blue transition-colors" to={Route::Blog}>
                        {"Blog"}
                    </Link<Route>>
                    <a
                        href="/public/resume.pdf"
                        class="px-4 py-4 text-white hover:bg-blue transition-colors">
                        {"Resume"}
                    </a>
                </div>
                // Icons on right-hand side
                <div class="flex px-4 py-4 ml-auto items-center">
                    <a
                        href="https://github.com/hanyuone"
                        class="px-2 text-neutral-200 hover:text-purple transition-colors">
                        <Icon icon_id={IconId::BootstrapGithub} height="1.1em" />
                    </a>
                    <a
                        href="https://linkedin.com/in/hanyuan-li"
                        class="px-2 text-neutral-200 hover:text-blue transition-colors">
                        <Icon icon_id={IconId::BootstrapLinkedin} height="1.1em" />
                    </a>
                    <a
                        href="mailto:work@hanyuone.live"
                        class="px-2 text-neutral-200 hover:text-yellow transition-colors">
                        <Icon icon_id={IconId::LucideMail} height="1.1em" />
                    </a>
                </div>
            </div>

        </nav>
    }
}

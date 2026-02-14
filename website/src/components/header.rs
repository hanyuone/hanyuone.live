use icondata as i;
use leptos::prelude::*;
use leptos_icons::Icon;

use crate::ROOT;

#[island]
pub fn MobileHeader() -> impl IntoView {
    let root = ROOT.unwrap_or("");
    let (hamburger_open, set_hamburger_open) = signal(false);

    view! {
        <div>
            // Mobile view
            <div class="md:hidden flex flex-row mx-4 border-b-[1px] border-white">
                <div class="flex">
                    <a href={format!("{root}/")} class="px-4 py-4 font-mono hover:bg-gray transition-colors">
                        "hanyuone.live"
                    </a>
                </div>
                <div class="flex px-4 py-4 ml-auto items-center">
                    <button class="text-neutral-200" on:click=move |_| {
                        set_hamburger_open.update(|open| *open = !*open);
                    }>
                        <Icon icon={i::LuMenu} height="1.1em" />
                    </button>
                </div>
            </div>
            // Mobile hamburger menu
            <div
                class="absolute top-14 w-full px-4 bg-black flex flex-col transition duration-200 ease-in-out text-right"
                class=(["opacity-100", "translate-y-0", "z-20"], move || hamburger_open.get())
                class=(["opacity-0", "translate-y-1", "z-0"], move || !hamburger_open.get())>
                <a href={format!("{root}/blog")} class="w-full py-2 text-white hover:bg-blue transition-colors">
                    "Blog"
                </a>
                <a
                    href={format!("{root}/resume.pdf")}
                    class="w-full py-2 text-white hover:bg-blue transition-colors">
                    "Resume"
                </a>
                <div class="flex items-center justify-end">
                    <a
                        href="https://github.com/hanyuone"
                        class="pr-4 py-2 text-neutral-200 hover:text-purple transition-colors">
                        <Icon icon={i::BsGithub} height="1.1em" />
                    </a>
                    <a
                        href="https://linkedin.com/in/hanyuan-li"
                        class="pr-4 py-2 text-neutral-200 hover:text-blue transition-colors">
                        <Icon icon={i::BsLinkedin} height="1.1em" />
                    </a>
                    <a
                        href="mailto:work@hanyuone.live"
                        class="py-2 text-neutral-200 hover:text-yellow transition-colors">
                        <Icon icon={i::LuMail} height="1.1em" />
                    </a>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn Header() -> impl IntoView {
    let root = ROOT.unwrap_or("");

    view! {
        <nav>
            <MobileHeader />
            // Desktop view
            <div class="hidden md:flex flex-row mx-4 border-b border-white">
                // Home page link
                <div class="flex">
                    <a href={format!("{root}/")} class="px-4 py-4 font-mono hover:bg-gray transition-colors">
                        "hanyuone.live"
                    </a>
                </div>
                // Other pages
                <div class="flex px-4">
                    <a href={format!("{root}/blog")} class="px-4 py-4 text-white hover:bg-blue transition-colors">
                        "Blog"
                    </a>
                    <a
                        href={format!("{root}/resume.pdf")}
                        class="px-4 py-4 text-white hover:bg-blue transition-colors">
                        "Resume"
                    </a>
                </div>
                // Icons on right-hand side
                <div class="flex px-4 py-4 ml-auto items-center">
                    <a
                        href="https://github.com/hanyuone"
                        class="px-2 text-neutral-200 hover:text-purple transition-colors">
                        <Icon icon={i::BsGithub} height="1.1em" />
                    </a>
                    <a
                        href="https://linkedin.com/in/hanyuan-li"
                        class="px-2 text-neutral-200 hover:text-blue transition-colors">
                        <Icon icon={i::BsLinkedin} height="1.1em" />
                    </a>
                    <a
                        href="mailto:work@hanyuone.live"
                        class="px-2 text-neutral-200 hover:text-yellow transition-colors">
                        <Icon icon={i::LuMail} height="1.1em" />
                    </a>
                </div>
            </div>
        </nav>
    }
}

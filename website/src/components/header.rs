use icondata as i;
use leptos::prelude::*;

use crate::components::icon::Icon;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <nav>
            // Desktop view
            <div class="hidden md:flex flex-row mx-4 border-b border-white">
                // Home page link
                <div class="flex">
                    <a href="/" class="px-4 py-4 font-mono hover:bg-gray transition-colors">
                        "hanyuone.live"
                    </a>
                </div>
                // Other pages
                <div class="flex px-4">
                    <a href="/blog" class="px-4 py-4 text-white hover:bg-blue transition-colors">
                        "Blog"
                    </a>
                    <a
                        href="/public/resume.pdf"
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

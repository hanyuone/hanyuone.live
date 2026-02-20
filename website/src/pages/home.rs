use leptos::prelude::*;

use crate::{
    components::home::{background::Background, home_animation::HomeAnimation},
    ROOT,
};

#[component]
pub fn HomePage() -> impl IntoView {
    let root = ROOT.unwrap_or("");

    view! {
        <div class="h-(--fullmain) w-full absolute top-[57px] left-0 blur-sm z-0">
            <Background />
        </div>
        <div class="relative h-(--main) z-10 snap-y snap-mandatory overflow-x-hidden overflow-y-scroll no-scrollbar">
            <div class="snap-always snap-center">
                <div class="min-h-(--main)">
                    <HomeAnimation />
                </div>
            </div>
            <div class="snap-always snap-center">
                <div class="min-h-(--main)">
                    <div class="h-(--main) flex flex-col justify-center">
                        <p>
                            "Welcome to this little corner of the internet! (The internet sure has a lot of corners...)"
                            <br /><br />
                            "I have a deep fascination with programming languages. I have made a couple of\u{00a0}\u{200b}"
                            <a
                                class="text-green underline transition hover:bg-gray"
                                href="https://github.com/hanyuone/pancake">
                                "hobby"
                            </a>
                            "\u{00a0}\u{200b}"
                            <a
                                class="text-green underline transition hover:bg-gray"
                                href="https://github.com/hedron-crystal/hedron">
                                "programming"
                            </a>
                            "\u{00a0}\u{200b}"
                            <a
                                class="text-green underline transition hover:bg-gray"
                                href="https://github.com/LogicodeLang/Logicode">
                                "languages"
                            </a>
                            "\u{00a0}\u{200b}over the years, to varying degrees of success. I am currently a tutor for the\u{00a0}\u{200b}"
                            <a
                                class="text-green underline transition hover:bg-gray"
                                href="https://www.handbook.unsw.edu.au/undergraduate/courses/2025/COMP6991">
                                "Rust course"
                            </a>
                            "\u{00a0}\u{200b}at UNSW, which focuses on Rust's design choices and how those choices
                             (and aspects borrowed from other programming paradigms) help programmers avoid bugs."
                            <br /><br />
                            "I'm also interested in real-world languages - I speak Mandarin Chinese and English, and am learning French and Hokkien.
                             I translate web stories as a hobby, you can find some of my work in\u{00a0}\u{200b}"
                            <a
                                class="text-green underline transition hover:bg-gray"
                                href={format!("{root}/tag/translation")}>
                                "my blog"
                            </a>
                            "."
                        </p>
                    </div>
                </div>
            </div>
            <div class="snap-always snap-center">
                <div class="min-h-(--main)">
                    <div class="h-(--main) flex flex-col justify-center">
                        <p>
                            "In terms of employment, I have completed several internships in Sydney, covering everything from
                             fullstack development to Unity to working with microcontrollers. You can see my\u{00a0}\u{200b}"
                            <a
                                class="text-green underline transition hover:bg-gray"
                                href="/public/resume.pdf">
                                "resume"
                            </a>
                            "\u{00a0}\u{200b}for more information."
                            <br /><br />
                            "Finally, you can find my book, manga and film ratings on\u{00a0}\u{200b}"
                            <a
                                class="text-green underline transition hover:bg-gray"
                                href="https://app.thestorygraph.com/profile/hanyuone">
                                "StoryGraph"
                            </a>
                            ",\u{00a0}\u{200b}"
                            <a
                                class="text-green underline transition hover:bg-gray"
                                href="https://myanimelist.net/profile/hanyuone">
                                "MyAnimeList"
                            </a>
                            "\u{00a0}\u{200b}and\u{00a0}\u{200b}"
                            <a
                                class="text-green underline transition hover:bg-gray"
                                href="https://letterboxd.com/hanyuone">
                                "Letterboxd"
                            </a>
                            "\u{00a0}\u{200b}respectively."
                        </p>
                    </div>
                </div>
            </div>
        </div>
    }
}

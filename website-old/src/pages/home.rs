use markdown::structs::tag::TagId;
use yew::{function_component, html, use_state, Callback, Html};
use yew_router::prelude::Link;

use crate::{
    components::{
        head::Head,
        home::{
            background::Background,
            typewriter::{Block, Typewriter},
        },
    },
    pages::Route,
};

const INTRO_BLOCKS: [Block; 4] = [
    Block {
        // We need Unicode escape here (equivalent to "&nbsp;") since Yew
        // does not allow for us to do <p>&nbsp;</p> by itself
        text: "Hi, my name is\u{00a0}\u{200b}",
        class: "font-bold text-3xl",
    },
    Block {
        text: "Hanyuan",
        class: "font-bold text-3xl text-blue",
    },
    Block {
        text: ".",
        class: "font-bold text-3xl",
    },
    Block {
        // Simulate the blinking cursor effect
        text: ".",
        class: "text-3xl text-white text-opacity-0 animate-blink",
    },
];

#[function_component(Page)]
pub fn page() -> Html {
    let typing_complete = use_state(|| false);

    let opacity = if *typing_complete {
        "opacity-100"
    } else {
        "opacity-0"
    };

    html! {
        <>
            <Head>
                <title>{"Hanyuan's Website"}</title>
            </Head>
            <div class="h-fullmain w-full absolute top-[57px] left-0 blur-sm z-0">
                <Background />
            </div>
            <div class="relative h-main z-10 snap-y snap-mandatory overflow-x-hidden overflow-y-scroll no-scrollbar">
                <div class="snap-always snap-center">
                    <div class="min-h-main">
                        <div class="h-main flex flex-col justify-center items-center">
                            <div class="flex flex-row">
                                <Typewriter
                                    blocks={INTRO_BLOCKS.to_vec()}
                                    on_finish={Callback::from(move |_| {
                                        typing_complete.set(true);
                                    })} />
                            </div>
                            <div class={format!("{} {}", "flex flex-row transition duration-500", opacity)}>
                                <p>
                                    {"I am a final-year Advanced Computer Science student at\u{00a0}\u{200b}"}
                                    <a
                                        class="text-green underline transition hover:bg-gray"
                                        href="https://www.unsw.edu.au/engineering/our-schools/computer-science-and-engineering">
                                        {"UNSW Sydney"}
                                    </a>
                                    {"."}
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
                <div class="snap-always snap-center">
                    <div class="min-h-main">
                        <div class="h-main flex flex-col justify-center">
                            <p>{"Welcome to this little corner of the internet! (The internet sure has a lot of corners...)"}</p>
                            <br />
                            <p>
                                {"I have a deep fascination with programming languages. I have made a couple of\u{00a0}\u{200b}"}
                                <a
                                    class="text-green underline transition hover:bg-gray"
                                    href="https://github.com/hanyuone/pancake">
                                    {"hobby"}
                                </a>
                                {"\u{00a0}\u{200b}"}
                                <a
                                    class="text-green underline transition hover:bg-gray"
                                    href="https://github.com/hedron-crystal/hedron">
                                    {"programming"}
                                </a>
                                {"\u{00a0}\u{200b}"}
                                <a
                                    class="text-green underline transition hover:bg-gray"
                                    href="https://github.com/LogicodeLang/Logicode">
                                    {"languages"}
                                </a>
                                {"\u{00a0}\u{200b}over the years, to varying degrees of success. I am currently a tutor for the\u{00a0}\u{200b}"}
                                <a
                                    class="text-green underline transition hover:bg-gray"
                                    href="https://www.handbook.unsw.edu.au/undergraduate/courses/2025/COMP6991">
                                    {"Rust course"}
                                </a>
                                {"\u{00a0}\u{200b}at UNSW, which focuses on Rust's design choices and how those choices
                                 (and aspects borrowed from other programming paradigms) help programmers avoid bugs."}
                            </p>
                            <br />
                            <p>
                                {"I'm also interested in real-world languages - I speak Mandarin Chinese and English, and am learning French and Hokkien.
                                  I translate web stories as a hobby, you can find some of my work in\u{00a0}\u{200b}"}
                                <Link<Route> classes="text-green underline transition hover:bg-gray" to={Route::Tag { tag_id: TagId::Translation }}>
                                    {"my blog"}
                                </Link<Route>>
                                {"."}
                            </p>
                        </div>
                    </div>
                </div>
                <div class="snap-always snap-center">
                    <div class="min-h-main">
                        <div class="h-main flex flex-col justify-center">
                            <p>
                                {"In terms of employment, I have completed several internships in Sydney, covering everything from
                                  fullstack development to Unity to working with microcontrollers. You can see my\u{00a0}\u{200b}"}
                                <a
                                    class="text-green underline transition hover:bg-gray"
                                    href="/public/resume.pdf">
                                    {"resume"}
                                </a>
                                {"\u{00a0}\u{200b}for more information."}
                            </p>
                            <br />
                            <p>
                                {"Finally, you can find my book, manga and film ratings on\u{00a0}\u{200b}"}
                                <a
                                    class="text-green underline transition hover:bg-gray"
                                    href="https://app.thestorygraph.com/profile/hanyuone">
                                    {"StoryGraph"}
                                </a>
                                {",\u{00a0}\u{200b}"}
                                <a
                                    class="text-green underline transition hover:bg-gray"
                                    href="https://myanimelist.net/profile/hanyuone">
                                    {"MyAnimeList"}
                                </a>
                                {"\u{00a0}\u{200b}and\u{00a0}\u{200b}"}
                                <a
                                    class="text-green underline transition hover:bg-gray"
                                    href="https://letterboxd.com/hanyuone">
                                    {"Letterboxd"}
                                </a>
                                {"\u{00a0}\u{200b}respectively."}
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        </>
    }
}

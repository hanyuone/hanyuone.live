use yew::{function_component, html, use_state, Callback, Html};

use crate::components::{
    head::Head,
    home::background::Background,
    home::typewriter::{Block, Typewriter},
};

const INTRO_BLOCKS: [Block; 4] = [
    Block {
        // We need Unicode escape here (equivalent to "&nbsp;") since Yew
        // does not allow for us to do <p>&nbsp;</p> by itself
        text: "Hi, my name is\u{00a0}",
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
        class: "text-3xl text-white text-opacity-0 animate-blink"
    }
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
            <div class="relative flex flex-col">
                <div class="h-home blur-sm z-0">
                    <Background />
                </div>
                <div class="h-home content-center -mt-home z-10">
                    <div class="flex flex-col">
                        <div class="flex flex-row justify-center">
                            <Typewriter
                                blocks={INTRO_BLOCKS.to_vec()}
                                on_finish={Callback::from(move |_| {
                                    typing_complete.set(true);
                                })} />
                        </div>
                        <div class={format!("{} {}", "flex flex-row justify-center transition duration-500", opacity)}>
                            <p>
                                {"I am a final-year Advanced Computer Science student at\u{00a0}"}
                                <a
                                    class="text-green underline transition hover:bg-gray"
                                    href="https://www.unsw.edu.au/engineering/our-schools/computer-science-and-engineering">
                                    {"UNSW"}
                                </a>
                                {"."}
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        </>
    }
}

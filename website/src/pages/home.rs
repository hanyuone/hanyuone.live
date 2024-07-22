use yew::{function_component, html, use_state, Callback, Html};

use crate::components::{
    head::Head,
    home::typewriter::{Block, Typewriter},
};

const BLOCKS: [Block; 3] = [
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
            <div class="flex flex-col">
                <Typewriter
                    blocks={BLOCKS.to_vec()}
                    on_finish={Callback::from(move |_| {
                        typing_complete.set(true);
                    })} />
                <div class={format!("{} {}", "flex flex-row transition duration-500", opacity)}>
                    <p>{"I am a penultimate-year Computer Science student at\u{00a0}"}</p>
                    <a
                        class="text-green underline transition hover:bg-black-light"
                        href="https://www.unsw.edu.au/engineering/our-schools/computer-science-and-engineering">
                        {"UNSW"}
                    </a>
                    <p>{"."}</p>
                </div>
            </div>
        </>
    }
}

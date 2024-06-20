use yew::{function_component, html, use_state, Callback, Html};

use crate::components::{head::Head, home::typewriter::{ParagraphData, Typewriter}};

const PARAGRAPHS: [ParagraphData; 3] = [
    ParagraphData {
        text: "Hi, my name is &nbsp;",
        class: "font-bold text-3xl",
    },
    ParagraphData {
        text: "Hanyuan",
        class: "font-bold text-3xl text-blue",
    },
    ParagraphData {
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
            <div class="ml-20 flex flex-col">
                <Typewriter
                    paragraphs={PARAGRAPHS.to_vec()}
                    on_finish={Callback::from(move |_| {
                        typing_complete.set(true);
                    })} />
                <div class="flex flex-row">
                    <p class={format!("{} {}", "transition", opacity)}>
                        {"I am a penultimate-year Computer Science student at UNSW."}
                    </p>
                </div>
            </div>
        </>
    }
}

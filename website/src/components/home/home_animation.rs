use leptos::prelude::*;
use leptos_use::{use_interval, UseIntervalReturn};
use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize)]
struct Block {
    pub text: &'static str,
    pub class: &'static str,
}

struct BlockWithIndex {
    pub block: Block,
    pub start_index: usize,
}

const INTRO_BLOCKS: [Block; 4] = [
    Block {
        // We need Unicode escape here (equivalent to "&nbsp;") since Yew
        // does not allow for us to do <p>&nbsp;</p> by itself
        text: "Hi, my name's\u{00a0}\u{200b}",
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
        class: "text-3xl text-white/0 animate-blink",
    },
];

#[island]
pub fn HomeAnimation() -> impl IntoView {
    // Adds "starting index" to each text block, for typewriter
    // animation purposes
    // TODO: have this run once instead of on every rerender
    let mut total_length = 0;
    let mut display_list = vec![];

    for block in &INTRO_BLOCKS {
        display_list.push(BlockWithIndex {
            block: block.clone(),
            start_index: total_length,
        });

        total_length += block.text.chars().count();
    }

    let UseIntervalReturn {
        counter,
        reset: _,
        is_active,
        pause,
        resume: _,
    } = use_interval(100);

    Effect::watch(
        move || counter.get(),
        move |counter, _, _| {
            if (*counter as usize) >= total_length {
                pause();
            }
        },
        false,
    );

    view! {
        <div class="h-(--main) flex flex-col justify-center items-center">
            <div class="flex flex-row">
                {move || {
                    display_list
                        .iter()
                        .map(|display| {
                            let BlockWithIndex { block, start_index } = display;

                            let display_text = if (counter.get() as usize) < *start_index {
                                String::new()
                            } else {
                                let adjusted_index = (counter.get() as usize) - start_index;
                                block.text.chars()
                                    .take(adjusted_index)
                                    .collect::<String>()
                            };

                            view! {
                                <span class={block.class.to_string()}>{display_text}</span>
                            }
                        })
                        .collect::<Vec<_>>()
                }}
            </div>
            <div
                class="flex flex-row transition duration-500"
                class=("opacity-0", move || counter.get() == 0 || is_active.get())>
                <p>
                    "I am a final-year Advanced Computer Science student at\u{00a0}\u{200b}"
                    <a
                        class="text-green underline transition hover:bg-gray"
                        href="https://www.unsw.edu.au/engineering/our-schools/computer-science-and-engineering">
                        "UNSW Sydney"
                    </a>
                    "."
                </p>
            </div>
        </div>
    }
}

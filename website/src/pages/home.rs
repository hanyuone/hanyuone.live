use yew::{function_component, html, use_state, Html};
use yew_hooks::use_interval;

use crate::components::head::Head;

const TYPEWRITER_TEXT: &str = "Hi, my name is Hanyuan.";

#[function_component(Page)]
pub fn page() -> Html {
    let index = use_state(|| 0);
    let typing_complete = use_state(|| false);

    {
        let index_clone = index.clone();
        let typing_complete_clone = typing_complete.clone();

        use_interval(move || {
            if *index_clone < TYPEWRITER_TEXT.len() {
                index_clone.set(*index_clone + 1);
            } else {
                typing_complete_clone.set(true);
            }
        }, 100);
    }

    html! {
        <>
            <Head>
                <title>{"Hanyuan's Website"}</title>
            </Head>
            <div class="h-full flex flex-col">
                <div class="flex flex-row justify-center">
                    <p class="font-bold text-3xl">{&TYPEWRITER_TEXT[..*index]}</p>
                    <div class="w-1 h-8 animate-blink" />
                </div>
                <div class="flex flex-row justify-center">
                    <p class={format!("{} {}", "transition", if *typing_complete { "opacity-100" } else { "opacity-0" })}>{"Welcome to my blog!"}</p>
                </div>
            </div>
        </>
    }
}

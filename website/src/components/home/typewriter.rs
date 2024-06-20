use yew::{function_component, html, html_nested, use_state, Callback, Html, Properties};
use yew_hooks::use_interval;

#[derive(Clone, PartialEq)]
pub struct ParagraphData {
    pub text: &'static str,
    pub class: &'static str,
}

struct DisplayData {
    text: &'static str,
    class: &'static str,
    start: usize,
}

#[derive(Properties, PartialEq)]
pub struct TypewriterProps {
    pub paragraphs: Vec<ParagraphData>,
    pub on_finish: Callback<()>,
}

#[function_component(Typewriter)]
pub fn typewriter(props: &TypewriterProps) -> Html {
    let index = use_state(|| 0);

    let mut length = 0;
    let mut display_list = vec![];

    for paragraph in &props.paragraphs {
        let ParagraphData { text, class } = paragraph;
        display_list.push(DisplayData {
            text,
            class,
            start: length,
        });

        length += text.len();
    }

    {
        let on_finish = props.on_finish.clone();
        let index_clone = index.clone();

        use_interval(
            move || {
                if *index_clone < length {
                    index_clone.set(*index_clone + 1);
                } else {
                    on_finish.emit(());
                }
            },
            100,
        );
    }

    html! {
        <div class="flex flex-row">
            {
                display_list
                    .iter()
                    .map(|display| {
                        let DisplayData { text, class, start } = display;

                        let display_text = if *index.clone() < *start {
                            ""
                        } else {
                            let adjusted_index = *index.clone() - start;

                            if adjusted_index < text.len() {
                                &text[..adjusted_index]
                            } else {
                                text
                            }
                        };

                        html_nested! {
                            <p class={class.to_string()}>{display_text}</p>
                        }
                    })
                    .collect::<Vec<_>>()
            }
            <div class="w-1 h-8 animate-blink" />
        </div>
    }
}

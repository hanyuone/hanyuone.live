use yew::{function_component, html, html_nested, use_state, Callback, Html, Properties};
use yew_hooks::use_interval;

#[derive(Clone, PartialEq)]
pub struct Block {
    pub text: &'static str,
    pub class: &'static str,
}

struct TypewriterBlock {
    block: Block,
    start: usize,
}

#[derive(Properties, PartialEq)]
pub struct TypewriterProps {
    pub blocks: Vec<Block>,
    pub on_finish: Callback<()>,
}

#[function_component(Typewriter)]
pub fn typewriter(props: &TypewriterProps) -> Html {
    let index = use_state(|| 0);

    let mut length = 0;
    let mut display_list = vec![];

    for block in &props.blocks {
        display_list.push(TypewriterBlock {
            block: block.clone(),
            start: length,
        });

        length += block.text.chars().count();
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
                        let TypewriterBlock { block, start } = display;

                        let display_text = if *index.clone() < *start {
                            String::new()
                        } else {
                            let adjusted_index = *index.clone() - start;
                            block.text.chars()
                                .take(adjusted_index)
                                .collect::<String>()
                        };

                        html_nested! {
                            <p class={block.class.to_string()}>{display_text}</p>
                        }
                    })
                    .collect::<Vec<_>>()
            }
            <div class="w-1 h-8 animate-blink" />
        </div>
    }
}

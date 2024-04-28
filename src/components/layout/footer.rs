use yew::{function_component, html, Html};

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="mx-4 border-t-[1px]">
            <div class="flex flex-row px-4 py-4">
                <div class="flex">
                    <div>{"Made with <3 in 2024"}</div>
                </div>
            </div>
        </footer>
    }
}
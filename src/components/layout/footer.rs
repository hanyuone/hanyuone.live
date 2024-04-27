use yew::{function_component, html, Html};

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="bg-primary px-4 text-white">
            <div class="flex flex-row">
                <div class="flex">
                    <div>{"Made with <3 in 2024"}</div>
                </div>
            </div>
        </footer>
    }
}
use yew::{function_component, html, Html};

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="mx-4 border-t-[1px]">
            <div class="flex flex-row px-4 py-4">
                <div class="flex">
                    <p>{"Made with\u{00a0}"}</p>
                    <p class="text-red">{"<3"}</p>
                    <p>{"\u{00a0}in 2024"}</p>
                </div>
            </div>
        </footer>
    }
}

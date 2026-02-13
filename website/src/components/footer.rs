use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="mx-4 border-t">
            <div class="flex flex-row px-4 py-4">
                <div class="flex">
                    <p>"Made with\u{00a0}\u{200b}"</p>
                    <p class="text-red">"<3"</p>
                    <p>"\u{00a0}\u{200b}in 2024-25"</p>
                </div>
            </div>
        </footer>
    }
}

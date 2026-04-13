pub mod app;
pub mod components;
pub mod context;
pub mod pages;
pub mod renderer;

const ROOT: Option<&'static str> = option_env!("ROOT");

const COMMENTS_URL: &str = env!("COMMENTS_URL");

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_islands();
}

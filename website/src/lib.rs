// Needed for Leptos, as of Rust 1.94.0
#![recursion_limit = "256"]

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
    // See https://book.leptos.dev/islands.html#admonition-note
    #[allow(unused_imports)]
    use app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_islands();
}

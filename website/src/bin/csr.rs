use std::panic;

use wasm_bindgen::prelude::*;
use website::{
    app::{App, AppProps},
    context::BlogContext,
};

#[wasm_bindgen(main)]
async fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let blog_map_url = format!("{}/public/blog/blog_map.ron", env!("WEBSITE_URL"));
    let raw_blog_metadata = reqwest::get(blog_map_url)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let blog = BlogContext::new(&raw_blog_metadata);

    let app = yew::Renderer::<App>::with_props(AppProps { blog });

    #[cfg(feature = "hydration")]
    app.hydrate();

    #[cfg(not(feature = "hydration"))]
    app.render();
}

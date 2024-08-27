use gloo_net::http::Request;
use wasm_bindgen::prelude::*;
use website::{
    app::{App, AppProps},
    context::BlogContext,
};

#[wasm_bindgen(main)]
async fn main() {
    // FIXME: prevent double-building on MD rerender
    let raw_blog_metadata = Request::get("/public/blog/blog_map.ron")
        .send()
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

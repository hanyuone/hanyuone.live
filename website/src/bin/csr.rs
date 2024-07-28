use gloo_net::http::Request;
use wasm_bindgen::prelude::*;
use website::{
    app::{App, AppProps},
    context::BlogContext,
};

#[wasm_bindgen(main)]
async fn main() {
    // FIXME: prevent double-building on MD rerender
    let raw_content = Request::get("/public/blog/blog_map")
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

    let blog = BlogContext::new(raw_content.as_bytes());
    let app = yew::Renderer::<App>::with_props(AppProps { blog });

    #[cfg(feature = "hydration")]
    app.hydrate();

    #[cfg(not(feature = "hydration"))]
    app.render();
}

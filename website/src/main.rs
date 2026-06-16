#![recursion_limit = "256"]

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use std::fs;

    use leptos::prelude::*;
    use leptos_axum::generate_route_list_with_exclusions_and_ssg_and_context;
    use website::{app::*, context::BlogContext};

    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;

    let raw_blog_map = fs::read("./blogs/blog_map.ron").unwrap();
    let blog_context = BlogContext::new(&String::from_utf8(raw_blog_map).unwrap());

    // Generate the list of routes in your Leptos App
    let (_routes, static_routes) = generate_route_list_with_exclusions_and_ssg_and_context(
        {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        },
        None,
        {
            let blog_context = blog_context.clone();
            move || provide_context(blog_context.clone())
        },
    );

    static_routes.generate(&leptos_options).await;

    #[cfg(debug_assertions)]
    {
        use axum::Router;
        use leptos::logging::log;
        use leptos_axum::LeptosRoutes;

        let addr = leptos_options.site_addr;

        let app = Router::new()
            .leptos_routes(&leptos_options, _routes, {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            })
            .fallback(leptos_axum::file_and_error_handler_with_context(
                {
                    let blog_context = blog_context.clone();
                    move || provide_context(blog_context.clone())
                },
                shell,
            ))
            .with_state(leptos_options);

        // run our app with hyper
        // `axum::Server` is a re-export of `hyper::Server`
        log!("listening on http://{}", &addr);
        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
        axum::serve(listener, app.into_make_service())
            .await
            .unwrap();
    }
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}

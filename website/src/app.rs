use std::path::Path;

use futures::{channel::mpsc, Stream};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
    static_routes::StaticRoute,
    SsrMode,
};

use crate::{
    components::{footer::Footer, header::Header},
    pages::{blog::BlogPage, blog_post::BlogPostPage, home::HomePage},
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    let root = option_env!("ROOT").unwrap_or("");

    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options root />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[allow(unused)] // path is not used in non-SSR
fn watch_path(path: &Path) -> impl Stream<Item = ()> {
    #[allow(unused)]
    let (mut tx, rx) = mpsc::channel(0);

    #[cfg(feature = "ssr")]
    {
        use notify::{RecursiveMode, Watcher};

        let mut watcher = notify::recommended_watcher(move |res: Result<_, _>| {
            if res.is_ok() {
                // if this fails, it's because the buffer is full
                // this means we've already notified before it's regenerated,
                // so this page will be queued for regeneration already
                _ = tx.try_send(());
            }
        })
        .expect("could not create watcher");

        // Add a path to be watched. All files and directories at that path and
        // below will be monitored for changes.
        watcher
            .watch(path, RecursiveMode::NonRecursive)
            .expect("could not watch path");

        // we want this to run as long as the server is alive
        std::mem::forget(watcher);
    }

    rx
}

#[server]
pub async fn list_slugs() -> Result<Vec<String>, ServerFnError> {
    use markdown::structs::{blog::BlogId, metadata::BlogMetadata};
    use std::collections::HashMap;
    use tokio::fs;

    let raw_blog_map = fs::read("./blogs/blog_map.ron").await?;
    let blog_map =
        ron::from_str::<HashMap<BlogId, BlogMetadata>>(&String::from_utf8(raw_blog_map).unwrap())?;

    let slugs = blog_map.keys().map(|id| id.to_string()).collect::<Vec<_>>();

    Ok(slugs)
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    const ROOT: Option<&'static str> = option_env!("ROOT");
    let (sheets_href, base) = match ROOT {
        Some(root) => (format!("{root}/pkg/website.css"), root),
        None => ("/pkg/website.css".to_string(), ""),
    };

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href=sheets_href />

        // sets the document title
        <Title text="Welcome to Leptos" />

        // content for this welcome page
        <Router base>
            <div class="bg-black text-white flex flex-col min-h-screen justify-between">
                <Header />
                <main class="grow p-20">
                    <Routes fallback=|| "Page not found.".into_view()>
                        <Route
                            path=path!("/")
                            view=HomePage
                            ssr=SsrMode::Static(
                                StaticRoute::new(),
                            ) />
                        <Route
                            path=path!("/blog")
                            view=BlogPage
                            ssr=SsrMode::Static(
                                StaticRoute::new().regenerate(|_| watch_path(Path::new("./blogs/blog_map.ron"))),
                            ) />
                        <Route
                            path=path!("/blog/:slug")
                            view=BlogPostPage
                            ssr=SsrMode::Static(
                                StaticRoute::new()
                                    .prerender_params(|| async move {
                                        [("slug".into(), list_slugs().await.unwrap_or_default())]
                                            .into_iter()
                                            .collect()
                                    })
                                    .regenerate(|params| {
                                        let slug = params.get("slug").unwrap();
                                        watch_path(Path::new(&format!("./blogs/parsed/{slug}.ron")))
                                    }),
                            ) />
                    </Routes>
                </main>
                <Footer />
            </div>
        </Router>
    }
}

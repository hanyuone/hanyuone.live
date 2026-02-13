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
use markdown::structs::tag::TagId;

use crate::{
    components::{footer::Footer, header::Header},
    context::BlogContext,
    pages::{blog::BlogPage, blog_post::BlogPostPage, home::HomePage, tag::TagPage},
    ROOT,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    let root = ROOT.unwrap_or("");

    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options root islands=true />
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

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let metadata_map = use_context::<BlogContext>().unwrap();

    let formatter = |text| format!("{text} - Hanyuan's site");

    let (sheets_href, base) = match ROOT {
        Some(root) => (format!("{root}/pkg/website.css"), root),
        None => ("/pkg/website.css".to_string(), ""),
    };

    let slugs = metadata_map
        .content
        .keys()
        .map(|id| id.to_string())
        .collect::<Vec<_>>();

    let tags = TagId::get_all()
        .into_iter()
        .map(|tag| tag.to_string())
        .collect::<Vec<_>>();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href=sheets_href />

        // sets the document title
        <Title formatter />

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
                                    .prerender_params(move || {
                                        let slugs = slugs.clone();

                                        async move {
                                            [("slug".into(), slugs.clone())]
                                                .into_iter()
                                                .collect()
                                        }
                                    })
                                    .regenerate(|params| {
                                        let slug = params.get("slug").unwrap();
                                        watch_path(Path::new(&format!("./blogs/parsed/{slug}.ron")))
                                    }),
                            ) />
                        <Route
                            path=path!("tag/:tag_id")
                            view=TagPage
                            ssr=SsrMode::Static(
                                StaticRoute::new()
                                    .prerender_params(move || {
                                        let tags = tags.clone();

                                        async move {
                                            [("tag_id".into(), tags.clone())]
                                                .into_iter()
                                                .collect()
                                        }
                                    })
                                    .regenerate(|_| watch_path(Path::new("./blogs/tags.yaml")))
                            ) />
                    </Routes>
                </main>
                <Footer />
            </div>
        </Router>
    }
}

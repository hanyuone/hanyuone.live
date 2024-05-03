use yew::{
    function_component, html, html::PhantomComponent, suspense::use_future, Children,
    ContextProvider, Html, HtmlResult, Properties, Suspense,
};
use yew_router::{
    history::{AnyHistory, MemoryHistory},
    BrowserRouter, Routable, Router, Switch,
};

use crate::{
    components::layout::Layout,
    context::{BlogContext, HeadContext},
    pages::Route,
};

#[function_component(AppContent)]
fn app_content() -> Html {
    html! {
        <Layout>
            <Switch<Route> render={Route::switch} />
        </Layout>
    }
}

#[derive(PartialEq, Properties)]
struct BlogContextWrapperProps {
    pub children: Children,
}

#[function_component(BlogContextWrapper)]
fn blog_context_wrapper(props: &BlogContextWrapperProps) -> HtmlResult {
    let blog_context = use_future(BlogContext::new)?;

    Ok(html! {
        <ContextProvider<BlogContext> context={blog_context.clone()}>
            {props.children.clone()}
        </ContextProvider<BlogContext>>
    })
}

#[function_component(App)]
pub fn app() -> Html {
    let fallback = html! { <p>{"Loading..."}</p> };

    html! {
        <PhantomComponent<ContextProvider<HeadContext>>>
            <Suspense {fallback}>
                <BlogContextWrapper>
                    <BrowserRouter>
                        <AppContent />
                    </BrowserRouter>
                </BlogContextWrapper>
            </Suspense>
        </PhantomComponent<ContextProvider<HeadContext>>>
    }
}

#[derive(PartialEq, Properties)]
pub struct StaticAppProps {
    pub route: Route,
    pub head: HeadContext,
}

impl StaticAppProps {
    fn create_history(&self) -> AnyHistory {
        let path = self.route.to_path();
        let history = MemoryHistory::with_entries(vec![path]);
        history.into()
    }
}

#[function_component(StaticApp)]
pub fn static_app(props: &StaticAppProps) -> Html {
    let fallback = html! { <p>{"Loading..."}</p> };

    let history = props.create_history();

    html! {
        <ContextProvider<HeadContext> context={props.head.clone()}>
            <Suspense {fallback}>
                <BlogContextWrapper>
                    <Router history={history}>
                        <AppContent />
                    </Router>
                </BlogContextWrapper>
            </Suspense>
        </ContextProvider<HeadContext>>
    }
}

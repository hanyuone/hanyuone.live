use yew::{function_component, html, html::PhantomComponent, ContextProvider, Html, Properties};
use yew_router::{
    history::{AnyHistory, MemoryHistory},
    BrowserRouter, Routable, Router, Switch,
};

use crate::{
    components::layout::Layout,
    context::{BlogContext, HeadContext, TagContext},
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
pub struct AppProps {
    pub blog: BlogContext,
    pub tags: TagContext,
}

#[function_component(App)]
pub fn app(props: &AppProps) -> Html {
    html! {
        <PhantomComponent<ContextProvider<HeadContext>>>
            <ContextProvider<BlogContext> context={props.blog.clone()}>
                <ContextProvider<TagContext> context={props.tags.clone()}>
                    <BrowserRouter>
                        <AppContent />
                    </BrowserRouter>
                </ContextProvider<TagContext>>
            </ContextProvider<BlogContext>>
        </PhantomComponent<ContextProvider<HeadContext>>>
    }
}

#[derive(PartialEq, Properties)]
pub struct StaticAppProps {
    pub route: Route,
    pub head: HeadContext,
    pub blog: BlogContext,
    pub tags: TagContext,
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
    let history = props.create_history();

    html! {
        <ContextProvider<HeadContext> context={props.head.clone()}>
            <ContextProvider<BlogContext> context={props.blog.clone()}>
                <ContextProvider<TagContext> context={props.tags.clone()}>
                    <Router history={history}>
                        <AppContent />
                    </Router>
                </ContextProvider<TagContext>>
            </ContextProvider<BlogContext>>
        </ContextProvider<HeadContext>>
    }
}

use yew::{function_component, html, html::PhantomComponent, ContextProvider, Html, Properties};
use yew_router::{
    history::{AnyHistory, MemoryHistory},
    BrowserRouter, Routable, Router, Switch,
};

use crate::{
    components::{head::HeadContext, layout::Layout},
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

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <PhantomComponent<ContextProvider<HeadContext>>>
            <BrowserRouter>
                <AppContent />
            </BrowserRouter>
        </PhantomComponent<ContextProvider<HeadContext>>>
    }
}

#[derive(PartialEq, Properties)]
pub struct StaticAppProps {
    route: Route,
    head: HeadContext,
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
            <Router history={history}>
                <AppContent />
            </Router>
        </ContextProvider<HeadContext>>
    }
}

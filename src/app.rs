use yew::{function_component, html, Html, Properties};
use yew_router::{
    history::{AnyHistory, MemoryHistory},
    BrowserRouter, Routable, Router, Switch,
};

use crate::{components::layout::Layout, pages::Route};

#[function_component(AppContent)]
fn app_content() -> Html {
    html! {
        <Layout>
            <main>
                <Switch<Route> render={Route::switch} />
            </main>
        </Layout>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <AppContent />
        </BrowserRouter>
    }
}

#[derive(PartialEq, Properties)]
pub struct StaticAppProps {
    route: Route,
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
        <Router history={history}>
            <AppContent />
        </Router>
    }
}

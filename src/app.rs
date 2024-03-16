use yew::{function_component, html, Html};
use yew_router::{BrowserRouter, Switch};

use crate::pages::Route;

#[function_component(AppContent)]
fn app_content() -> Html {
    html! {
        <main>
            <Switch<Route> render={Route::switch} />
        </main>
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

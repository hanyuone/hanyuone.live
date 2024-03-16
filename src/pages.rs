use yew::{html, Html};
use yew_router::Routable;

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
}

impl Route {
    pub fn switch(self) -> Html {
        match self {
            Route::Home => html! { <h1>{"Home"}</h1> }
        }
    }
}

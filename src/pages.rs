use yew::{html, Html};
use yew_router::Routable;

mod about;
mod home;

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
}

impl Route {
    pub fn switch(self) -> Html {
        match self {
            Route::Home => html! { <home::Page /> },
            Route::About => html! { <about::Page /> },
        }
    }
}

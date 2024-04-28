use enum_iterator::Sequence;
use yew::{html, Html};
use yew_router::Routable;

mod about;
mod home;

#[derive(Debug, Clone, PartialEq, Routable, Sequence)]
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

use enum_iterator::Sequence;
use markdown::structs::blog::BlogId;
use yew::{html, Html};
use yew_router::Routable;

mod about;
mod blog;
mod blog_post;
mod home;

#[derive(Clone, PartialEq, Routable, Sequence)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/blog")]
    Blog,
    #[at("/blog/:blog_id")]
    BlogPost { blog_id: BlogId },
}

impl Route {
    pub fn switch(self) -> Html {
        match self {
            Route::Home => html! { <home::Page /> },
            Route::About => html! { <about::Page /> },
            Route::Blog => html! { <blog::Page /> },
            Route::BlogPost { blog_id } => html! { <blog_post::Page {blog_id} /> },
        }
    }
}

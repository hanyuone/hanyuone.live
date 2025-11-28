use enum_iterator::Sequence;
use markdown::structs::{blog::BlogId, tag::TagId};
use yew::{html, Html};
use yew_router::Routable;

mod blog;
mod blog_post;
mod home;
mod tag;

#[derive(Clone, PartialEq, Routable, Sequence)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/blog")]
    Blog,
    #[at("/blog/:blog_id")]
    BlogPost { blog_id: BlogId },
    #[at("/blog/tag/:tag_id")]
    Tag { tag_id: TagId },
}

impl Route {
    pub fn switch(self) -> Html {
        match self {
            Route::Home => html! { <home::Page /> },
            Route::Blog => html! { <blog::Page /> },
            Route::BlogPost { blog_id } => html! { <blog_post::Page {blog_id} /> },
            Route::Tag { tag_id } => html! { <tag::Page {tag_id} /> },
        }
    }
}

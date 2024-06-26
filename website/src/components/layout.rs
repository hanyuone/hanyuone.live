use yew::{function_component, html, Children, Html, Properties};

mod footer;
mod header;

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    html! {
        <div class="bg-black-dark text-white flex flex-col h-screen justify-between">
            <header::Header />
            <main>
                {props.children.clone()}
            </main>
            <footer::Footer />
        </div>
    }
}

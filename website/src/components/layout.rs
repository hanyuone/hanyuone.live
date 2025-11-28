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
        <div class="bg-black text-white flex flex-col min-h-screen justify-between">
            <header::Header />
            <main class="grow p-20">
                {props.children.clone()}
            </main>
            <footer::Footer />
        </div>
    }
}

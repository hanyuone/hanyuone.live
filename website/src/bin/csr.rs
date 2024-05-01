use website::app::App;

fn main() {
    let app = yew::Renderer::<App>::new();
    
    #[cfg(feature = "hydration")]
    app.hydrate();

    #[cfg(not(feature = "hydration"))]
    app.render();
}

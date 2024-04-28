use std::{
    env, io, path::{Path, PathBuf}
};

use website::{
    app::{StaticApp, StaticAppProps},
    components::head::{HeadContext, HeadRender, HeadRenderProps},
    pages::Route,
};
use yew::LocalServerRenderer;
use yew_router::Routable;

/// CURRENT PLAN OF RENDERING ALL ROUTES (modified from blakerain.com):
/// 1. Get default template from `dist` folder
/// 2. Render head and body
/// 3. Inject head and body into template at specified locations:
///   - Between `head-ssg-before` and `head-ssg-after` for head
///   - At end of `body` tag for body

struct Template {
    content: String,
    head_index: usize,
    body_index: usize,
}

impl Template {
    async fn load(path: impl AsRef<Path>) -> io::Result<Self> {
        let content = tokio::fs::read_to_string(path).await?;
        println!("{}", content);

        let Some(head_index) = content.find("<script id=head-ssg-after") else {
            return Err(io::Error::new(io::ErrorKind::Other, "Malformed index.html"));
        };

        let Some(body_index) = content.find("</body>") else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Malformed index.html",
            ));
        };

        Ok(Self {
            content,
            head_index,
            body_index,
        })
    }

    async fn render(&self, head: String, body: String) -> String {
        if head.is_empty() {
            eprintln!("warning: empty <head>");
        }

        if body.is_empty() {
            eprintln!("warning: empty <body>");
        }

        let mut result = String::with_capacity(self.content.len());
        result.push_str(&self.content[..self.head_index]);
        result.push_str(&head);
        result.push_str(&self.content[self.head_index..self.body_index]);
        result.push_str(&body);
        result.push_str(&self.content[self.body_index..]);
        result
    }
}

struct Env {
    template: Template,
    target_dir: PathBuf,
}

impl Env {
    async fn new() -> io::Result<Self> {
        let target_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("dist");
        let template = Template::load(target_dir.join("index.html")).await?;

        Ok(Self {
            template,
            target_dir,
        })
    }

    async fn render_route(&self, route: Route) -> String {
        let head = HeadContext::default();

        let render = {
            let head = head.clone();
            LocalServerRenderer::<StaticApp>::with_props(StaticAppProps { route, head })
        };

        let mut body = String::new();
        render.render_to_string(&mut body).await;

        let render =
            LocalServerRenderer::<HeadRender>::with_props(HeadRenderProps { context: head });

        let mut head = String::new();
        render.render_to_string(&mut head).await;

        self.template.render(head, body).await
    }

    async fn write_str<P: AsRef<Path>>(&self, path: P, s: &str) -> std::io::Result<()> {
        let path = self.target_dir.clone().join(path);

        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        tokio::fs::write(path, s).await
    }
}

struct RouteTarget {
    route: Route,
    target: PathBuf,
}

fn collect_routes() -> Vec<RouteTarget> {
    enum_iterator::all::<Route>()
        .map(|route| {
            let path = route.to_path();

            let target = if path == "/" {
                PathBuf::from("index.html")
            } else {
                PathBuf::from(&path[1..]).with_extension("html")
            };

            RouteTarget { route, target }
        })
        .collect()
}

async fn render_routes(env: &Env) -> io::Result<()> {
    for RouteTarget { route, target } in collect_routes() {
        let result = env.render_route(route).await;
        env.write_str(target, &result).await?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let env = Env::new().await?;

    render_routes(&env).await?;

    Ok(())
}

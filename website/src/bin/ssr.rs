use std::{
    env, io,
    path::{Path, PathBuf},
};

use website::{
    app::{StaticApp, StaticAppProps},
    components::head::{HeadRender, HeadRenderProps},
    context::{BlogContext, HeadContext},
    pages::Route,
};
use yew::LocalServerRenderer;
use yew_router::Routable;

/// Basic struct for converting a HTML file into a "template" - a place where
/// we can inject tags in `<head>` and `<body>`.
struct Template {
    content: String,
    head_index: usize,
    body_index: usize,
}

impl Template {
    /// Converts the path of an HTML file (usually our `index.html`) into a
    /// `Template`. Errors when there is no `head-ssg-after` (for injecting
    /// `<head>` tags into) or `</body>` (for injecting `<body>` tags into).
    async fn load(path: impl AsRef<Path>) -> io::Result<Self> {
        let content = tokio::fs::read_to_string(path).await?;
        eprintln!("{}", content);

        let Some(head_index) = content.find("<script id=head-ssg-after") else {
            return Err(io::Error::new(io::ErrorKind::Other, "Malformed index.html: no head"));
        };

        let Some(body_index) = content.find("</body>") else {
            return Err(io::Error::new(io::ErrorKind::Other, "Malformed index.html: no body"));
        };

        Ok(Self {
            content,
            head_index,
            body_index,
        })
    }

    /// Injects HTML into the head and body.
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

/// A "holder" for the `index.html` template and the directory where generated
/// HTML files should go.
struct Env {
    target_dir: PathBuf,
    template: Template,
    blog_context: BlogContext,
}

impl Env {
    async fn new() -> io::Result<Self> {
        let target_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../dist");
        let template = Template::load(target_dir.join("index.html")).await?;

        let raw_blog_context = tokio::fs::read(target_dir.join("public/blog/blog_cards")).await?;
        let blog_context = BlogContext::new(&raw_blog_context);

        Ok(Self {
            target_dir,
            template,
            blog_context,
        })
    }

    /// Render a route using `LocalServerRenderer`.
    async fn render_route(&self, route: Route) -> String {
        let head = HeadContext::default();

        let render = {
            let head = head.clone();
            LocalServerRenderer::<StaticApp>::with_props(StaticAppProps { route, head, blog: self.blog_context.clone() })
        };

        let mut body = String::new();
        render.render_to_string(&mut body).await;

        let render =
            LocalServerRenderer::<HeadRender>::with_props(HeadRenderProps { context: head });

        let mut head = String::new();
        render.render_to_string(&mut head).await;

        self.template.render(head, body).await
    }

    /// Write a string to a certain path within `self.target_dir`.
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

/// Maps all existing routes, adding extra information about the HTML file
/// location the rendered HTML for that route should be stored (in the `RouteTarget`
/// struct).
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

/// Statically render all routes into plain HTML files, with routes defined
/// in the `Route` enum.
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

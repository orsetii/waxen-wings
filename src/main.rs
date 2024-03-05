use std::io;

use askama::Template;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod docs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "waxen_wings=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("initializing router...");
    let assets_path = std::env::current_dir()?;

    let api_router = Router::new()
        .route("/hello", get(hello_api))
        .route("/content/md/:name", get(docs::md_api));

    let router = Router::new()
        .nest("/api", api_router)
        .route("/", get(index))
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
        );

    info!("router initialized, now listening on port 3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await?;
    Ok(())
}

async fn hello_api() -> &'static str {
    "Hello!"
}

async fn index() -> impl IntoResponse {
    let template = IndexTemplate::new().unwrap();
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    categories: Vec<String>,
    page_content: String,
}
impl IndexTemplate {
    pub fn new() -> io::Result<Self> {
        let mut cats = Vec::new();
        for entry in std::fs::read_dir(format!(
            "{}/docs",
            std::env::current_dir()?.to_str().unwrap()
        ))? {
            let path = entry?.path();
            if path.is_dir() {
                cats.push(path.file_name().unwrap().to_string_lossy().to_string());
            }
        }

        Ok(Self {
            categories: cats,
            page_content: String::from_utf8(std::fs::read(format!(
                "{}/docs/home/index.md",
                std::env::current_dir().unwrap().to_str().unwrap()
            ))?)
            .unwrap(),
        })
    }
}

/// A wrapper type that we'll use to encapsulate HTML parsed by askama into valid HTML for axum to serve.
struct HtmlTemplate<T>(T);

/// Allows us to convert Askama HTML templates into valid HTML for axum to serve in the response.
impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        // Attempt to render the template with askama
        match self.0.render() {
            // If we're able to successfully parse and aggregate the template, serve it
            Ok(html) => Html(html).into_response(),
            // If we're not, return an error or some bit of fallback HTML
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}

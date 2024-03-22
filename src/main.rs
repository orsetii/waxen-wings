use std::io;

use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
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

    let router = Router::new()
        .nest("/api", api::router())
        .route("/", get(index))
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("router initialized, now listening on port 3000");
    axum::serve(listener, router).await?;
    Ok(())
}

async fn index() -> impl IntoResponse {
    let template = IndexTemplate::new().unwrap();
    HtmlTemplate(template)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CategoryType {
    Doc,
    OtherHtml,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Category {
    pub name: String,
    pub cat_type: CategoryType,
}

impl Category {
    pub fn new_doc(name: &'static str) -> Self {
        Self {
            name: name.to_string(),
            cat_type: CategoryType::Doc,
        }
    }
    pub fn new_other_html(name: &'static str) -> Self {
        Self {
            name: name.to_string(),
            cat_type: CategoryType::OtherHtml,
        }
    }
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    page_content: String,
}
impl IndexTemplate {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            page_content: String::from_utf8(std::fs::read(format!(
                "{}/docs/welcome.md",
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

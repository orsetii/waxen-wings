use askama::Template;
use axum::{response::IntoResponse, routing::get, Router};

pub mod christianity;
pub mod islam;
pub mod other;

pub fn router() -> Router {
    Router::new().route("/", get(home))
}

#[derive(Template)]
#[template(path = "theology.html")]
struct TheologyHomeTemplate {}

pub async fn home() -> impl IntoResponse {
    let template = TheologyHomeTemplate {};
    crate::HtmlTemplate(template)
}

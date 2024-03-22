use askama::Template;
use axum::response::IntoResponse;

#[derive(Template)]
#[template(path = "icarus.html")]
struct IcarusHomeTemplate {}

pub async fn home() -> impl IntoResponse {
    let template = IcarusHomeTemplate {};
    crate::HtmlTemplate(template)
}

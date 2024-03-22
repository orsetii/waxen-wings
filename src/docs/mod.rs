async fn md_api(name: &str) -> String {
    let mut path = name.to_string();
    if !path.contains(".md") {
        path += "/index.md"
    }
    info!("Reading '{}'", path);
    String::from_utf8(std::fs::read(format!("{}/{}", get_doc_folder(), path)).unwrap()).unwrap()
}

fn get_doc_folder() -> String {
    format!(
        "{}/docs",
        std::env::current_dir().unwrap().to_str().unwrap()
    )
}
use askama::Template;
use axum::{extract::Path, response::IntoResponse};
use tracing::info;

#[derive(Template)]
#[template(path = "text_page.html")]
struct MdContentTemplate {
    md_text: String,
}
pub async fn md_content(Path(name): Path<String>) -> impl IntoResponse {
    let template = MdContentTemplate {
        md_text: md_api(&name).await,
    };
    crate::HtmlTemplate(template)
}

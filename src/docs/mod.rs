use axum::extract::Path;

pub async fn md_api(Path(mut name): Path<String>) -> String {
    if !name.contains(".md") {
        name += "/index.md"
    }
    String::from_utf8(std::fs::read(format!("{}/{}", get_doc_folder(), name)).unwrap()).unwrap()
}

fn get_doc_folder() -> String {
    format!(
        "{}/docs",
        std::env::current_dir().unwrap().to_str().unwrap()
    )
}

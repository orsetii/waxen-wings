use axum::{routing::get, Router};

pub mod icarus;
pub mod theology;

pub fn router() -> Router {
    Router::new()
        .route("/content/md/:name", get(super::docs::md_content))
        .route("/content/icarus/", get(icarus::home))
        .nest("/content/theology/", theology::router())
}

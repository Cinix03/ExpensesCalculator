use axum::{routing::get, Router, response::IntoResponse};

pub async fn root_handler() -> impl IntoResponse {
    "Hello, World!"
}

pub async fn about_handler() -> impl IntoResponse {
    "About page"
}

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(root_handler))
        .route("/about", get(about_handler))
}
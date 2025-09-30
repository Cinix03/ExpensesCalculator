use axum::{Router, response::IntoResponse, routing::get, routing::post};

pub async fn root_handler() -> impl IntoResponse {
    "Hello, World!"
}

pub async fn create_user() -> impl IntoResponse {
    "Hey"
}

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(root_handler))
        .route("/about", post(create_user))
}

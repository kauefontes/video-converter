use axum::{Router, routing::get};
use tower_http::limit::RequestBodyLimitLayer;

use crate::routes::{show_hello, upload_image, upload_video};

pub fn create_app() -> Router {
    Router::new()
        .route("/", get(show_hello).post(upload_video))
        .route("/upload_image", get(show_hello).post(upload_image))
        .layer(axum::extract::DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(250 * 1024 * 1024)) // 250mb
        .layer(tower_http::trace::TraceLayer::new_for_http())
}
use axum::response::Redirect;
use axum::{routing::get, Router};
use tower_http::limit::RequestBodyLimitLayer;

use crate::routes::{redirect_amorporsp, upload_image, upload_video};

pub fn create_app() -> Router {
    Router::new()
        .route(
            "/upload_video",
            get(|| async { Redirect::permanent(redirect_amorporsp().await) }).post(upload_video),
        )
        .route(
            "/upload_image",
            get(|| async { Redirect::permanent(redirect_amorporsp().await) }).post(upload_image),
        )
        .layer(axum::extract::DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(250 * 1024 * 1024)) // 250mb
        .layer(tower_http::trace::TraceLayer::new_for_http())
}


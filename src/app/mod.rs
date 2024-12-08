use axum::response::Redirect;
use axum::{routing::get, Router};
use tower_http::limit::RequestBodyLimitLayer;
use axum::extract::Path;
use axum::response::IntoResponse;

use crate::routes::{redirect_amorporsp, upload_image, upload_video};

async fn get_id_handler(Path(id): Path<u32>) -> impl IntoResponse {
    format!("Id: {}", id)
}

pub fn create_app() -> Router {
    Router::new()
        .route(
            "/upload_video",
            get(get_id_handler).post(upload_video),
        )
        .route(
            "/upload_image",
            get(get_id_handler).post(upload_image),
        )
        .layer(axum::extract::DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(250 * 1024 * 1024)) // 250mb
        .layer(tower_http::trace::TraceLayer::new_for_http())
}
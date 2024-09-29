mod video_converter;

use axum::{
    extract::{DefaultBodyLimit, Multipart},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use tower_http::limit::RequestBodyLimitLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use video_converter::convert_to_apple_compatible;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/", get(show_form).post(accept_form))
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(
            250 * 1024 * 1024, // 250mb
        ))
        .layer(tower_http::trace::TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn show_form() -> Html<&'static str> {
    Html(
        r#"
      <!doctype html>
      <html>
          <head></head>
          <body>
              <form action="/" method="post" enctype="multipart/form-data">
                  <label>
                      Upload file:
                      <input type="file" name="file">
                  </label>

                  <input type="submit" value="Upload file">
              </form>
          </body>
      </html>
      "#,
    )
}

async fn accept_form(mut multipart: Multipart) -> impl IntoResponse {
    let mut input_path = String::new();
    let mut output_path = String::new();

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        if name == "file" {
            let filename = field.file_name().unwrap().to_string();
            input_path = format!("/tmp/{}", filename);

            let path = Path::new(&input_path);
            let stem = path.file_stem().unwrap().to_str().unwrap();
            let extension = path.extension().unwrap().to_str().unwrap();
            output_path = format!("/tmp/{}_aac.{}", stem, extension);

            let mut file = File::create(&input_path).unwrap();
            let data = field.bytes().await.unwrap();
            file.write_all(&data).unwrap();
        }
    }

    match convert_to_apple_compatible(&input_path, &output_path).await {
        Ok(_) => {
            let mut file = File::open(&output_path).unwrap();
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).unwrap();
            (
                axum::http::StatusCode::OK,
                [
                    ("Content-Type", "video/mp4"),
                    (
                        "Content-Disposition",
                        format!(
                            "attachment; filename=\"{}\"",
                            Path::new(&output_path)
                                .file_name()
                                .unwrap()
                                .to_str()
                                .unwrap()
                        )
                        .as_str(),
                    ),
                ],
                buffer,
            )
                .into_response()
        }
        Err(_) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Erro na conversão".to_string(),
        )
            .into_response(),
    }
}

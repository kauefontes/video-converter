mod uploader;

use axum::{
    extract::Multipart,
    response::{Html, IntoResponse},
    http::StatusCode,
};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::video_converter::convert_to_apple_compatible;
use crate::image_optimizer::optimize_image;
use anyhow::Result;
use uploader::upload_to_s3;

pub async fn show_hello() -> Html<&'static str> {
    Html("<h1>Amor por São Paulo</h1>")
}

pub async fn upload_video(mut multipart: Multipart) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut input_path = String::new();
    let mut output_path = String::new();
    let mut presigned_url = String::new();

    while let Some(field) = multipart.next_field().await.map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))? {
        let name = field.name().unwrap().to_string();
        if name == "file" {
            let filename = field.file_name().unwrap().to_string();
            input_path = format!("/tmp/{}", filename);

            let path = Path::new(&input_path);
            let stem = path.file_stem().unwrap().to_str().unwrap();
            let extension = path.extension().unwrap().to_str().unwrap();
            output_path = format!("/tmp/{}_aac.{}", stem, extension);

            let mut file = File::create(&input_path).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            let data = field.bytes().await.map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
            file.write_all(&data).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        } else if name == "presigned_url" {
            presigned_url = field.text().await.map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
        }
    }

    convert_to_apple_compatible(&input_path, &output_path).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    upload_to_s3(&output_path, &presigned_url).await?;

    Ok((StatusCode::OK, "Arquivo enviado com sucesso".to_string()).into_response())
}

pub async fn upload_image(mut multipart: Multipart) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut input_path = String::new();
    let mut output_path = String::new();
    let mut presigned_url = String::new();

    while let Some(field) = multipart.next_field().await.map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))? {
        let name = field.name().ok_or((StatusCode::BAD_REQUEST, "Missing field name".to_string()))?.to_string();
        if name == "file" {
            let filename = field.file_name().ok_or((StatusCode::BAD_REQUEST, "Missing file name".to_string()))?.to_string();
            input_path = format!("/tmp/{}", filename);
            output_path = format!("/tmp/optimized_{}", filename);

            let mut file = File::create(&input_path).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            let data = field.bytes().await.map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
            file.write_all(&data).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        } else if name == "presigned_url" {
            presigned_url = field.text().await.map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
        }
    }

    optimize_image(&input_path, &output_path).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    upload_to_s3(&output_path, &presigned_url).await?;

    Ok((StatusCode::OK, "Arquivo enviado com sucesso".to_string()).into_response())
}
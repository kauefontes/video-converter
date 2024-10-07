use anyhow::Result;
use axum::http::StatusCode;
use reqwest::Client;
use std::fs::File;
use std::io::Read;

pub async fn upload_to_s3(
    output_path: &str,
    presigned_url: &str,
) -> Result<(), (StatusCode, String)> {
    let mut file =
        File::open(output_path).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let client = Client::new();
    let response = client
        .put(presigned_url)
        .body(buffer)
        .send()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Erro ao enviar arquivo para o S3".to_string(),
        ))
    }
}


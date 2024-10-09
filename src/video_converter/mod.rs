use std::process::Command;
use serde_json::Value;
use std::process::Stdio;
use ffmpeg_cli::{FfmpegBuilder, File, Parameter};
use futures::{future::ready, StreamExt};

pub async fn has_aac_codec(input: &str) -> Result<bool, Box<dyn std::error::Error>> {
    tracing::info!("Checking if the video has the aac codec");
    tracing::info!("Input: {}", input);
    let output = Command::new("ffprobe")
        .arg("-v")
        .arg("error")
        .arg("-select_streams")
        .arg("a:0")
        .arg("-show_entries")
        .arg("stream=codec_name")
        .arg("-of")
        .arg("json")
        .arg(input)
        .output()?;

    let stdout = String::from_utf8(output.stdout)?;
    let json: Value = serde_json::from_str(&stdout)?;

    if let Some(codec_name) = json["streams"][0]["codec_name"].as_str() {
        Ok(codec_name == "aac")
    } else {
        Ok(false)
    }
}

pub async fn convert_to_apple_compatible(
    input: &str,
    output: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let builder = FfmpegBuilder::new()
        .stderr(Stdio::piped())
        .option(Parameter::Single("nostdin"))
        .option(Parameter::Single("y"))
        .input(File::new(input))
        .output(
            File::new(output)
                .option(Parameter::KeyValue("vcodec", "libx264"))
                .option(Parameter::KeyValue("preset", "ultrafast"))
                .option(Parameter::KeyValue("crf", "24"))
                .option(Parameter::KeyValue("acodec", "aac"))
                .option(Parameter::KeyValue("b:a", "128k"))
                .option(Parameter::KeyValue("movflags", "+faststart")),
        );

    let ffmpeg = builder.run().await.expect("Erro ao executar ffmpeg");

    ffmpeg
        .progress
        .for_each(|x| {
            match dbg!(x) {
                Ok(_) => (),
                Err(e) => eprintln!("Error: {:?}", e),
            }
            ready(())
        })
        .await;

    let output_result = ffmpeg
        .process
        .wait_with_output()
        .expect("Erro ao esperar resultado do ffmpeg");

    if output_result.status.success() {
        println!(
            "{}\nstderr:\n{}",
            output_result.status,
            std::str::from_utf8(&output_result.stderr).expect("Erro ao converter")
        );
        tracing::debug!("Output: {}", output);
        Ok(output.to_string())
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Erro na conversão",
        )))
    }
}

pub async fn process_video(input: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    if has_aac_codec(input).await? {
        println!("The video already has the aac codec.");
    } else {
        convert_to_apple_compatible(input, output).await?;
    }
    Ok(())
}
use ffmpeg_cli::{FfmpegBuilder, File, Parameter};
use futures::{future::ready, StreamExt};
use std::process::Stdio;

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
                .option(Parameter::KeyValue("crf", "23"))
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
        Ok(output.to_string())
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Erro na conversão",
        )))
    }
}

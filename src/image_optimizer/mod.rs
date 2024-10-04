use oxipng::{optimize, InFile, OutFile, Options};
use std::path::Path;

pub async fn optimize_image(input_path: &str, output_path: &str) -> Result<(), String> {
    let options = Options::from_preset(3);
    let input = InFile::Path(Path::new(input_path).to_path_buf());
    let output = OutFile::Path {
        path: Some(Path::new(output_path).to_path_buf()),
        preserve_attrs: false,
    };

    optimize(&input, &output, &options)
        .map_err(|e| e.to_string())
}
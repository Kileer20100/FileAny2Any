use std::path::Path;
use image::{ImageFormat};

pub fn convert_bmp(fileInput: &str,filename: &str) {

    let output = format!("{}.bmp", filename);

    let img = image::open(&Path::new(fileInput)).expect("Failed to open image");

    img.save_with_format(output, ImageFormat::Bmp).expect("Failed to save image");

}
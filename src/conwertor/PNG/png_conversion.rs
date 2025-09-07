use std::path::Path;
use image::{ImageFormat};

pub fn convert_png(fileInput: &str,filename: &str) {

    let output = format!("{}.png", filename);

    let img = image::open(&Path::new(fileInput)).expect("Failed to open image");

    img.save_with_format(output, ImageFormat::Png).expect("Failed to save image");

}
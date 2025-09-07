use std::path::Path;
use image::{ImageFormat};

pub fn convert_jpg(fileInput: &str,filename: &str) {
    let output_path = "./output";

    let output = format!("{}/{}.jpg",output_path, filename);

    let img = image::open(&Path::new(fileInput)).expect("Failed to open image");

    img.save_with_format(output, ImageFormat::Jpeg).expect("Failed to save image");

}
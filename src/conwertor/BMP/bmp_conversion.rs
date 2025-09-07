use std::path::Path;
use image::{ImageFormat};

pub fn convert_bmp(fileInput: &str,filename: &str) {
    let output_path = "./output";

    let output = format!("{}/{}.bmp",output_path, filename);


    let img = image::open(&Path::new(fileInput)).expect("Failed to open image");

    img.save_with_format(output, ImageFormat::Bmp).expect("Failed to save image");

}
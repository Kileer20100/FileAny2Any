use std::path::Path;
use image::{ImageFormat};

pub fn convert_png(fileInput: &str,filename: &str, output_path: &str) {

    let output = format!("{}/{}.png",output_path, filename);

    let img = image::open(&Path::new(fileInput)).expect("Failed to open image");

    img.save_with_format(output, ImageFormat::Png).expect("Failed to save image");

    println!("Конвертация в PNG завершена:{}/{}",output_path, filename);
}
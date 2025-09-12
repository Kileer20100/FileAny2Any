use std::path::Path;
use image::{ImageFormat};
use crate::global::global_state::{update_stream_count};

pub fn convert_jpg(file_input: &str,filename: &str, output_path: &str) {

    let output = format!("{}/{}.jpg",output_path, filename);

    let img = image::open(&Path::new(file_input)).expect("Failed to open image");

    img.save_with_format(output, ImageFormat::Jpeg).expect("Failed to save image");

    update_stream_count(1 as u32);

    println!("Конвертация в JPG завершена:{}/{}", output_path, filename);
}
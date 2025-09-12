use std::path::Path;
use image::{ImageFormat};
use crate::global::global_state::{update_stream_count};

pub fn convert_bmp(file_input: &str,filename: &str, output_path: &str) {

    let output = format!("{}/{}.bmp",output_path, filename);


    let img = image::open(&Path::new(file_input)).expect("Failed to open image");

    img.save_with_format(output, ImageFormat::Bmp).expect("Failed to save image");

    update_stream_count(1 as u32);


    println!("Конвертация в BMP завершена: {}/{}", output_path, filename);

}
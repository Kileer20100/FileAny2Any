use std::path::Path;
//use std::thread;

use crate::conwertor::bmp::bmp_conversion::convert_bmp;
use crate::conwertor::jpg::jpg_conversion::convert_jpg;
use crate::conwertor::png::png_conversion::convert_png;


pub fn currency_exchange(input_file: String, output_path: String, expansion: String){


    let stem: String = {

        Path::new(input_file.as_str())
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()

    };

    if expansion == "PNG" {
        convert_png(input_file.as_str(), stem.as_str(), output_path.as_str());
    } else if expansion == "BMP" {
        convert_bmp(input_file.as_str(), stem.as_str(), output_path.as_str());
    } else if expansion == "JPG" {
        convert_jpg(input_file.as_str(), stem.as_str(), output_path.as_str());
    } else {
        println!("Unsupported file format: {}", expansion);
    }

}

/*pub fn currency_exchange(input_file: String, output_path: String, expansion: String) {

    let input: Vec<String> = Vec::new();


    let mut vec = input;

    vec.push(input_file.to_string());



    let stem: String = {

        Path::new(&vec[0].as_str())
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()

    };


    let theard_1 = thread::spawn({
        let vec1 = vec.clone();
        let stem1 = stem.clone();
        let output_path = output_path.clone();
        move||convert_png(&vec1[0].as_str(), &stem1, output_path.as_str())
    });

    let theard_2 = thread::spawn({
        let vec1 = vec.clone();
        let stem1 = stem.clone();
        let output_path = output_path.clone();
        move||convert_bmp(&vec1[0].as_str(), &stem1, output_path.as_str())
    });

    let theard_3 = thread::spawn({
        let vec1 = vec.clone();
        let stem1 = stem.clone();
        let output_path = output_path.clone();
        move||convert_jpg(&vec1[0].as_str(), &stem1, output_path.as_str())
    });

    theard_1.join().unwrap();
    theard_2.join().unwrap();
    theard_3.join().unwrap();
}*/
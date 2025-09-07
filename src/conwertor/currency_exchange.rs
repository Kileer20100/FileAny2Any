use std::path::Path;
use std::thread;

use crate::conwertor::BMP::bmp_conversion::convert_bmp;
use crate::conwertor::JPG::jpg_conversion::convert_jpg;
use crate::conwertor::PNG::png_conversion::convert_png;

pub fn currency_exchange() {
        let input: Vec<String> = Vec::new();


    let mut vec = input;
    vec.push("input.jpg".to_string());



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
        move||convert_png(&vec1[0].as_str(), &stem1)
    });

    let theard_2 = thread::spawn({
        let vec1 = vec.clone();
        let stem1 = stem.clone();
        move||convert_bmp(&vec1[0].as_str(), &stem1)
    });

    let theard_3 = thread::spawn({
        let vec1 = vec.clone();
        let stem1 = stem.clone();
        move||convert_jpg(&vec1[0].as_str(), &stem1)
    });

    theard_1.join().unwrap();
    theard_2.join().unwrap();
    theard_3.join().unwrap();
}
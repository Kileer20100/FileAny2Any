use std::ffi::{CStr};
use std::os::raw::c_char;
use rayon::{prelude::*};

mod conwertor;
mod dir;
mod net;
mod global;

use crate::conwertor::currency_exchange::currency_exchange;
use crate::dir::search::serch_dir::serch_dir;
use crate::global::global_state::{ update_max, update_status, TCP_STATUS, TCP_STREAM, TCP_MAX};


#[unsafe(no_mangle)]
pub extern "C" fn StartConwert(
    input_path: *const c_char,
    output_path: *const c_char,
    expansion: *const c_char
) -> u8 {
    // Преобразуем C строки в Rust &str
    let input_path = unsafe {
        if input_path.is_null() { return 0; }
        CStr::from_ptr(input_path).to_str().unwrap_or("")
    };
    let output_path = unsafe {
        if output_path.is_null() { return 0; }
        CStr::from_ptr(output_path).to_str().unwrap_or("")
    };
    let expansion = unsafe {
        if expansion.is_null() { return 0; }
        CStr::from_ptr(expansion).to_str().unwrap_or("")
    };

    update_status(true);

    println!("Input Path: {}", input_path);
    println!("Output Path: {}", output_path);
    println!("Expansion: {}", expansion);

    println!("{}", TCP_STATUS.lock().unwrap());

    // Поиск файлов и директорий
    let (file_list, _dir_list) = match serch_dir(input_path.to_string()) {
        Ok(res) => res,
        Err(_) => return 0,
    };

    update_max(file_list.len() as u32);


    // Обработка файлов параллельно
    file_list.par_iter().for_each(|i| {
        currency_exchange(i.clone(), output_path.to_string(), expansion.to_string());
    });

    1 // Успешно
}


#[unsafe(no_mangle)]
pub extern "C" fn InfoMAXProcess() -> u32{
    let tcp_max = TCP_MAX.lock().unwrap();
    tcp_max.clone()
}

#[unsafe(no_mangle)]
pub extern "C" fn InfoStreamProcess() -> u32{
    let tcp_stream = TCP_STREAM.lock().unwrap();
    tcp_stream.clone()
}

#[unsafe(no_mangle)]
pub extern "C" fn InfoProcessTSP() -> bool{
    let tcp_status = TCP_STATUS.lock().unwrap();
    tcp_status.clone()
}
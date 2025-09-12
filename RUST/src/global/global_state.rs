use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TCP_MAX: Mutex<u32> = Mutex::new(0);
    pub static ref TCP_STREAM: Mutex<u32> = Mutex::new(0);
    pub static ref TCP_STATUS: Mutex<bool> = Mutex::new(false);
}


pub fn update_stream_count( count: u32 ){
    let mut tcp_sream = TCP_STREAM.lock().unwrap();
    *tcp_sream += count;

}
pub fn update_max( status_max_count: u32 ){
    let mut tcp_max = TCP_MAX.lock().unwrap();
    *tcp_max = status_max_count;
}

pub fn update_status( status: bool ){
    let mut tcp_status = TCP_STATUS.lock().unwrap();
    *tcp_status = status;

}

use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TCP_MAX: Mutex<u32> = Mutex::new(0);
    pub static ref TCP_STREAM: Mutex<u32> = Mutex::new(0);
    pub static ref TCP_STATUS: Mutex<bool> = Mutex::new(false);
}

#[repr(C)]
struct InfoProcess {
    max_count: u32,
    stream: u32,
    status: bool,
}


pub fn UpdateStreamCount( count: u32 ){
    let mut tcpSream = TCP_STREAM.lock().unwrap();
    *tcpSream = count;

}
pub fn UpdateMAX( status: u32 ){
    let mut tcpMAX = TCP_MAX.lock().unwrap();
    *tcpMAX = status;
}

pub fn UpdateStatus( status: bool ){
    let mut tcpStatus = TCP_STATUS.lock().unwrap();
    *tcpStatus = status;

}

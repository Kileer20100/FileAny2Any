use std::net::TcpStream;
use std::io::Write;
use rayon::str::ParallelString;
use serde_json::json;

pub fn send_progress(value: i32, message: &str) -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;

    // Формируем JSON-объект
    let payload = json!({
        "value": value,
        "message": message
    });

    // Превращаем в строку и добавляем перевод строки (важно!)
    let msg = format!("{}\n", payload.to_string());

    stream.write_all(msg.as_bytes())?;
    Ok(())
}

pub fn send_progress_start(value: i32, message: &str) -> std::io::Result<()> {

    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    // Отправляем сообщение и закрываем соединение
    stream.write_all("Hi Rust".as_bytes())?;

    println!("Connected to server, sending progress...");

    Ok(())
}
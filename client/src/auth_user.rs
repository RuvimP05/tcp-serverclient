use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::net::TcpStream;
pub fn authenticate_user(stream: &mut TcpStream) {
    let file: File = File::open("./key").expect("Failed to open key file");
    let mut key: String = "".to_string();
    BufReader::new(file)
        .read_to_string(&mut key)
        .expect("Failed to read key");

    stream
        .write_all(key.trim().as_bytes())
        .expect("Failed to write to server");

    let mut auth_buf: [u8; 1] = [0; 1];
    let size: usize = stream
        .read(&mut auth_buf)
        .expect("could not recieve authentication from server");
    let auth_status: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&auth_buf[..size]);
    if auth_status != "1" {
        println!("Could not authenticate");
        return;
    }
}

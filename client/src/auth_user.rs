use native_tls::TlsStream;
use std::fs::File;
use std::io::{self, BufReader, Read, Write};
use std::net::TcpStream;

type EncryptedStream = TlsStream<TcpStream>;

pub fn authenticate_user(stream: &mut EncryptedStream) {
    let file: File = File::open("./key").expect("Could not open key file");
    let mut key: String = "".to_string();
    let _ = BufReader::new(file)
        .read_to_string(&mut key)
        .map_err(|err: io::Error| eprintln!("Could not read key. Error: {}", err));

    let _ = stream
        .write_all(key.trim().as_bytes())
        .map_err(|err: io::Error| {
            eprintln!("Could not write to server. Error: {}", err);
        });

    let mut auth_buf: [u8; 1024] = [0; 1024];
    let size: usize = stream.read(&mut auth_buf).unwrap_or_else(|err: io::Error| {
        eprintln!("Could not read authentication packet. Error: {}", err);
        0
    });
    // println!("{:?}", auth_buf);
    let auth_string: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&auth_buf[..size]);
    if auth_string.as_ref() != "1" {
        println!("Could not authenticate");
        std::process::exit(3);
    }
}

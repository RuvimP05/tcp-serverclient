use native_tls::TlsStream;
use std::{borrow::Cow, io::Read, net::TcpStream};

type EncryptedStream = TlsStream<TcpStream>;

pub fn get_username(stream: &mut EncryptedStream) -> String {
    let mut buffer: Vec<u8> = vec![0; 16];
    let name: String = match stream.read(&mut buffer) {
        Ok(size) => {
            if size == 0 {
                // End of stream, connection closed by the client
                println!("Client disconnected.");
                return String::from("");
            }
            let name: Cow<'_, str> = String::from_utf8_lossy(&buffer);
            name.to_string()
        }
        Err(err) => {
            eprintln!("Error reading from client: {}", err);
            String::from("")
        }
    };
    name
}

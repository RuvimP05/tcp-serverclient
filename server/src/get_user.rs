use std::{borrow::Cow, io::Read, net::TcpStream};

pub fn get_username(stream: &mut TcpStream) -> String {
    let mut buffer: Vec<u8> = vec![0; 16];
    let name = match stream.read(&mut buffer) {
        Ok(size) => {
            if size == 0 {
                // End of stream, connection closed by the client
                println!("Client disconnected.");
                return String::from("");
            }
            let name: Cow<'_, str> = String::from_utf8_lossy(&buffer);
            name.to_string()
        }
        Err(e) => {
            eprintln!("Error reading from client: {}", e);
            "".to_owned()
        }
    };
    name
}

use std::fs;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};

fn authenticate_key(key: &str) -> bool {
    let correct_key: String = fs::read_to_string("./key").unwrap();
    if key == correct_key {
        true
    } else {
        false
    }
}
fn key_len() -> usize {
    let key: String = fs::read_to_string("./key").unwrap();
    key.len()
}

fn authenticate_client(stream: &mut TcpStream, user: SocketAddr) {
    println!("incoming connection from {:?}", user);
    let mut buffer: Vec<u8> = vec![0; key_len()];
    match stream.read(&mut buffer) {
        Ok(size) => {
            if size == 0 {
                // End of stream, connection closed by the client
                println!("Client disconnected.");
                return;
            }
        }
        Err(e) => {
            eprintln!("Error reading from client: {}", e);
            return;
        }
    }
    let key: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer);
    let auth: bool = authenticate_key(key.trim());
    if auth == false {
        let auth_buf: &[u8] = "0".as_bytes();
        stream.write_all(auth_buf).unwrap();
        return;
    }
    let auth_buf: &[u8] = "1".as_bytes();
    stream.write_all(auth_buf).unwrap();
    println!("user {} has been authenticated", user);
}

fn handle_client(mut stream: TcpStream) {
    let user: SocketAddr = stream.peer_addr().unwrap();
    //AUTHENTICATION
    authenticate_client(&mut stream, user);
    //MAIN LOOP
    let mut buffer: [u8; 1024] = [0; 1024];
    loop {
        // Read data from the client
        match stream.read(&mut buffer) {
            Ok(size) => {
                if size == 0 {
                    // End of stream, connection closed by the client
                    println!("Client disconnected.");
                    break;
                }
                let message: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer);
                print!("user {} inputted: {}", user, message);

                //send response to keep client active
                stream.write_all(&[0]).unwrap();
            }
            Err(e) => {
                eprintln!("Error reading from client: {}", e);
                break;
            }
        }
        buffer = [0; 1024];
    }
}

fn main() {
    // Bind the server to a specific address and port
    let listener: TcpListener =
        TcpListener::bind("0.0.0.0:6969").expect("Failed to bind to address");

    println!("Server listening on port 6969");

    // Accept incoming connections and process them in a loop
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Spawn a new thread to handle each incoming connection
                std::thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}

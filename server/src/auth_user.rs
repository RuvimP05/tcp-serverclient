use native_tls::TlsStream;
use std::error::Error;
use std::fs;
use std::io::{Read, Write};
use std::net::SocketAddr;

use crate::get_user;

type Result<T> = std::result::Result<T, Box<dyn Error>>;
type EncryptedStream = TlsStream<std::net::TcpStream>;

fn authenticate_key(key: &str) -> Result<bool> {
    let correct_key: String = fs::read_to_string("./key")
        .map_err(|err| format!("couldn't read file into string. Error: {}", err))?;
    Ok(key == correct_key.trim())
}

pub fn key_len() -> Result<usize> {
    let key: String = fs::read_to_string("./key")
        .map_err(|err| format!("couldn't read file into string. Error: {}", err))?;
    Ok(key.trim().len())
}

pub fn authenticate_client(stream: &mut EncryptedStream, peer: SocketAddr) -> Result<String> {
    println!("Incoming connection from {:?}", peer);

    let key_length: usize =
        key_len().map_err(|err| format!("couldn't get length of key. Error: {:?}", err))?;

    let mut buffer: Vec<u8> = vec![0; key_length];

    match stream.read(&mut buffer) {
        Ok(size) => {
            if size == 0 {
                // End of stream, connection closed by the client
                println!("Client disconnected.");
                return Ok(String::new());
            }
        }
        Err(err) => {
            eprintln!("Error reading from client: {}", err);
            return Err(err.into());
        }
    }

    let key = String::from_utf8_lossy(&buffer).trim().to_string();
    let auth_status = authenticate_key(&key)?;

    if !auth_status {
        // Tell client auth was not successful
        let auth_buf: &[u8] = b"0";
        if let Err(err) = stream.write_all(auth_buf) {
            eprintln!(
                "Could not send failed authentication packet to IP {}. Error: {}",
                peer, err
            );
        }
        return Err(format!("failed to authenticate user from {}", peer).into());
    }

    // Send confirmation to client
    let auth_buf: &[u8] = b"1";
    if let Err(err) = stream.write_all(auth_buf) {
        eprintln!(
            "Could not send successful authentication packet to IP {}. Error: {}",
            peer, err
        );
    }

    // Get username from client
    let name = get_user::get_username(stream);
    println!("User {} has been authenticated", name);
    Ok(name)
}

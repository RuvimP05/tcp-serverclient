use std::fs;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};

use crate::get_user;
type Result<T> = std::result::Result<T, ()>;

fn authenticate_key(key: &str) -> Result<bool> {
    let correct_key: String = fs::read_to_string("./key")
        .map_err(|err| eprintln!("couldn't read file into string. Error: {}", err))?;
    if key == correct_key {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn key_len() -> Result<usize> {
    let key: String = fs::read_to_string("./key")
        .map_err(|err| eprintln!("couldn't read file into string. Error: {}", err))?;
    Ok(key.len())
}

pub fn authenticate_client(stream: &mut TcpStream, user: SocketAddr) -> Result<String> {
    println!("incoming connection from {:?}", user);
    let mut buffer: Vec<u8> =
        vec![0; key_len().map_err(|err| eprintln!("couldnt get length of key. Error: {:?}", err))?];
    match stream.read(&mut buffer) {
        Ok(size) => {
            if size == 0 {
                // End of stream, connection closed by the client
                println!("Client disconnected.");
                return Ok(String::from(""));
            }
        }
        Err(e) => {
            eprintln!("Error reading from client: {}", e);
            return Err(());
        }
    }
    let key: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer);
    let auth: bool = authenticate_key(key.trim())?;
    if auth == false {
        let auth_buf: &[u8] = "0".as_bytes();
        let _ = stream.write_all(auth_buf).map_err(|err| {
            eprintln!(
                "could not send failed authentication packet to ip {}. Error: {}",
                user, err
            )
        });
        return Err(());
    }
    let auth_buf: &[u8] = "1".as_bytes();
    let _ = stream.write_all(auth_buf).map_err(|err| {
        eprintln!(
            "could not send successful authentication packet to ip {}. Error: {}",
            user, err
        )
    });
    let name = get_user::get_username(stream);
    println!("user {} has been authenticated", name);
    Ok(name)
}
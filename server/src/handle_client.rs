use crate::auth_user;
use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
};

type Result<T> = std::result::Result<T, ()>;

pub fn handle_client(mut stream: TcpStream) -> Result<()> {
    let user: SocketAddr = stream
        .peer_addr()
        .map_err(|err| eprintln!("could not obtain peer address. Error: {}", err))?;
    //AUTHENTICATION
    let name: String = auth_user::authenticate_client(&mut stream, user)?;
    //MAIN LOOP
    let mut buffer: [u8; 1024] = [0; 1024];
    loop {
        // Read data from the client
        match stream.read(&mut buffer) {
            Ok(size) => {
                if size == 0 {
                    // End of stream, connection closed by the client
                    println!("Client {} disconnected.", name);
                    break;
                }
                let message: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer);
                print!("user {} inputted: {}", name, message);

                //send response to keep client active
                let _ = stream.write_all(&[0]).map_err(|err| {
                    eprintln!("Could not send response to client {}. Error: {}", name, err)
                });
            }
            Err(e) => {
                eprintln!("Error reading from client: {}", e);
                break;
            }
        }
        buffer = [0; 1024];
    }
    Ok(())
}

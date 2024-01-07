use native_tls::{TlsAcceptor, TlsStream};

use crate::auth_user;
use std::{
    io::{self, Read, Write},
    net::{SocketAddr, TcpStream},
};

type Result<T> = std::result::Result<T, ()>;
type EncryptedStream = TlsStream<TcpStream>;

pub fn handle_client(stream: TcpStream, acceptor: TlsAcceptor) -> Result<()> {
    let mut stream: EncryptedStream = match acceptor.accept(stream) {
        Ok(stream) => stream,
        Err(err) => {
            eprintln!("Error accepting TLS connection: {:?}", err);
            return Err(());
        }
    };
    let peer: SocketAddr = stream.get_ref().peer_addr().map_err(|err: io::Error| {
        eprintln!("could not obtain peer address. Error: {}", err);
    })?;
    //AUTHENTICATION
    let name: String = auth_user::authenticate_client(&mut stream, peer)?;

    //MAIN LOOP
    let mut buffer: [u8; 1024] = [0; 1024];
    loop {
        // Read data from the client
        match stream.read(&mut buffer) {
            Ok(size) => {
                if size == 0 {
                    // End of stream, connection closed by the client
                    println!("{} disconnected.", name);
                    break;
                }
                let message: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer);
                print!("user {} inputted: {}", name, message);

                //send response to keep client active
                let _ = stream
                    .write_all(message.as_bytes())
                    .map_err(|err: io::Error| {
                        eprintln!("Could not send response to {}. Error: {}", name, err)
                    });
            }
            Err(err) => {
                eprintln!("Error reading from {}. Error: {}", name, err);
                break;
            }
        }
        buffer = [0; 1024];
    }
    Ok(())
}

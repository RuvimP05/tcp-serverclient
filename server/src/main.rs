use native_tls::{Identity, TlsAcceptor};
use std::io;
use std::net::TcpListener;
use std::result;

mod auth_user;
mod get_user;
mod handle_client;

type Result<T> = result::Result<T, ()>;

fn main() -> Result<()> {
    // Bind the server to a specific address and port
    let listener: TcpListener = TcpListener::bind("0.0.0.0:6969")
        .map_err(|err: io::Error| eprintln!("ERROR: could not bind to address: {}", err))?;
    println!("Server listening on port 6969");

    // Accept incoming connections and process in a loop
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Spawn a new thread to handle each incoming connection
                std::thread::spawn(|| {
                    let identity: Identity =
                        Identity::from_pkcs12(include_bytes!("certificate.p12"), "1234567890")
                            .expect("Failed to load PKCS12 file");
                    let acceptor: TlsAcceptor =
                        TlsAcceptor::new(identity).expect("Failed to create TLS acceptor");
                    let _ = handle_client::handle_client(stream, acceptor)
                        .map_err(|err| eprintln!("Failed to handle client. Error: {:?}", err));
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
    Ok(())
}

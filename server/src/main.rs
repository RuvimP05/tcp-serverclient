use native_tls::{Identity, TlsAcceptor};
use std::io;
use std::net::TcpListener;

mod auth_user;
mod get_user;
mod handle_client;

type Result<T> = std::result::Result<T, ()>;

fn main() -> Result<()> {
    //Bind the server to a specific address and port
    let listener: TcpListener = TcpListener::bind("0.0.0.0:6969")
        .map_err(|err: io::Error| eprintln!("Failed to bind to address. Error: {}", err))?;
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
            Err(err) => {
                eprintln!("Failed accepting connection. Error: {}", err);
            }
        }
    }
    Ok(())
}

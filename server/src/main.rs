use std::net::TcpListener;
use std::result;

mod auth_user;
mod get_user;
mod handle_client;

type Result<T> = result::Result<T, ()>;

fn main() -> Result<()> {
    // Bind the server to a specific address and port
    let listener: TcpListener = TcpListener::bind("0.0.0.0:6969")
        .map_err(|err| eprintln!("ERROR: could not bind to address: {}", err))?;

    println!("Server listening on port 6969");

    // Accept incoming connections and process in a loop
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Spawn a new thread to handle each incoming connection
                std::thread::spawn(|| {
                    let _ = handle_client::handle_client(stream)
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

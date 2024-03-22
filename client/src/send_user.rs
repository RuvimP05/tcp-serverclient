use native_tls::TlsStream;
use std::io::{self, stdin, Write};
use std::net::TcpStream;

use crate::{clear_terminal, truncate};

type EncryptedStream = TlsStream<TcpStream>;

pub fn send_name(stream: &mut EncryptedStream) {
    let mut name: String = String::new();

    let _ = io::stdout()
        .flush()
        .map_err(|err| eprintln!("Could not flush io. Error: {}", err));
    print!("Enter Username (max: 16 characters): ");
    let _ = io::stdout()
        .flush()
        .map_err(|err: io::Error| eprintln!("Could not flush io. Error: {}", err));

    //input name
    stdin()
        .read_line(&mut name)
        .unwrap_or_else(|err: io::Error| {
            eprintln!("Could not read line. Error: {}", err);
            0
        });

    clear_terminal();

    println!("Welcome, {}", truncate(name.clone(), 16));
    //Cut name to length and send to server
    let name: String = truncate(name, 16);
    let _ = stream
        .write_all(name.trim().as_bytes())
        .map_err(|err: io::Error| eprintln!("Could not send name to server. Error: {}", err));
}

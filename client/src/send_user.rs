use crate::{clear_terminal, truncate};
use std::io::{self, stdin, Write};
use std::net::TcpStream;

pub fn send_name(stream: &mut TcpStream) {
    let mut name: String = String::new();

    let _ = io::stdout()
        .flush()
        .map_err(|err| eprintln!("couldn't flush io. Error: {}", err));
    print!("Enter Username (max: 16 characters): ");
    let _ = io::stdout()
        .flush()
        .map_err(|err| eprintln!("couldn't flush io. Error: {}", err));

    //input name
    stdin().read_line(&mut name).expect("Failed to read line");

    clear_terminal();

    println!("Welcome, {}", truncate(name.clone(), 16));
    //Cut name to length and send to server
    let name: String = truncate(name, 16);
    stream
        .write_all(name.trim().as_bytes())
        .expect("Failed to write to server");
}

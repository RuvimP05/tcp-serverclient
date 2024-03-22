use crossterm::{
    cursor, execute,
    terminal::{Clear, ClearType},
};
use native_tls::{Certificate, TlsConnector, TlsStream};
use std::io::{self, stdin, Read, Write};
use std::net::TcpStream;

mod auth_user;
mod send_user;

type EncryptedStream = TlsStream<TcpStream>;

fn clear_terminal() {
    let _ = execute!(io::stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0))
        .map_err(|err: io::Error| eprintln!("Couldn't clear terminal. Error: {}", err));
}

fn truncate(s: String, max_width: usize) -> String {
    s.chars().take(max_width).collect()
}

fn main() {
    clear_terminal();
    let arg: Vec<String> = std::env::args().collect();
    if arg.len() != 2 {
        eprintln!("Usage: ./client <IP.ADDR:PORT>")
    }
    let certificate: Certificate = match Certificate::from_pem(include_bytes!("cert.pem")) {
        Ok(cert) => cert,
        Err(err) => {
            eprintln!("Could not generate certificate. Error: {}", err);
            std::process::exit(1);
        }
    };
    // Connect to the server
    let connector: TlsConnector = match TlsConnector::builder()
        .add_root_certificate(certificate)
        .build()
    {
        Ok(connector) => connector,
        Err(err) => {
            eprintln!("could not build connector. Error: {}", err);
            std::process::exit(2);
        }
    };
    let stream: TcpStream = TcpStream::connect(&arg[1]).expect("Failed to connect");
    let mut stream: EncryptedStream = connector.connect("Ruvimp05", stream).expect("");
    auth_user::authenticate_user(&mut stream);
    send_user::send_name(&mut stream);
    // Read user input and send it to the server
    loop {
        let mut input: String = String::new();

        print!("Enter a message (or 'exit' to quit): ");
        let _ = io::stdout()
            .flush()
            .map_err(|err: io::Error| eprintln!("Could not flush io. Error: {}", err));

        stdin().read_line(&mut input).expect("Failed to read line");

        if input.trim() == "exit" {
            clear_terminal();
            println!("Exiting the client.");
            break;
        }

        clear_terminal();

        input = truncate(input, 1024);
        // Send user input to the server
        let _ = stream
            .write_all(input.as_bytes())
            .map_err(|err: io::Error| {
                eprintln!("Could not write user input to stream. Error: {}", err)
            });

        // Read the response from the server
        let mut buffer: [u8; 1024] = [0; 1024];
        let size: usize = stream.read(&mut buffer).unwrap_or_else(|err: io::Error| {
            eprintln!("Could not read stream from server. Error: {}", err);
            0
        });
        println!("response: {}", String::from_utf8_lossy(&buffer[..size]));
    }
}

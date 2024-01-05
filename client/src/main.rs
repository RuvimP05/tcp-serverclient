use crossterm::{
    cursor, execute,
    terminal::{Clear, ClearType},
};
use std::fs::File;
use std::io::{self, stdin, BufReader, Read, Write};
use std::net::TcpStream;
fn clear_terminal() {
    execute!(io::stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
}

fn truncate(s: String, max_width: usize) -> String {
    s.chars().take(max_width).collect()
}

fn authenticate_user(stream: &mut TcpStream) {
    let file: File = File::open("./key").expect("Failed to open key file");
    let mut key: String = "".to_string();
    BufReader::new(file)
        .read_to_string(&mut key)
        .expect("Failed to read key");

    stream
        .write_all(key.trim().as_bytes())
        .expect("Failed to write to server");

    let mut auth_buf: [u8; 1] = [0; 1];
    let size: usize = stream
        .read(&mut auth_buf)
        .expect("could not recieve authentication from server");
    let auth_status: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&auth_buf[..size]);
    if auth_status != "1" {
        println!("Could not authenticate");
        return;
    }
}
fn main() {
    clear_terminal();
    let arg: Vec<String> = std::env::args().collect();
    if arg.len() != 2 {
        eprintln!("Usage: client.exe <IP.ADDR:PORT>")
    }
    // Connect to the server
    let mut stream: TcpStream = TcpStream::connect(&arg[1]).expect("Failed to connect to server");

    authenticate_user(&mut stream);
    // Read user input and send it to the server
    loop {
        let mut input: String = String::new();

        print!("Enter a message (or 'exit' to quit): ");
        io::stdout().flush().unwrap();

        stdin().read_line(&mut input).expect("Failed to read line");

        if input.trim() == "exit" {
            clear_terminal();
            println!("Exiting the client.");
            break;
        }

        clear_terminal();

        input = truncate(input, 1024);
        // Send user input to the server
        stream
            .write_all(input.as_bytes())
            .expect("Failed to write to server");

        // Read the response from the server
        let mut buffer: [u8; 1024] = [0; 1024];
        stream
            .read(&mut buffer)
            .expect("Failed to read from server");
    }
}

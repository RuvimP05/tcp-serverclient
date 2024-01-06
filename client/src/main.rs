use crossterm::{
    cursor, execute,
    terminal::{Clear, ClearType},
};
use std::io::{self, stdin, Read, Write};
use std::net::TcpStream;

mod auth_user;
mod send_user;

fn clear_terminal() {
    let _ = execute!(io::stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0))
        .map_err(|err| eprintln!("Couldn't clear terminal. Error: {}", err));
}

fn truncate(s: String, max_width: usize) -> String {
    s.chars().take(max_width).collect()
}

fn main() {
    clear_terminal();
    let arg: Vec<String> = std::env::args().collect();
    if arg.len() != 2 {
        eprintln!("Usage: client.exe <IP.ADDR:PORT>")
    }
    // Connect to the server
    let mut stream: TcpStream = TcpStream::connect(&arg[1]).expect("Failed to connect to server");

    auth_user::authenticate_user(&mut stream);
    send_user::send_name(&mut stream);
    // Read user input and send it to the server
    loop {
        let mut input: String = String::new();

        print!("Enter a message (or 'exit' to quit): ");
        let _ = io::stdout()
            .flush()
            .map_err(|err| eprintln!("couldn't flush io. Error: {}", err));

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

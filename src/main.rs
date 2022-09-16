use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::Read;
use std::io::Write;
use std::str::from_utf8;

fn handle_client(mut stream: TcpStream) {
    let mut buffer: [u8; 500] = [0; 500];

    let client_ip_addr = stream.peer_addr().unwrap();

    println!("Handling client {client_ip_addr}");

    loop {
        if !stream.read(&mut buffer).is_ok() {
            println!("Could not read into buffer - disconnecting");
            return;
        }

        let original_string = from_utf8(&buffer).unwrap();
        println!("Received {original_string}");

        let uppercase_string = original_string.to_uppercase();

        if uppercase_string[0..7].eq("GOODBYE") {
            stream.write("Goodbye!!!".as_bytes());
            println!("Disconnecting client");
            stream.shutdown(Shutdown::Both);
            return;
        }

        if !stream.write(uppercase_string.as_bytes()).is_ok() {
            println!("Could not send back to client");
            return;
        }
        println!("Sent back {uppercase_string}");

        // Clear out the buffer so that it's ready for the next message
        buffer = [0; 500];
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("localhost:15000")?;

    for stream in listener.incoming() {
        handle_client(stream?);
    }

    Ok(())
}

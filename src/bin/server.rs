use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};


fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut file = File::create("received_file.txt")?;
    let mut buffer = [0; 1024];

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => break, // Connection closed
            Ok(bytes_read) => {
                println!("Received {} bytes", bytes_read);
                file.write_all(&buffer[..bytes_read])?;
            }
            Err(e) => {
                eprintln!("Failed to read from socket: {}", e);
                break;
            }
        }
    }
    
         println!("File received successfully.");
         match stream.write_all(b"Transfer complete") {
             Ok(_) => println!("Acknowledgment sent to client."),
             Err(e) => eprintln!("Failed to send acknowledgment: {}", e),
         }
    Ok(())
}


fn start_server() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Server listening on port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                
                //Spawn a new thread to handle the client
                std::thread::spawn(move || {
                    if let Err(e) = handle_client(stream) {
                        eprintln!("Error handling client: {}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
    Ok(())
}

fn main() {
    if let Err(e) = start_server() {
        eprintln!("Server error: {}", e);
    }
}
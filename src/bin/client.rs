use std::fs::File;
use std::io::{ Read, Write};
use std::net::TcpStream;
use std::io::stdin;

fn get_file_path() -> String {
    println!("Enter the path of the file to send:");
    let mut file_path = String::new();
    stdin().read_line(&mut file_path).expect("Failed to read line");
    file_path.trim().to_string()
}

fn send_file(file_path: &str) -> std::io::Result<()> {
    let mut file = File::open(file_path)?;
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    println!("Connected to server.");
    let mut buffer = [0; 1024];

    loop {
        let bytes_read = match file.read(&mut buffer) {
            Ok(0) => break, // EOF reached
            Ok(n) => n,
            Err(e) => {
                eprintln!("Failed to read from file: {}", e);
                return Err(e);
            }
        };
        if bytes_read == 0 {
            break; // EOF reached
        }
        stream.write_all(&buffer[..bytes_read])?;
        println!("Sent {} bytes", bytes_read);
    }
    stream.shutdown(std::net::Shutdown::Write)?;
    println!("File sent successfully.");
    let mut ack_buffer = [0; 1024];
    match stream.read(&mut ack_buffer) {
        Ok(bytes_read) => {
            let ack_message = String::from_utf8_lossy(&ack_buffer[..bytes_read]);
            println!("Received acknowledgment from server: {}", ack_message);
        }
        Err(e) => {
            eprintln!("Failed to read acknowledgment from server: {}", e);
        }
    }
    Ok(())
}

fn main() {
    let file_path = get_file_path();
    if let Err(e) = send_file(&file_path) {
        eprintln!("Error sending file: {}", e);
    }
}
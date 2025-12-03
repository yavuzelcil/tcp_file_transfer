# TCP File Transfer

A simple TCP-based file transfer application written in Rust that allows sending files between a server and client over a local network.

## Features

- **Server**: Receives files from clients and saves them locally
- **Client**: Sends files to the server
- Efficient binary data transfer over TCP
- Simple command-line interface

## Usage

### Start the Server

```bash
cargo run --bin server
```

The server will listen on `127.0.0.1:7878` for incoming file transfers.

### Send a File

```bash
cargo run --bin client
```

The client will send `src/bin/temp.txt` to the server.

## Project Structure

```
src/
  bin/
    server.rs    # Server implementation
    client.rs    # Client implementation
    temp.txt     # Example file to transfer
  main.rs        # Main entry point
```

## TODO

- [ ] Add command-line arguments for custom file paths
- [ ] Support for multiple file transfers in a single session
- [ ] Progress bar for large file transfers
- [ ] Error handling improvements
- [ ] Encryption support for secure file transfer
- [ ] Configuration file for server address and port
- [ ] Resume capability for interrupted transfers
- [ ] Directory transfer support
- [ ] Compression before transfer
- [ ] GUI interface option

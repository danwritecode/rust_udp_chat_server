# Rust UDP Chat

A simple, for-fun UDP client-server application written in Rust. This project emphasizes using Rust's standard library for network communication without external dependencies. It allows multiple clients to join a chat and broadcast messages to each other in real-time.

## Features

- **UDP Communication**: Utilizes UDP for low-latency, connectionless communication between clients and server.
- **Concurrent Message Broadcasting**: Leverages Rust's threading and synchronization primitives to handle concurrent message broadcasts to connected clients.
- **Dynamic Client Management**: Automatically adds new clients to the broadcast list and manages them throughout the session.

## Prerequisites

Before running the application, ensure you have Rust installed on your system. Visit [the official Rust website](https://www.rust-lang.org/tools/install) for installation instructions.

## Setup

1. **Clone the Repository**

```bash
 git clone <repository-url>
 cd rust-udp-chat
```

2. **Configuration**

   Currently, the application requires manually setting the destination address ('DEST_ADDR') in the code. This is the address and port the server listens on, e.g., '"127.0.0.1:34254"'. In future versions, this will be configurable via an environment variable.

3. **Building the Project**

   Compile the project with Rust's package manager, Cargo.

```bash
cargo build --release
```

## Running the Application

To start the application, you can choose to run it in either server or client mode.

### Server Mode

1. Edit the 'DEST_ADDR' in the source code to the address you want the server to listen on.
2. Run the server: `cargo run`

   When prompted, select mode 1 for server mode.

### Client Mode

1. Ensure 'DEST_ADDR' in the source code matches the server's listening address.
2. Run the client: `cargo run`

   When prompted, select mode 2 for client mode. Enter your username and start chatting.

## Future Enhancements

- **Environment Variables for Configuration**: Planning to allow setting the server address and port through environment variables for ease of use.
- **Improved Client Management**: Enhancements for better handling of client connections and disconnections.

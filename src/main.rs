use std::collections::HashMap;
use std::net::UdpSocket;
use std::io;
use std::sync::Arc;
use std::thread;

const DEST_ADDR:&str = "";
const CLIENT_ADDR:&str = "0.0.0.0:0";

fn init_server_mode() -> std::io::Result<()> {
    println!("Now in server broadcast mode");
    let mut clients = HashMap::new();

    let socket = UdpSocket::bind(DEST_ADDR)?;

    loop {
        let mut buf = [0; 10000];
        let (amt, src) = socket.recv_from(&mut buf)?;

        clients.entry(src.ip()).or_insert(src);
        let buf = &mut buf[..amt];

        println!(">> {:?}", String::from_utf8(buf.to_vec()).unwrap());

        for (ip, c) in &clients {
            println!("Broadcasting to ip: {}", ip);
            // only send the response to the clients that didn't send the message
            if c != &src {
                socket.send_to(buf, c)?;
            }
        }
    }
}

fn init_client_mode() -> std::io::Result<()> {
    let socket = Arc::new(UdpSocket::bind(CLIENT_ADDR)?);
    let socket_ptr = socket.clone();

    thread::spawn(move || {
        init_client_listener(socket_ptr).unwrap();
    });

    loop {
        let input = client_input();
        socket.send_to(&input, DEST_ADDR)?;
    }
}

fn init_client_listener(socket: Arc<UdpSocket>) -> std::io::Result<()> {
    loop {
        let mut buf = [0; 10000];
        let (amt, _src) = socket.recv_from(&mut buf)?;

        let buf = &mut buf[..amt];

        println!(">> {:?}", String::from_utf8(buf.to_vec()).unwrap());
    }
}

fn client_input() -> Vec<u8> {
    println!("Message input: ");
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer).unwrap();

    let buffer = buffer.trim();

    buffer.to_string().into_bytes()
}

fn initiate_cli() -> i32 {
    println!("Select mode: 1 for server, 2 for client");
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer).unwrap();

    buffer.trim().parse::<i32>().unwrap()
}

fn main() -> std::io::Result<()> {
    match initiate_cli() {
        1 => init_server_mode().unwrap(),
        2 => init_client_mode().unwrap(),
        _ => unimplemented!("Unsupported mode activiated")
    }

    Ok(())
}


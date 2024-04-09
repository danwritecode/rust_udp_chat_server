use std::collections::HashMap;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use std::thread;


pub struct ToipServer {
    server_address: String
}

impl ToipServer {
    pub fn new(server_address: String) -> Self {
        ToipServer { server_address }
    }

    pub fn init(&self) -> std::io::Result<()> {
        let mut clients = HashMap::new();
        let socket = UdpSocket::bind(&self.server_address)?;

        loop {
            let mut buf = [0; 10000];
            let (amt, src) = socket.recv_from(&mut buf)?;

            let client_key = format!("{}:{}", src.ip(), src.port());
            clients.entry(client_key).or_insert(src);

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
}

#[derive(Clone)]
pub struct Message {
    pub username: String,
    pub content: String,
}

pub struct ToipClient {
    server_address: String,
    pub username: String,
    pub message_buffer: Arc<Mutex<Vec<Message>>>,
    socket: Arc<UdpSocket>
}

impl ToipClient {
    pub fn new(server_address: String, username: String) -> Self {
        let message_buffer = Arc::new(Mutex::new(vec![]));
        let socket = match UdpSocket::bind("0.0.0.0:0") {
            Ok(s) => s,
            Err(e) => panic!("Error occured when binding to udp socket: {}", e)
        };
        let socket = Arc::new(socket);

        ToipClient { server_address, username, message_buffer, socket }
    }

    /// Sends join message and also spawns thread to listen to incoming messages and
    /// adds any incoming messages to the message buffer
    pub fn init(&self) -> std::io::Result<()> {
        // send join message
        let join_message = format!("{}<$|>Has joined the channel", &self.username);
        self.socket.send_to(join_message.as_bytes(), &self.server_address)?;

        let socket_ptr = self.socket.clone();
        let messages_ptr = self.message_buffer.clone();

        thread::spawn(move || {
            ToipClient::init_client_listener(socket_ptr, messages_ptr).unwrap();
        });

        Ok(())
    }

    /// Adds message to the message buffer and also sends to the server address
    pub fn send_message(&self, message: String) -> std::io::Result<()> {
        let mut messages = self.message_buffer.lock().unwrap();
        messages.push(Message { username: self.username.clone(), content: message.clone() });
        
        let message_string = format!("{}<$|>{}", self.username, message);
        self.socket.send_to(&message_string.as_bytes(), self.server_address.clone())?;

        Ok(())
    }

    // inits the client listener for us in a separate thread
    fn init_client_listener(socket: Arc<UdpSocket>, messages: Arc<Mutex<Vec<Message>>>) -> std::io::Result<()> {
        loop {
            let mut buf = [0; 10000];
            let (amt, _src) = socket.recv_from(&mut buf)?;

            let buf = &mut buf[..amt];
            let message_parts = String::from_utf8(buf.to_vec())
                .unwrap()
                .split("<$|>")
                .map(|p| p.to_string())
                .take(2)
                .collect::<Vec<String>>();

            let username = &message_parts[0];
            let content = &message_parts[1];

            let mut messages = messages.lock().unwrap();
            messages.push(Message { username: username.clone(), content: content.clone() });
        }
    }
}

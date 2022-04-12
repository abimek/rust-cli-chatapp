use packets::{Connection, MessageSendPacket, Packet, PacketId};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::net::{TcpListener, TcpStream};

#[allow(dead_code)]

struct User {
    server: Arc<Mutex<Server>>,
    username: Option<String>,
    connection: Connection,
}

impl User {
    fn new(server: Arc<Mutex<Server>>, stream: TcpStream) -> Self {
        User {
            server,
            username: None,
            connection: Connection::new(stream),
        }
    }

    async fn read_packets(&mut self) {
        loop {
            if let Ok((packet_id, data)) = self.connection.read_packet().await {
                if let Some(i) = PacketId::from_u64(packet_id) {
                    match i {
                        PacketId::MessageSend => {
                            if let Ok(packet) = MessageSendPacket::from_string(&data) {
                                let lock = self.server.lock();
                                if let Ok(server) = lock {
                                    (*server).broadcast_message(packet).await;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    async fn send_message(&mut self, packet: MessageSendPacket) {
        //TODO Come Back Here Tomorrow Morning
        self.connection.write_packet(packet).await.expect("idk");
    }
}

struct Server {
    users: HashMap<String, User>,
}

impl Server {
    fn new() -> Self {
        Server {
            users: HashMap::new(),
        }
    }

    pub async fn broadcast_message(&self, data: MessageSendPacket) {}
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:3030").await.unwrap();
    let server = Arc::new(Mutex::new(Server::new()));

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let server_a = Arc::clone(&server);
        tokio::spawn(async move {
            handle_client(server_a, socket).await;
        });
    }
}

async fn handle_client(server: Arc<Mutex<Server>>, stream: TcpStream) {
    let mut user = User::new(server, stream);
}

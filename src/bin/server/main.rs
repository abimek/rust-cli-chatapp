use futures::lock::Mutex;
use packets::{Connection, MessageSendPacket, Packet, PacketId};
use std::{collections::HashMap, sync::Arc};
use tokio::net::{TcpListener, TcpStream};

struct User {
    server: Arc<Mutex<Server>>,
    //username: Option<String>,
    connection: Connection,
}

impl User {
    fn new(server: Arc<Mutex<Server>>, stream: TcpStream) -> Self {
        User {
            server,
            //       username: None,
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
                                let mut lock = self.server.lock().await;
                                lock.broadcast_message(packet).await;
                            }
                        }
                    }
                }
            }
        }
    }
    async fn send_message(&mut self, packet: MessageSendPacket) {
        self.connection.write_packet(packet).await.unwrap();
    }
}

struct Server {
    users: HashMap<u64, User>,
    total_count: u64,
}

impl Server {
    fn new() -> Self {
        Server {
            users: HashMap::new(),
            total_count: 0,
        }
    }

    pub async fn broadcast_message(&mut self, data: MessageSendPacket) {
        for (_, user) in self.users.iter_mut() {
            user.send_message(data.clone()).await;
        }
    }
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
    let server_2 = Arc::clone(&server);
    let mut user = User::new(server, stream);
    user.read_packets().await;
    let mut underlying = server_2.lock().await;
    let n = underlying.total_count;
    underlying.users.insert(n + 1, user);
    underlying.total_count = underlying.total_count + 1;
}

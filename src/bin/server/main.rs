use packets::{Connection, MessageSendPacket, Packet, PacketId};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::broadcast::{self, Receiver, Sender},
};
struct User {
    connection: Connection,
}

impl User {
    fn new(stream: TcpStream) -> Self {
        User {
            connection: Connection::new(stream),
        }
    }

    async fn run(&mut self, tran_recv: (Sender<PacketData>, Receiver<PacketData>)) {
        let (tx, mut rx) = tran_recv;
        loop {
            tokio::select! {
                Ok((packet_id, data)) = self.connection.read_packet() => {
                    if let Some(i) = PacketId::from_u64(packet_id) {
                        match i {
                            PacketId::MessageSend => {
                                tx.send(PacketData::new(i, data)).unwrap();
                            }
                        }
                    }
                },
                Ok(pk_data) = rx.recv() => {
                    match pk_data.packet_id {
                        PacketId::MessageSend => {
                            self.connection.write_packet(MessageSendPacket::from_string(&pk_data.data).unwrap()).await.unwrap();
                        },
                    }
                }
            }
        }
    }
}

/*struct Server {
    users: HashMap<u64, User>,
    total_count: u64,
}*/

/*impl Server {
    fn new() -> Self {
        Server {
            users: HashMap::new(),
            total_count: 0,
        }
    }
}*/

#[derive(Clone, Debug)]
struct PacketData {
    packet_id: PacketId,
    data: String,
}

impl PacketData {
    fn new(id: PacketId, data: String) -> Self {
        PacketData {
            packet_id: id,
            data,
        }
    }
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:5030").await.unwrap();
    //  let server = Arc::new(Mutex::new(Server::new()));
    let (tx, _rx) = broadcast::channel::<PacketData>(30);
    println!("Server Started!");
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        println!("new Connection");
        let rx1 = tx.subscribe();
        let tx1 = tx.clone();
        //     let server_a = Arc::clone(&server);
        tokio::spawn(async move {
            handle_client(socket, (tx1, rx1)).await;
        });
    }
}

async fn handle_client(stream: TcpStream, tran_recv: (Sender<PacketData>, Receiver<PacketData>)) {
    let mut user = User::new(stream);
    user.run(tran_recv).await;
}

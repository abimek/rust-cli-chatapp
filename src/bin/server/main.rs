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
                    println!("Reading");
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
                            let pk = MessageSendPacket::from_string(&pk_data.data).unwrap();
                            println!("Writing: {}: {}", pk.sender, pk.message);
                            self.connection.write_packet(pk).await.unwrap();
                        },
                    }
                }
            }
        }
    }
}

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
    let (tx, _rx) = broadcast::channel::<PacketData>(30);
    println!("Server Started!");
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        println!("new Connection");
        let rx1 = tx.subscribe();
        let tx1 = tx.clone();
        tokio::spawn(async move {
            handle_client(socket, (tx1, rx1)).await;
        });
    }
}

async fn handle_client(stream: TcpStream, tran_recv: (Sender<PacketData>, Receiver<PacketData>)) {
    let mut user = User::new(stream);
    user.run(tran_recv).await;
}

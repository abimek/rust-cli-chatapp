use std::io::{stdin, stdout, Write};

use async_std::io::WriteExt;
use packets::{Connection, MessageSendPacket, Packet, PacketId};
use tokio::net::TcpStream;

struct User {
    username: String,
    connection: Connection,
}

impl User {
    async fn run(&mut self) {
        println!("Write Something");
        loop {
            let mut line = String::new();
            let std = async_std::io::stdin();
            tokio::select! {
                Ok((pk_id, pk_data)) = self.connection.read_packet() => {
                    if let Some(id) = PacketId::from_u64(pk_id) {
                        match id {
                            PacketId::MessageSend => {
                                if let Ok(pk) = MessageSendPacket::from_string(&pk_data) {
                                    //TEMPROARY FIX OF PRITNING TO SELF UNTIL I CAN GET A LOGIN
                                    //SEQUENCE OPPERTIONAL
                                    if pk.sender != self.username {
                                        stdout().flush().unwrap();
                                        async_std::io::stdout().write_all(format!("{}: {}", pk.sender, pk.message).as_bytes());
                                    }
                                }
                            }
                        }
                    }
                },
                Ok(_) = std.read_line(&mut line) => {
                    self.connection.write_packet(MessageSendPacket::new(self.username.clone(), line)).await.unwrap();
                },
            }
        }
    }
}

#[tokio::main]
async fn main() {
    println!("Please type in your username");
    let mut username = String::new();
    stdin()
        .read_line(&mut username)
        .expect("error while readig line");
    username.pop();
    let stream = TcpStream::connect("127.0.0.1:5030").await.unwrap();

    let mut user = User {
        username,
        connection: Connection::new(stream),
    };

    user.run().await;
}

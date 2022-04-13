use std::io::stdin;

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
            tokio::select! {
                line = User::read_line() => {
                    self.connection.write_packet(MessageSendPacket::new(self.username.clone(), line)).await.unwrap();
                },
                Ok((pk_id, pk_data)) = self.connection.read_packet() => {
                    if let Some(id) = PacketId::from_u64(pk_id) {
                        match id {
                            PacketId::MessageSend => {
                                if let Ok(pk) = MessageSendPacket::from_string(&pk_data) {
                                    //TEMPROARY FIX OF PRITNING TO SELF UNTIL I CAN GET A LOGIN
                                    //SEQUENCE OPPERTIONAL
                                    if pk.sender != self.username {
                                        println!("{}: {}", pk.sender, pk.message);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    async fn read_line() -> String {
        loop {
            let mut data = String::new();
            stdin()
                .read_line(&mut data)
                .expect("Error while reading data from terminal");
            data.pop();
            if !data.contains("\n") | data.eq("") {
                return data;
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
    let mut address: String = String::new();
    println!("Please input a server address in the format <ip>:<port>");
    stdin()
        .read_line(&mut address)
        .expect("error while reading ip");

    let stream = TcpStream::connect("127.0.0.1:5030").await.unwrap();

    let mut user = User {
        username,
        connection: Connection::new(stream),
    };

    user.run().await;
}

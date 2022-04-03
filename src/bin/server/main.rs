use std::net::{SocketAddr, TcpListener, TcpStream};
use std::collections::HashMap;
use std::io::{Read, Write};

use packets::MessageSendPacket;

struct User {
    username: String,
    stream: TcpStream,
}

impl User {

    fn new(tcp_stream: TcpStream) -> Self {
        //TODO Read the Username from Stream
        let username = String::from("bob");
        User{
            username,
            stream: tcp_stream
        }
    }
    
    fn handle_packets(&mut self, server: &mut Server) {
        loop {
            let mut data = [0;4];
            self.stream.read_exact(&mut data).expect("Error reading packet, server shutting down...");
            let mut bytes_to_read = vec![0; i32::from_be_bytes(data.clone()) as usize];
            self.stream.read_exact(&mut bytes_to_read).expect("Error reading packet, server shutting down...");
            let mut s = String::from_utf8(bytes_to_read).unwrap();
            let mut packet = MessageSendPacket::from_string(&s).unwrap();
            server.broadcast_message(packet);
        }
    }

}

//SERVER CODE
struct Server {
    users: Vec<User>
}

impl Server {
    fn new() -> Self {
        Server {
            users: Vec::new()
        }
    }

    fn broadcast_message(&mut self, packet: MessageSendPacket){
        let str = packet.to_string().unwrap();
        let bytes = str.as_bytes();

        let length = (bytes.len() as i32).clone().to_ne_bytes();

        for mut user in &mut self.users {
            user.stream.write(&length.clone());
            user.stream.write(bytes.clone());
        }
    }

    fn add_user(&mut self, user: User){
        self.users.push(user);
    }
}

#[tokio::main]
async fn main(){
    let listener = TcpListener::bind("0.0.0.0:3051").expect("Failure while binding to port 3051");

    let mut server = Server::new();

    for stream in listener.incoming() {
        handle_client(stream.unwrap(), &mut server);
    }
}

async fn handle_client(stream: TcpStream, server: &mut Server) {
    let mut user = User::new(stream);
    user.handle_packets(server);
    server.add_user(user);
}


use std::net::{SocketAddr, TcpListener, TcpStream};
use std::collections::HashMap;
use std::io::Read;
use byte_array::ByteArray;

struct User {
    identifier: i32,
    username: String,
    stream: TcpStream,
    channels: Vec<String>
}

impl User {

    fn new(id: i32, tcp_stream: TcpStream) -> Self {
        //TODO Read the Username from Stream
        let username = String::from("bob");
        User{
            identifier: id,
            username,
            stream: tcp_stream,
            channels: Vec::new(),
        }
    }
    
    async fn handle_packets(&mut self){
        let mut data;
        loop {
            data = [0;4];
            self.stream.read_exact(&data).expect("Error reading packet, server shutting down...");
            let bytes_to_read = i32::from_be_bytes(data.clone());
            self.stream.read_exact(&next_data).expect("Error reading packet, server shutting down...");


        }
    }
}
//Channel Code

struct Channel {
    identifier: String,
    users: Vec<i32>
}

impl Channel {

    fn new(identifier: String) -> Self {
        Channel{
            identifier,
            users: Vec::new()
        }
    }

    fn add_user(&mut self, identifier: i32){
        self.users.push(identifier);
    }

}

    
//SERVER CODE
struct Server {
    users: Vec<User>,
    channels: HashMap<String, Channel>,
    usercount: i32
}

impl Server {
    fn new() -> Self {
        Server {
            users: Vec::new(),
            channels: HashMap::new(),
            usercount: 0
        }
    }

    fn add_channel(&mut self, channel: Channel){
        self.channels.insert(channel.identifier.clone(), channel);
    }

    fn increment_user_count(&mut self) -> i32 {
        self.usercount = self.usercount+1;
        self.usercount.clone()
    }

    fn add_user(&mut self, user: User){
        self.users.push(user);
    }
}

#[tokio::main]
async fn main(){
    let listener = TcpListener::bind("0.0.0.0:3050").expect("Failure while binding to port 3050");

    let mut server = Server::new();

    loop {
        let (socket, _) = listener.accept().unwrap();
        let userid = server.increment_user_count();
        handle_client(socket, &mut server, userid);
    }

}

async fn handle_client(stream: TcpStream, server: &mut Server, userid: i32) {
    let mut user = User::new(userid, stream);
    user.handle_packets();
    server.add_user(user);
}


use std::net::{SocketAddr, TcpListener, TcpStream};
use std::collections::HashMap;

//USER CODE
use std::net::{TcpStream}

struct User<String> {
    identifier: i32,
    username: String,
    stream: TcpStream,
    channels: Vec<String>
}

impl User {

    fn new(id: i32, tcp_stream: TcpStream) -> Self {
        //TODO Read the Username from Stream
        User{
            identifier: id,
            username, 
            stream: tcp_stream,
            channels: Vec::new();
        }
    }
    
    async fn handle_packets(&self){
        loop {
            self.stream.read_exact();
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

    fn add_user(identifier: i32){
        users.push(identifier);
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

    fn addChannel(&self, channel: Channel){
        self.channels.insert(channel.identifier, channel);
    }

    fn increment_user_count(&self) -> i32 {
        self.usercount++;
        self.usercount.clone();
    }

    fn addUser(&self, user: User){
        self.users.push(user);
    }
}

#[tokio::main]
async fn main(){
    let listener = TcpListener::bind("0.0.0.0:3050").expect("Failure while binding to port 3050") 

    let mut server = Server::new();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let userid = server.increment_user_count();
        handle_client(tcp_stream, server, userid);
    }

}

async fn handle_client(stream: TcpStream, server: &mut Server, userid: i32) {
    let mut user = User::new(userid, stream);
    user.handle_packets();
    server.addUser(user);
}


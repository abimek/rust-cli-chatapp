
use std::net::{SocketAddr, TcpListener, TcpStream};

//USER CODE
use std::net::{TcpStream}

struct User {
    username: String,
    stream: TcpStream
}


impl User {

    fn new(stream: TcpStream) -> Self {

    }        
}
    

//SERVER CODE
struct Server {
    users: Vec<User>,
    channels: Vec<User>
}

impl Server {
    fn new() -> Self {
        Server {
            users: Vec<User>::new(),
            channels: Vec<Channel>::new()       
        }
    }

    fn addUser(&self, user: User){
        self.users.push(user);
    }

    fn addChannel(&self, channel: Channel){
        self.channels.push(channel);
    }
}

#[tokio::main]
async fn main(){
    let listener = TcpListener::bind("0.0.0.0:3050").expect("Failure while binding to port 3050") 

    let mut server = Server::new();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        handle_client(tcp_stream, server);
    }

}

async fn handle_client(stream: TcpStream, server: &mut Server) {
    server.addUser()
}
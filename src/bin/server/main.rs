use std::collections::HashMap;
use tokio::net::{TcpListener, TcpStream};

#[allow(dead_code)]

struct User {
    username: Option<String>,
    stream: TcpStream,
}

impl User {
    fn new(stream: TcpStream) -> Self {
        User {
            username: None,
            stream,
        }
    }

    async fn read_packets(&self) {
        loop {}
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
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:3030").await.unwrap();
    let mut server = Server::new();

    loop {
        let (socket, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            handle_client(socket).await;
        });
    }
}

async fn handle_client(stream: TcpStream) {
    let mut user = User::new(stream);
}

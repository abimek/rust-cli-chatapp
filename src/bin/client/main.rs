use std::io::stdin;
use std::net::TcpStream;
#[tokio::main]
async fn main(){
    println!("Please type in your username");
    let mut username = String::new();
    stdin().read_line(&username).expect("error while readig line");
    let mut address: String = String::new();
    println!("Please input a server address in the format <ip>:<port>");
    stdin().read_line(&address).expect("error while reading ip");


    let mut stream = TcpStream::connect(address).expect("Error while connecting to server");
}

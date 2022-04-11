use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;

type Result<T> = std::result::Result<T, Error>;

enum Error {
    ReadLineError,
}

pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Connection { stream }
    }

    pub fn read_data() -> Result<Box<dyn Packet>> {}
}

pub trait Packet {
    fn from_string(data: &str) -> serde_json::Result<Self>
    where
        Self: Sized;

    fn to_string(&self) -> serde_json::Result<String>
    where
        Self: Sized;
}

#[derive(Serialize, Deserialize)]
pub struct MessageSendPacket {
    sender: String,
    message: String,
}

impl Packet for MessageSendPacket {
    fn from_string(data: &str) -> serde_json::Result<MessageSendPacket> {
        let p: serde_json::Result<MessageSendPacket> = serde_json::from_str(data);
        p
    }

    fn to_string(&self) -> serde_json::Result<String> {
        serde_json::to_string(&self)
    }
}

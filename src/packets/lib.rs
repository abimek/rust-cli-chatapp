use serde::{Deserialize, Serialize};
use tokio::{io::{BufWriter, BufReader, AsyncBufReadExt, AsyncReadExt}, net::{TcpStream, tcp::{OwnedWriteHalf, OwnedReadHalf}}};

type Result<T> = std::result::Result<T, Error>;

enum Error {
    PacketError(PacketErrorType),
    ReadDataError(ReadDataErrorType),
}

enum ReadDataErrorType {
    FailedPacketIdRead
}

enum PacketErrorType {
     NoneExistantPacketId,
     PacketConstructionFail
}

pub struct Connection {
    writer: BufWriter<OwnedWriteHalf>,
    reader: BufReader<OwnedReadHalf>
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {

        let (mut read, mut write) = stream.into_split();
        Connection {
            writer: BufWriter<write>,
            reader: BufReader<read>
        }
    }

    async fn read_data(&mut self) -> Result<Box<dyn Packet>> {
        match self.reader.read_u64().await {
            Ok(id) => {
                match PacketId::from_u64(id) {
                    Some(packet) => {
                        let mut data = String::new();
                        self.reader.read_line(&mut data);
                        match packet {
                            PacketId::MessageSend => {
                                if let Ok(pk) = MessageSendPacket::from_string(&data) {
                                    return Ok(Box::new(pk))
                                }
                                return Err(Error::PacketError(PacketErrorType::PacketConstructionFail));
                            },
                            _ => Err(Error::PacketError(PacketErrorType::NoneExistantPacketId))
                        } 
                    },
                    None => return Err(Error::PacketError(PacketErrorType::NoneExistantPacketId))
                }
            },
            std::result::Result::Err(n) => {
                return Err(Error::ReadDataError(ReadDataErrorType::FailedPacketIdRead));
            } 
        }
    }
}

pub enum PacketId {
    MessageSend
}

impl PacketId {
   pub fn from_u64(int: u64) -> Option<PacketId> {
        match int {
            0 => Some(PacketId::MessageSend),
            _ => None
        }
   } 
}

pub trait Packet {
    fn from_string(data: &str) -> serde_json::Result<Self>
    where
        Self: Sized;

    fn to_string(&self) -> serde_json::Result<String>
    where
        Self: Sized;

    fn get_id() -> PacketId
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

    fn get_id() -> PacketId {
        PacketId::MessageSend
    }
}

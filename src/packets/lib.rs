use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader, BufWriter},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream,
    },
};

type Result<T> = std::result::Result<T, Error>;

enum Error {
    SerdeError(serde_json::Error),
    PacketError(PacketErrorType),
    ReadDataError(ReadDataErrorType),
}

enum ReadDataErrorType {
    FailedPacketIdRead,
}

enum PacketErrorType {
    NoneExistantPacketId,
    PacketConstructionFail,
}

pub struct Connection {
    writer: BufWriter<OwnedWriteHalf>,
    reader: BufReader<OwnedReadHalf>,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        let (mut read, mut write) = stream.into_split();
        Connection {
            writer: BufWriter::new(write),
            reader: BufReader::new(read),
        }
    }
    async fn write_packet<T: Packet>(&mut self, packet: T) -> Result<()> {
        match packet.to_string() {
            Ok(mut packet_data) => {
                let packet_id = packet.get_identifier();
                if let Some(id) = PacketId::to_u64(packet_id) {
                    packet_data.push_str("\n");
                    let bytes = packet_data.as_bytes();
                    self.writer.write_all(&id.to_ne_bytes()).await.unwrap();
                    self.writer.write_all(bytes).await.unwrap();
                    self.writer.flush().await.unwrap();
                    ()
                }
                Err(Error::PacketError(PacketErrorType::NoneExistantPacketId))
            }
            Err(e) => Err(Error::SerdeError(e)),
        }
    }

    async fn read_packet(&mut self) -> Result<Box<dyn Packet>> {
        match self.reader.read_u64().await {
            Ok(id) => match PacketId::from_u64(id) {
                Some(packet) => {
                    let mut data = String::new();
                    self.reader.read_line(&mut data).await.unwrap();
                    match packet {
                        PacketId::MessageSend => {
                            if let Ok(pk) = MessageSendPacket::from_string(&data) {
                                return Ok(Box::new(pk));
                            }
                            return Err(Error::PacketError(
                                PacketErrorType::PacketConstructionFail,
                            ));
                        }
                    }
                }
                None => return Err(Error::PacketError(PacketErrorType::NoneExistantPacketId)),
            },
            std::result::Result::Err(n) => {
                return Err(Error::ReadDataError(ReadDataErrorType::FailedPacketIdRead));
            }
        }
    }
}

pub enum PacketId {
    MessageSend,
}

impl PacketId {
    pub fn from_u64(int: u64) -> Option<PacketId> {
        match int {
            0 => Some(PacketId::MessageSend),
            _ => None,
        }
    }

    pub fn to_u64(packet: PacketId) -> Option<u64> {
        match packet {
            PacketId::MessageSend => Some(0),
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

    fn get_identifier(&self) -> PacketId
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

    fn get_identifier(&self) -> PacketId {
        PacketId::MessageSend
    }

    fn get_id() -> PacketId {
        PacketId::MessageSend
    }
}

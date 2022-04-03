use serde::{Serialize, Deserialize};
use serde_json::from_str;
use serde_json::to_string;
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct MessageSendPacket {
    sender: String,
    message: String
}

impl MessageSendPacket {

    pub fn from_string(data: &str) -> Result<MessageSendPacket> {
        let mut p: Result<MessageSendPacket> = from_str(data);
        p
    }

    pub fn to_string(&self) -> Result<String> {
        to_string(&self)
    }
}









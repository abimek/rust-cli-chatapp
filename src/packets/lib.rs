use serde::{Serialize, Deserialize};
use serde_json::from_str;
use serde_json::to_string;
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct MessageSendPacket {
    id: i32,
    message: String
}

impl MessageSendPacket {

    fn from_string(data: &str) -> Result<String> {
        from_str(data)
    }

    fn to_string(&self) -> Result<String> {
        to_string(&self)
    }
}









use serde::{Serialize, Deserialize};
use serde_json::{Result, Value};

trait Packet {
    fn get_id() -> i32;

    fn read(data: String) -> Result<Self, serde_json::Error> {
        let v: Self = serde_json::from_str(data)?;
        v
    };
    fn write(&self) -> String {
        serde_json::to_string(&self);
    }
}

#[derive(Serialize, Deserialize)]
struct RequestLoginPacket {
    username: String
}

impl Packet for RequestLoginPacket{
    fn get_id() -> i32 {
        0
    }
}

#[derive(Serialize, Deserialize)]
struct ValidateLoginPacket {
    valid: bool
}

#[derive(Serialize, Deserialize)]
struct RequestJoinChannelPacket {
    channel_name: String,
    channel_password: Option<String>
}

enum ChannelJoinFailure {
    IncorrectPassword
}

#[derive(Serialize, Deserialize)]
struct ConfirmChannelJoinPacket {
    valid: bool,
    reason: Option<ChannelJoinFailure>
}

#[derive(Serialize, Deserialize)]
struct MessageSendPacket {
    id: i32,
    message: String
}






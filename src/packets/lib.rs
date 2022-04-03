use serde::{Serialize, Deserialize};
use serde_json::{Result, Value};

trait Packet {
    fn get_id() -> i32;
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

impl Packet for ValidateLoginPacket{
    fn get_id() -> i32 {
        1
    }
}

#[derive(Serialize, Deserialize)]
struct RequestJoinChannelPacket {
    channel_name: String,
    channel_password: Option<String>
}

impl Packet for RequestJoinChannelPacket{
    fn get_id() -> i32 {
        2
    }
}

#[derive(Serialize, Deserialize)]
enum ChannelJoinFailure {
    IncorrectPassword
}


#[derive(Serialize, Deserialize)]
struct ConfirmChannelJoinPacket {
    valid: bool,
    reason: Option<ChannelJoinFailure>
}

impl Packet for ConfirmChannelJoinPacket{
    fn get_id() -> i32 {
        2
    }
}

#[derive(Serialize, Deserialize)]
struct RequestLeaveChannelPacket {
    channel_name: String,
    channel_password: Option<String>
}

impl Packet for RequestLeaveChannelPacket{
    fn get_id() -> i32 {
        3
    }
}

#[derive(Serialize, Deserialize)]
struct ConfirmChannelLeavePacket {
    valid: bool
}

impl Packet for ConfirmChannelLeavePacket{
    fn get_id() -> i32 {
        4
    }
}

#[derive(Serialize, Deserialize)]
struct MessageSendPacket {
    id: i32,
    message: String
}

impl Packet for MessageSendPacket{
    fn get_id() -> i32 {
        5
    }
}










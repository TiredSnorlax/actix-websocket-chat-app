use actix::{
    prelude::{Message, Recipient},
    Addr,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::client::WsConn;

#[derive(Message)]
#[rtype(result = "()")]
pub struct ServerMessage(pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub user_id: Uuid,
    pub room_id: Uuid,
    pub addr: Addr<WsConn>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub user_id: Uuid,
    pub room_id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub user_id: Uuid,
    pub msg: String,
    pub room_id: Uuid,
}

#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "()")]
pub struct InfoMessage {
    pub info_type: String,
    pub body: String,
    pub room_id: Uuid,
}

#[derive(Message, Serialize, Deserialize, Clone)]
#[rtype(result = "()")]
pub struct UserMessage {
    pub user_id: Uuid,
    pub msg: String,
    pub room_id: Uuid,
}
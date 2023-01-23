use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use actix::{Actor, Addr, Context, Handler, Recipient};
use uuid::Uuid;

use crate::{
    client::WsConn,
    messages::{ClientMessage, Connect, Disconnect, InfoMessage, ServerMessage, UserMessage},
};

struct User {
    addr: Addr<WsConn>,
    username: String,
}

type Socket = Addr<WsConn>;

pub struct Lobby {
    sessions: HashMap<Uuid, Socket>,
    rooms: HashMap<Uuid, HashSet<Uuid>>,
}

impl Lobby {
    pub fn new() -> Self {
        Lobby {
            sessions: HashMap::new(),
            rooms: HashMap::new(),
        }
    }

    fn send_message(&self, to_id: &Uuid, info: InfoMessage) {
        if let Some(recipient) = self.sessions.get(to_id) {
            recipient.do_send(info);
        } else {
            println!("Message to {} failed to send", to_id);
        }
    }

    fn send_info(&self, to_id: &Uuid, info: InfoMessage) {
        if let Some(recipient) = self.sessions.get(to_id) {
            recipient.do_send(info);
        } else {
            println!("Info message to {} failed to send", to_id);
        }
    }

    fn relay_user_message(&self, to_id: &Uuid, user_msg: UserMessage) {
        if let Some(recipient) = self.sessions.get(to_id) {
            recipient.do_send(user_msg);
        } else {
            println!("User message to {} failed to send", to_id);
        }
    }
}

impl Actor for Lobby {
    type Context = Context<Self>;
}

impl Handler<Connect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) {
        self.rooms
            .entry(msg.room_id)
            .or_insert_with(HashSet::new)
            .insert(msg.user_id);

        self.rooms
            .get(&msg.room_id)
            .unwrap()
            .iter()
            .filter(|id| *id != &msg.user_id)
            .for_each(|id| {
                self.send_info(
                    id,
                    InfoMessage {
                        info_type: "Join".to_owned(),
                        body: format!("New user, {} just joined!", msg.user_id),
                        room_id: msg.room_id,
                    },
                )
            });

        self.sessions.insert(msg.user_id, msg.addr);

        self.send_message(
            &msg.user_id,
            InfoMessage {
                info_type: "Alert".to_owned(),
                body: format!("Your id is: {}", msg.user_id),
                room_id: msg.room_id,
            },
        );
    }
}

impl Handler<Disconnect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        if self.sessions.remove(&msg.user_id).is_some() {
            self.rooms
                .get(&msg.room_id)
                .unwrap()
                .iter()
                .filter(|id| *id != &msg.user_id)
                .for_each(|id| {
                    self.send_info(
                        id,
                        InfoMessage {
                            info_type: "Leave".to_owned(),
                            body: format!("User {} has disconnected", msg.user_id),
                            room_id: msg.room_id,
                        },
                    )
                });

            if let Some(room) = self.rooms.get_mut(&msg.room_id) {
                if room.len() > 1 {
                    room.remove(&msg.user_id);
                } else if room.len() == 1 {
                    self.rooms.remove(&msg.room_id);
                }
            }
        }
    }
}

// impl Handler<ClientMessage> for Lobby {
//     type Result = ();
//     fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
//         self.rooms
//             .get(&msg.room_id)
//             .unwrap()
//             .iter()
//             .for_each(|user| self.send_message(user, &msg.msg.clone()))
//     }
// }

impl Handler<UserMessage> for Lobby {
    type Result = ();
    fn handle(&mut self, msg: UserMessage, _: &mut Context<Self>) {
        self.rooms
            .get(&msg.room_id)
            .unwrap()
            .iter()
            .for_each(|user| self.relay_user_message(user, msg.clone()))
    }
}

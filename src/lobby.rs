use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use actix::{Actor, Context, Handler, Recipient};
use uuid::Uuid;

use crate::messages::{ClientMessage, Connect, Disconnect, ServerMessage};

type Socket = Recipient<ServerMessage>;

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

    // * Idk if msg needs to be a str slice
    fn send_message(&self, to_id: &Uuid, msg: &str) {
        if let Some(recipient) = self.sessions.get(to_id) {
            recipient.do_send(ServerMessage(msg.to_owned()));
        } else {
            println!("Message to {} failed to send", to_id);
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
                self.send_message(id, &format!("New user, {} just joined!", msg.user_id))
            });

        self.sessions.insert(msg.user_id, msg.addr);

        self.send_message(&msg.user_id, &format!("Your id is: {}", msg.user_id));
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
                    self.send_message(id, &format!("User {} has disconnected", msg.user_id))
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

impl Handler<ClientMessage> for Lobby {
    type Result = ();
    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
        self.rooms
            .get(&msg.room_id)
            .unwrap()
            .iter()
            .for_each(|user| self.send_message(user, &msg.msg.clone()))
    }
}

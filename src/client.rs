use std::time::{Duration, Instant};

use actix::{
    fut, Actor, ActorContext, ActorFutureExt, Addr, AsyncContext, ContextFutureSpawner, Handler,
    Running, StreamHandler, WrapFuture,
};
use actix_web_actors::ws;
use uuid::Uuid;

use crate::{
    lobby::Lobby,
    messages::{ClientMessage, Connect, Disconnect, InfoMessage, UserMessage, ServerMessage},
};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct WsConn {
    room: Uuid,
    user_id: Uuid,
    lobby_addr: Addr<Lobby>,
    hb: Instant,
}

impl WsConn {
    pub fn new(room: Uuid, lobby_addr: Addr<Lobby>, user_id: Uuid) -> Self {
        WsConn {
            room,
            user_id,
            lobby_addr,
            hb: Instant::now(),
        }
    }

    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("Disconnecting user {}", act.user_id);
                act.lobby_addr.do_send(Disconnect {
                    user_id: act.user_id,
                    room_id: act.room,
                });
                ctx.stop();
                return;
            };
            ctx.ping(b"hi");
        });
    }
}

impl Actor for WsConn {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        let addr = ctx.address();

        self.lobby_addr
            .send(Connect {
                addr,
                room_id: self.room,
                user_id: self.user_id,
            })
            .into_actor(self)
            .then(|res, _, ctx| {
                match res {
                    Ok(_res) => (),
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.lobby_addr.do_send(Disconnect {
            user_id: self.user_id,
            room_id: self.room,
        });
        Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConn {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            Ok(ws::Message::Continuation(_)) => {
                ctx.stop();
            }
            Ok(ws::Message::Nop) => (),
            Ok(ws::Message::Text(s)) => self.lobby_addr.do_send(UserMessage {
                user_id: self.user_id,
                msg: s.to_string(),
                room_id: self.room,
            }),

            Err(e) => panic!("{}", e),
        }
    }
}

impl Handler<ServerMessage> for WsConn {
    type Result = ();

    fn handle(&mut self, msg: ServerMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl Handler<InfoMessage> for WsConn {
    type Result = ();

    fn handle(&mut self, msg: InfoMessage, ctx: &mut Self::Context) {
        let json_res = serde_json::to_string(&msg).unwrap();
        ctx.text(json_res);
    }
}

impl Handler<UserMessage> for WsConn {
    type Result = ();

    fn handle(&mut self, msg: UserMessage, ctx: &mut Self::Context) {
        let json_res = serde_json::to_string(&msg).unwrap();
        ctx.text(json_res);
    }
}

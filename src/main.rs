use std::str::FromStr;

use crate::client::WsConn;
use crate::lobby::Lobby;
use actix::Actor;
use actix::Addr;
use actix_files as fs;
use actix_web::middleware::Logger;
use actix_web::web;
use actix_web::Responder;
use actix_web::{get, web::Data, web::Path, web::Payload, Error, HttpRequest, HttpResponse};
use actix_web::{App, HttpServer};
use actix_web_actors::ws;
use uuid::Uuid;

mod client;
mod lobby;
mod messages;

#[get("/ws/{group_id}")]
async fn start_connection(
    req: HttpRequest,
    path: Path<String>,
    stream: Payload,
    srv: Data<Addr<Lobby>>,
) -> impl Responder {
    let group_id = path.into_inner();
    let group_id = Uuid::from_str(group_id.as_str()).unwrap_or(Uuid::new_v4());
    let ws = WsConn::new(group_id, srv.get_ref().clone());

    ws::start(ws, &req, stream)
}

#[get("/test")]
async fn hello() -> impl Responder {
    println!("test");
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let chat_server = Lobby::new().start(); //create and spin up a lobby

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(chat_server.clone()))
            .service(start_connection)
            .service(hello) //register our route. rename with "as" import or naming conflict
            .service(fs::Files::new("/chat", "./static").show_files_listing())
            .wrap(Logger::default())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

use std::str::FromStr;
use std::sync::Mutex;

use crate::client::WsConn;
use crate::lobby::Lobby;
use actix::Actor;
use actix::Addr;
use actix_cors::Cors;
use actix_files as fs;
use actix_web::http::header;
use actix_web::http::StatusCode;
use actix_web::middleware::Logger;
use actix_web::post;
use actix_web::web;
use actix_web::Responder;
use actix_web::{get, web::Data, web::Path, web::Payload, HttpRequest, HttpResponse};
use actix_web::{App, HttpServer};
use actix_web_actors::ws;
use actix_web_lab::web::Redirect;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

mod client;
mod lobby;
mod messages;

const CLIENT_DOMAIN: &str = "http://127.0.0.1:5173";
const SERVER_DOMAIN: &str = "http://127.0.0.1:8080";

#[derive(Deserialize)]
struct SocketQuery {
    session: String,
}

#[get("/ws/{group_id}")]
async fn start_socket(
    req: HttpRequest,
    path: Path<String>,
    stream: Payload,
    srv: Data<Addr<Lobby>>,
    query: web::Query<SocketQuery>,
) -> impl Responder {
    let group_id = path.into_inner();
    let group_id = Uuid::from_str(group_id.as_str()).unwrap_or(Uuid::new_v4());
    if let Ok(session) = Uuid::from_str(&query.session) {
        let new_socket = WsConn::new(group_id, srv.get_ref().clone(), session);
        return ws::start(new_socket, &req, stream);
    }
    Ok(HttpResponse::BadRequest().body("Bad session"))
}

#[derive(Deserialize, Serialize, Debug)]
struct ConnectionResponse {
    session: Uuid,
}

#[post("/connect")]
async fn start_connection() -> web::Json<ConnectionResponse> {
    web::Json(ConnectionResponse {
        session: Uuid::new_v4(),
    })
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Room {
    name: String,
    description: String,
    id: Uuid,
}

struct RoomsState {
    rooms: Mutex<Vec<Room>>,
}

#[get("/list")]
async fn list_rooms(data: web::Data<RoomsState>) -> web::Json<Vec<Room>> {
    let rooms = data.rooms.lock().unwrap();
    web::Json((*rooms.clone()).to_vec())
}

#[derive(Debug, Serialize, Deserialize)]
struct NewChatBody {
    name: String,
    description: String,
}

#[post("/room/new")]
async fn start_new_room(
    body: web::Json<NewChatBody>,
    data: web::Data<RoomsState>,
) -> impl Responder {
    let mut rooms = data.rooms.lock().unwrap();

    let new_id = Uuid::new_v4();

    let new_room = Room {
        name: body.name.clone(),
        description: body.description.clone(),
        id: new_id.clone(),
    };

    (*rooms).push(new_room);

    // TODO add a client domain constant
    Redirect::to(format!("{}room/{}", CLIENT_DOMAIN, new_id)).using_status_code(StatusCode::FOUND)
}

#[post("/room/{room_id}/data")]
async fn get_room_data(path: Path<String>, data: web::Data<RoomsState>) -> web::Json<Room> {
    let room_id = path.into_inner();
    let rooms = data.rooms.lock().unwrap();

    if let Some(room) = (*rooms)
        .iter()
        .find(|room| (*room).id == Uuid::from_str(&room_id).unwrap_or(Uuid::new_v4()))
    {
        return web::Json(room.clone());
    } else {
        return web::Json(Room {
            name: "Default".to_owned(),
            description: "".to_owned(),
            id: Uuid::new_v4(),
        });
    }
}

#[get("/test")]
async fn hello(req: HttpRequest) -> impl Responder {
    let session = req.cookies();
    println!("{:?}", session);
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let chat_server = Lobby::new().start(); //create and spin up a lobby
    let rooms = Data::new(RoomsState {
        rooms: Mutex::new(vec![Room {
            name: "main".to_owned(),
            description: "This is the main chat room that will always be available".to_owned(),
            id: Uuid::new_v4(),
        }]),
    });

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin(CLIENT_DOMAIN)
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials(),
            )
            .app_data(Data::new(chat_server.clone()))
            .app_data(rooms.clone())
            .service(start_socket)
            .service(start_connection)
            .service(list_rooms)
            .service(start_new_room)
            .service(get_room_data)
            .service(hello) //register our route. rename with "as" import or naming conflict
            .service(fs::Files::new("/chat", "./static").show_files_listing())
            .wrap(Logger::default())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

mod entities;
mod environment;
mod schema;

use actix::Actor;
use actix::StreamHandler;
use actix_web::web;
use actix_web::web::scope;
use actix_web::web::Json;
use actix_web::App;
use actix_web::Error;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::HttpServer;
use actix_web::Responder;
use actix_web::{get, post};
use actix_web_actors::ws;
use diesel::prelude::*;
use diesel::SqliteConnection;
use entities::organization::Organization;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use tracing_actix_web::TracingLogger;

/// Define HTTP actor
struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

#[derive(Serialize, Deserialize, Debug)]
struct Event<T> {
    pub event_type: String,
    pub data: T,
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                let event: Event<Value> =
                    serde_json::from_str(&text).expect("error when unwrap payload");

                if &event.event_type == "organizations:list" {
                    let vars = environment::get();
                    let mut conn = SqliteConnection::establish(&vars.database_url)
                        .unwrap_or_else(|_| panic!("Error connecting to {}", vars.database_url));
                    use schema::organizations::dsl::*;

                    let result = organizations.load::<Organization>(&mut conn).unwrap();
                    ctx.text(serde_json::to_string(&result).unwrap());
                }
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

#[get("/ws")]
async fn websocket(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(MyWs {}, &req, stream)
}

#[post("/organizations")]
async fn create_organization(params: Json<Organization>) -> impl Responder {
    let vars = environment::get();
    let mut conn = SqliteConnection::establish(&vars.database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", vars.database_url));

    diesel::insert_into(schema::organizations::table)
        .values(&params.0)
        .execute(&mut conn)
        .unwrap();

    return HttpResponse::Created().json(&params);
}

#[get("/organizations")]
async fn list_organizations() -> impl Responder {
    let vars = environment::get();
    let mut conn = SqliteConnection::establish(&vars.database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", vars.database_url));
    use schema::organizations::dsl::*;

    let result = organizations.load::<Organization>(&mut conn).unwrap();

    return HttpResponse::Ok().json(result);
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
            .wrap(TracingLogger::default())
            .service(
                scope("/api/v1")
                    .service(create_organization)
                    .service(list_organizations),
            )
            .service(websocket)
    })
    .workers(4)
    .bind(("127.0.0.1", 4000))?
    .run()
    .await
}

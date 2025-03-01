use actix_web::{get, web, App, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct Message {
    text: String,
}

#[get("/api/message")]
async fn get_message() -> impl Responder {
    let message = Message {
        text: "Hello from Actix Web!".to_string(),
    };
    web::Json(message)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_message))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
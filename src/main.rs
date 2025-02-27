mod web_front;
// トレイト境界エラー
// use web_front::src::app::front;

// Actix Web のエントリーポイントになる予定
use actix_web::*;
use serde::Serialize;
/*
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
*/
#[derive(Serialize)]
struct Message {
    text: String,
}

#[get("/api/message")]
async fn get_message() -> impl Responder {
    let message = Message {
        text: "Hello, from Actix Web!".to_string(),
    };
    web::Json(message)
}
/*
async fn front() -> impl Responder {
    HttpResponse::Ok().body("Yew app")
}
*/

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            /*
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .route("/web/src/main.rs", web::get().to(front))
            */
            .service(get_message)
    })
    .bind(("127.0.0.1:8080"))?
    .run()
    .await
}
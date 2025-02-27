// Actix Web のエントリーポイントになる予定
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

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

async fn front() -> impl Responder {
    HttpResponse::Ok().body("Yew app")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            /*
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            */
            .route("/web/src/main.rs", web::get().to(front))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
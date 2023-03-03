use actix_web::{get, HttpResponse, App, HttpServer, Responder};


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}


#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
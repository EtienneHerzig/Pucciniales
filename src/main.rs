use actix_web::{get, HttpResponse, App, HttpServer, Responder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod, SslAcceptorBuilder};



#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}


#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut builder: SslAcceptorBuilder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
    builder.set_private_key_file("ssc_key.pem", SslFiletype::PEM)?;
    builder.set_certificate_chain_file("ssc_cert.pem")?;

    
    HttpServer::new(|| { App::new().service(hello) })
    .bind_openssl("127.0.0.1:443", builder)?
    .run()
    .await
}
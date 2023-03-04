mod routes;

use actix_web::{App, HttpServer};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod, SslAcceptorBuilder};
use routes as route;


#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut builder: SslAcceptorBuilder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
    builder.set_private_key_file("ssc_key.pem", SslFiletype::PEM)?;
    builder.set_certificate_chain_file("ssc_cert.pem")?;


    HttpServer::new(|| { App::new()
        .service(route::index::index)
    })
    .bind_openssl("0.0.0.0:443", builder)?
    .run()
    .await
}
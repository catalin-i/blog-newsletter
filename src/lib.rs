use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

#[derive(serde::Deserialize)]
struct UserData {
    name: String,
    email: String,
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    Ok(server)
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

async fn subscribe(form: web::Form<UserData>) -> impl Responder {
    println!("User: {}, with email: {}, subscribed!", form.name, form.email);
    HttpResponse::Ok()
}

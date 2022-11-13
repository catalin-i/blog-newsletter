use actix_web::{web, HttpResponse, Responder};

#[derive(serde::Deserialize)]
pub struct UserData {
    name: String,
    email: String,
}

pub async fn subscribe(form: web::Form<UserData>) -> impl Responder {
    println!(
        "User: {}, with email: {}, subscribed!",
        form.name, form.email
    );
    HttpResponse::Ok()
}

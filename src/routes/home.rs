use actix_web::{get, Responder};

#[get("/home")]
async fn home() -> impl Responder {
    let response: &str = "Welcome to Actix Web Server";
    response
}


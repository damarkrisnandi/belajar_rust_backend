use crate::routes::User;
use actix_web::{http::StatusCode, post, web::Json, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct createUserResponse {
    id: u32,
    user: User,
}

#[post("/user/create")]
pub async fn create_new_user(user: Json<User>) -> impl Responder {
    println!("POST: /user/create");

    (
        Json(createUserResponse {
            id: 1,
            user: user.0,
        }),
        StatusCode::CREATED,
    )
}


use actix_web::{
    get,
    http::StatusCode,
    web::{Json, Path},
    Responder,
};
use serde::{Deserialize, Serialize};
#[get("hello/{firstname}/{lastname}")]
async fn hello_user(params: Path<(String, String)>) -> impl Responder {
    let response: User = User::new(params.0.clone(), params.1.clone());
    (Json(response), StatusCode::OK)
}

#[derive(Serialize, Deserialize)]
pub struct User {
    first_name: String,
    last_name: String,
}

impl User {
    fn new(firstname: String, lastname: String) -> Self {
        Self {
            first_name: firstname,
            last_name: lastname,
        }
    }
}


use actix_web::{get, http::StatusCode, web::Json, App, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct Book {
    id: i64,
    title: String,
    description: String,
}

impl Book {
    fn new(id_input: i64, title_input: String, desc_input: String) -> Self {
        Self {
            id: id_input,
            title: title_input,
            description: desc_input,
        }
    }
}

#[get("books")]
async fn get_all_books() -> impl Responder {
    let book1 = Book::new(1_i64, "Linear Algebra".to_string(), "Lorem ipsum dolor sit amet, officia excepteur ex fugiatofficia pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis officia eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt velit enim. Voluptate laboris sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis.".to_string());
    let book2 = Book::new(2_i64, "Data Sciences for Babies".to_string(),"fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate dolor minim nulla est proi".to_string());
    let books = [book1, book2];
    (Json(books), StatusCode::OK)
}


use actix_web::{dev::Server, App, HttpServer};

mod routes;
use routes::*;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let server: Server = HttpServer::new(|| {
        App::new()
            .service(home)
            .service(hello_user)
            .service(get_all_books)
            .service(create_new_user)
    })
    .bind(("127.0.0.1", 8000))?
    .run();
    println!("Server Running at 127.0.0.1:8000");
    server.await
}

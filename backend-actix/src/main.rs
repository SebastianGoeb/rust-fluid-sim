use actix_files as fs;
use actix_web::{web, App, HttpServer};

mod gravity;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let gravity_app = web::Data::new(gravity::new());
    HttpServer::new(move || {
        App::new()
            .configure(|cfg| gravity::config(gravity_app.clone(), cfg))
            .service(fs::Files::new("/static", "../frontend/src/static").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

use std::sync::Mutex;
use std::thread::scope;
use std::time::{SystemTime, UNIX_EPOCH};

use actix_files as fs;
use actix_web::{App, get, HttpServer, Responder, Result, web};
use actix_web::web::Json;
use serde::Serialize;
use gravity_sim::{Entity, State, Vec2};

use crate::gravity::GravityApp;

mod gravity;

#[derive(Debug, Serialize)]
struct Pos {
    x: f64,
    y: f64,
}

#[derive(Debug, Serialize)]
struct Tick {
    entities: Vec<Pos>,
}

#[get("/tick")]
async fn tick() -> Result<impl Responder> {
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let h_ms = 500;
    let v_ms = h_ms * 10;

    let t = (duration.as_millis() % 1000) as f64 / 1000.0 * 3.14 * 2.0;
    // let s = t.sin();

    let entities = vec![
        Pos {
            x: t.sin() * 20.0 + 25.0,
            y: t.cos() * 20.0 + 25.0,
        },
        Pos {
            x: t.sin() * 20.0 + 25.0,
            y: t.cos() * 20.0 + 75.0,
        },
        Pos {
            x: t.sin() * 20.0 + 75.0,
            y: t.cos() * 20.0 + 25.0,
        },
        Pos {
            x: t.sin() * 20.0 + 75.0,
            y: t.cos() * 20.0 + 75.0,
        },
    ];
    Ok(Json(Tick { entities }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let gravity_app = web::Data::new(gravity::new());
    HttpServer::new(move ||
        App::new()
            .configure(|cfg| gravity_app.config(gravity_app.clone(), cfg))
            .service(web::scope("/api").service(tick))
            .service(fs::Files::new("/static", "../frontend/src/static").index_file("index.html"))
    )
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
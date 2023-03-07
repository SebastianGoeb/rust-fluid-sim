use std::sync::Mutex;

use actix_web::web::Json;
use actix_web::{post, web, Responder, Result};

use gravity_sim::State;

#[post("")]
pub async fn setup(data: web::Data<GravityApp>, body: web::Json<State>) -> Result<impl Responder> {
    let mut state = data.state.lock().unwrap();
    *state = body.to_owned();
    Ok(Json(state.clone()))
}

#[post("/step_naive")]
pub async fn step_naive(data: web::Data<GravityApp>) -> Result<impl Responder> {
    let mut state = data.state.lock().unwrap();
    *state = gravity_sim::step_naive(&*state);
    Ok(Json(state.clone()))
}

pub struct GravityApp {
    pub state: Mutex<State>,
}

pub fn new() -> GravityApp {
    GravityApp {
        state: Mutex::new(State {
            step_s: 10.0,
            time_s: 0.0,
            entities: vec![],
        }),
    }
}

pub fn config(data: web::Data<GravityApp>, cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/gravity")
            .app_data(data)
            .service(setup)
            .service(step_naive),
    );
}

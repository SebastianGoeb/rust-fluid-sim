use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use actix_web::{post, Responder, Result, web};
use actix_web::web::{Data, Json};
use serde::{Serialize, Serializer};
use gravity_sim::{Entity, State, Vec2};

#[post("/step_naive")]
async fn step_naive(data: web::Data<GravityApp>) -> Result<impl Responder> {
    let mut state = data.state.lock().unwrap();
    *state = gravity_sim::step_naive(&*state);
    Ok(Json(state.clone()))
}

pub struct GravityApp {
    pub state: Mutex<State>
}

pub fn new() -> GravityApp {
    GravityApp {
        state: Mutex::new(State {
            time_s: 0,
            entities: vec![
                Entity {
                    mass_kg: 1.0,
                    position_m: Vec2 { x: 0.0, y: 0.0},
                    velocity_ms: Vec2 { x: 0.0, y: 0.0}
                }
            ]
        })
    }
}

impl GravityApp {
    pub fn config(&self, data: Data<GravityApp>, cfg: &mut web::ServiceConfig) {
        cfg.service(web::scope("/gravity")
            .app_data(&data.state)
            .service(step_naive));
    }
}

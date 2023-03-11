mod geom;

use geom::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entity {
    pub mass_kg: f64,
    pub position_m: Vec2,
    pub velocity_ms: Vec2,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct State {
    pub step_s: f64,
    pub time_s: f64,
    pub entities: Vec<Entity>,
}

pub fn step_naive(state: &State) -> State {
    let g = 6.67430e-11; // gravitational constant
    let step_s = 10.0;

    let forces_n: Vec<Vec2> = state
        .entities
        .iter()
        .map(|entity| {
            state
                .entities
                .iter()
                .filter(|other_entity| {
                    *other_entity as *const _ as *const () != entity as *const _ as *const ()
                })
                .map(|other_entity| {
                    let r_m = (&other_entity.position_m - &entity.position_m).magnitude();
                    let force_n = (g * entity.mass_kg * other_entity.mass_kg) / (r_m * r_m);
                    let unit_vec = (&other_entity.position_m - &entity.position_m).unit();
                    unit_vec * force_n
                })
                .sum()
        })
        .collect();

    let new_entities: Vec<Entity> = state
        .entities
        .iter()
        .zip(forces_n.iter())
        .map(|(entity, force_n)| {
            let new_velocity = &entity.velocity_ms + &(&(force_n / entity.mass_kg) * step_s);
            let new_position = &entity.position_m + &(&new_velocity * step_s);
            Entity {
                mass_kg: entity.mass_kg,
                position_m: new_position,
                velocity_ms: new_velocity,
            }
        })
        .collect();
    return State {
        step_s: state.step_s,
        time_s: state.time_s + state.step_s,
        entities: new_entities,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}

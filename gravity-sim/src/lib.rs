use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

#[derive(Serialize, Clone)]
pub struct Entity {
    pub mass_kg: f64,
    pub position_m: Vec2,
    pub velocity_ms: Vec2
}

#[derive(Serialize, Clone)]
pub struct State {
    pub time_s: i32,
    pub entities: Vec<Entity>
}

pub fn step_naive(state: &State) -> State {
    let forces = state.entities.iter().map(|_entity| Vec2 {x: 0.0, y: 0.0});
    return State {
        time_s: state.time_s + 1,
        entities: state.entities.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}

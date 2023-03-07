use std::ops;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Vec2 {
        Vec2 { x, y }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn unit(&self) -> Vec2 {
        let m = self.magnitude();
        Vec2 {
            x: self.x / m,
            y: self.y / m,
        }
    }
}

impl ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Add<&Vec2> for &Vec2 {
    type Output = Vec2;

    fn add(self, rhs: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Sub<&Vec2> for &Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Mul<f64> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f64) -> Vec2 {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::Mul<f64> for &Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f64) -> Vec2 {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::Div<f64> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f64) -> Vec2 {
        Vec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl ops::Div<f64> for &Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f64) -> Vec2 {
        Vec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl std::iter::Sum<Self> for Vec2 {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        iter.reduce(|a, b| a + b).unwrap_or(Vec2 { x: 0.0, y: 0.0 })
    }
}

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

    let forces_n: Vec<Vec2> = state.entities.iter().map(|entity| {
        state.entities.iter()
            .filter(|other_entity| *other_entity as *const _ as * const() != entity as *const _ as *const())
            .map(|other_entity| {
                let r_m = (&other_entity.position_m - &entity.position_m).magnitude();
                let force_n = (g * entity.mass_kg * other_entity.mass_kg) / (r_m * r_m);
                let unit_vec = (&other_entity.position_m - &entity.position_m).unit();
                unit_vec * force_n
            }).sum()
    }).collect();

    let new_entities: Vec<Entity> = state.entities.iter().zip(forces_n.iter())
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

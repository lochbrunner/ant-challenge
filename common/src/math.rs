use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pose {
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
}

impl Pose {
    pub fn zero() -> Self {
        Self {
            x: 0.,
            y: 0.,
            rotation: 0.,
        }
    }
}

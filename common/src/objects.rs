use crate::math::{Pose, Vector2};
use serde::{Deserialize, Serialize};

pub type Team = u8;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SugarHill {
    pub pose: Pose,
    pub volume: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AntHill {
    pub pose: Pose,
    pub team: Team,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ant {
    pub pose: Pose,
    pub team: Team,
    pub hp: f32, // 0 - 1
    pub velocity: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SmellCloud {
    pub position: Vector2,
    pub code: u32,
    pub age: u32, // ins steps
    pub team: Team,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Map {
    pub width: f32,
    pub height: f32,
}

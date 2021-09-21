use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pose {
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
}

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
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Frame {
    pub ants: Vec<Ant>,
    pub anthills: Vec<AntHill>,
    pub raspberries: Vec<Pose>,
    pub sugar_hills: Vec<SugarHill>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Map {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Recording {
    pub map: Map,
    pub frames: Vec<Frame>,
}

impl Recording {
    pub fn new() -> Self {
        Recording {
            map: Map {
                width: 128.,
                height: 128.,
            },
            frames: vec![],
        }
    }

    pub fn load<R>(reader: R) -> Result<Self, String>
    where
        R: std::io::Read,
    {
        bincode::deserialize_from::<R, Self>(reader).map_err(|msg| msg.to_string())
    }

    pub fn dump<W>(&self, writer: W) -> Result<(), String>
    where
        W: std::io::Write,
    {
        bincode::serialize_into(writer, self).map_err(|msg| msg.to_string())
    }
}

#[cfg(test)]
mod tests {}

use serde::{Deserialize, Serialize};

pub mod math;
pub mod objects;

pub use math::{Pose, Vector2};
pub use objects::{Ant, AntHill, Map, SmellCloud, SugarHill, Team};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Frame {
    pub ants: Vec<Ant>,
    pub anthills: Vec<AntHill>,
    pub raspberries: Vec<Pose>,
    pub sugar_hills: Vec<SugarHill>,
    pub smells_clouds: Vec<SmellCloud>,
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

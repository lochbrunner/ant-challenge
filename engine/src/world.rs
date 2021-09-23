use common::{Ant, AntHill, Frame, Map, Pose, SmellCloud, SugarHill, Team, Vector2};
use nalgebra::Isometry2;
use ncollide2d::bounding_volume;
use ncollide2d::bounding_volume::BoundingVolume;
use ncollide2d::pipeline::{CollisionGroups, CollisionObjectSlabHandle, GeometricQueryType};
use ncollide2d::shape::{Ball, ShapeHandle};
use ncollide2d::world::CollisionWorld;

trait Mirror {
    fn mirror(&self) -> Self;
}

impl Mirror for Isometry2<f32> {
    fn mirror(&self) -> Self {
        Isometry2::from_parts(self.translation.inverse(), -self.rotation)
    }
}

enum MapItem {
    Anthill(AntHill),
    Ant(Ant),
    SugarHill(SugarHill),
    Raspberry(Pose),
    SmellCloud(SmellCloud),
}

trait Convert<Target> {
    fn convert(&self) -> Target;
}

const ANT_HILL_RADIUS: f32 = 3.5;
const ANT_RADIUS: f32 = 0.5;
const SUGAR_HILL_RADIUS: f32 = 2.;
const RASPBERRY_RADIUS: f32 = 0.5;
const SMELL_CLOUD_RADIUS: f32 = 3.;

impl Convert<Pose> for nalgebra::Isometry2<f32> {
    fn convert(&self) -> Pose {
        Pose {
            x: self.translation.x,
            y: self.translation.y,
            rotation: self.rotation.into_inner().re,
        }
    }
}

impl Convert<Vector2> for nalgebra::Isometry2<f32> {
    fn convert(&self) -> Vector2 {
        Vector2 {
            x: self.translation.x,
            y: self.translation.y,
        }
    }
}

pub struct World {
    pub map: Map,
    world: CollisionWorld<f32, MapItem>,
    ant_hill_shape: ShapeHandle<f32>,
    sugar_hill_shape: ShapeHandle<f32>,
    ant_shape: ShapeHandle<f32>,
    raspberry_shape: ShapeHandle<f32>,
    smell_cloud_shape: ShapeHandle<f32>,
    collision_groups: CollisionGroups,
    smell_collision_groups: CollisionGroups,
    ant_hills: Vec<CollisionObjectSlabHandle>, // really needed?
    sugar_hills: Vec<CollisionObjectSlabHandle>, // really needed?
    ants: Vec<CollisionObjectSlabHandle>,      // really needed?
    raspberries: Vec<CollisionObjectSlabHandle>, // really needed?
    smell_clouds: Vec<CollisionObjectSlabHandle>, // really needed?
}

impl World {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            map: Map { width, height },
            world: CollisionWorld::new(0.02f32),
            ant_hill_shape: ShapeHandle::new(Ball::new(ANT_HILL_RADIUS)),
            sugar_hill_shape: ShapeHandle::new(Ball::new(SUGAR_HILL_RADIUS)),
            ant_shape: ShapeHandle::new(Ball::new(ANT_RADIUS)),
            raspberry_shape: ShapeHandle::new(Ball::new(RASPBERRY_RADIUS)),
            smell_cloud_shape: ShapeHandle::new(Ball::new(SMELL_CLOUD_RADIUS)),
            collision_groups: CollisionGroups::new(),
            smell_collision_groups: CollisionGroups::new()
                .with_membership(&[2])
                .with_blacklist(&[2]),
            ant_hills: Vec::new(),
            ants: Vec::new(),
            sugar_hills: Vec::new(),
            raspberries: Vec::new(),
            smell_clouds: Vec::new(),
        }
    }

    /// Call this after adding items to the world
    pub fn update(&mut self) {
        self.world.update()
    }

    pub fn step(&mut self) {
        // TODO: move, act, remove, add new items
        self.world.update();
        unimplemented!();
    }

    pub fn sense(&self) {
        unimplemented!();
        // Find collisions
        // Smell
        // look via ray-casting
    }

    pub fn add_ant_hill(&mut self, position: Isometry2<f32>, team: Team) {
        let (handle, _) = self.world.add(
            position,
            self.ant_hill_shape.clone(),
            self.collision_groups,
            GeometricQueryType::Contacts(0.0, 0.0),
            MapItem::Anthill(AntHill {
                pose: Pose::zero(),
                team,
            }),
        );
        self.ant_hills.push(handle);
    }

    pub fn try_add_ant_hill(&mut self, position: Isometry2<f32>, team: Team) -> Option<()> {
        let bounding_box = bounding_volume::aabb(&Ball::new(ANT_HILL_RADIUS), &position);
        if self
            .world
            .interferences_with_aabb(&bounding_box, &self.collision_groups)
            .count()
            > 0
        {
            None
        } else {
            self.add_ant_hill(position, team);
            Some(())
        }
    }

    pub fn add_ant_hill_mirrored(&mut self, position: Isometry2<f32>, team: Team) -> Option<()> {
        let mirrored = position.mirror();
        let bounding_box = bounding_volume::aabb(&Ball::new(ANT_HILL_RADIUS), &position);
        let bounding_box_mirrored = bounding_volume::aabb(&Ball::new(ANT_HILL_RADIUS), &mirrored);
        if self
            .world
            .interferences_with_aabb(&bounding_box, &self.collision_groups)
            .count()
            > 0
            || self
                .world
                .interferences_with_aabb(&bounding_box_mirrored, &self.collision_groups)
                .count()
                > 0
            || bounding_box.intersects(&bounding_box_mirrored)
        {
            None
        } else {
            self.add_ant_hill(position, team);
            self.add_ant_hill(mirrored, team);
            Some(())
        }
    }

    pub fn add_sugar_hill(&mut self, position: Isometry2<f32>) {
        let (handle, _) = self.world.add(
            position,
            self.sugar_hill_shape.clone(),
            self.collision_groups,
            GeometricQueryType::Contacts(0.0, 0.0),
            MapItem::SugarHill(SugarHill {
                pose: Pose::zero(),
                volume: 1.,
            }),
        );
        self.sugar_hills.push(handle);
    }

    pub fn try_add_sugar_hill(&mut self, position: Isometry2<f32>) -> Option<()> {
        let bounding_box = bounding_volume::aabb(&Ball::new(SUGAR_HILL_RADIUS), &position);
        if self
            .world
            .interferences_with_aabb(&bounding_box, &self.collision_groups)
            .count()
            > 0
        {
            None
        } else {
            self.add_sugar_hill(position);
            Some(())
        }
    }

    pub fn add_sugar_hill_mirrored(&mut self, position: Isometry2<f32>) -> Option<()> {
        let mirrored = position.mirror();
        let geometry = Ball::new(SUGAR_HILL_RADIUS);
        let bounding_box = bounding_volume::aabb(&geometry, &position);
        let bounding_box_mirrored = bounding_volume::aabb(&geometry, &mirrored);
        if self
            .world
            .interferences_with_aabb(&bounding_box, &self.collision_groups)
            .count()
            > 0
            || self
                .world
                .interferences_with_aabb(&bounding_box_mirrored, &self.collision_groups)
                .count()
                > 0
            || bounding_box.intersects(&bounding_box_mirrored)
        {
            None
        } else {
            self.add_sugar_hill(position);
            self.add_sugar_hill(mirrored);
            Some(())
        }
    }

    pub fn add_ant(&mut self, position: Isometry2<f32>, team: Team) {
        let (handle, _) = self.world.add(
            position,
            self.ant_shape.clone(),
            self.collision_groups,
            GeometricQueryType::Contacts(0.0, 0.0),
            MapItem::Ant(Ant {
                pose: Pose::zero(),
                team,
                hp: 1.,
                velocity: 0.,
            }),
        );
        self.ants.push(handle);
    }

    pub fn try_add_ant(&mut self, position: Isometry2<f32>, team: Team) -> Option<()> {
        let bounding_box = bounding_volume::aabb(&Ball::new(ANT_RADIUS), &position);
        if self
            .world
            .interferences_with_aabb(&bounding_box, &self.collision_groups)
            .count()
            > 0
        {
            None
        } else {
            self.add_ant_hill(position, team);
            Some(())
        }
    }

    pub fn add_raspberry(&mut self, position: Isometry2<f32>) {
        let (handle, _) = self.world.add(
            position,
            self.raspberry_shape.clone(),
            self.collision_groups,
            GeometricQueryType::Contacts(0.0, 0.0),
            MapItem::Raspberry(Pose::zero()),
        );
        self.raspberries.push(handle);
    }

    pub fn try_add_raspberry(&mut self, position: Isometry2<f32>) -> Option<()> {
        let bounding_box = bounding_volume::aabb(&Ball::new(RASPBERRY_RADIUS), &position);
        if self
            .world
            .interferences_with_aabb(&bounding_box, &self.collision_groups)
            .count()
            > 0
        {
            None
        } else {
            self.add_raspberry(position);
            Some(())
        }
    }

    pub fn add_raspberry_mirrored(&mut self, position: Isometry2<f32>) -> Option<()> {
        let mirrored = position.mirror();
        let geometry = Ball::new(RASPBERRY_RADIUS);
        let bounding_box = bounding_volume::aabb(&geometry, &position);
        let bounding_box_mirrored = bounding_volume::aabb(&geometry, &mirrored);
        if self
            .world
            .interferences_with_aabb(&bounding_box, &self.collision_groups)
            .count()
            > 0
            || self
                .world
                .interferences_with_aabb(&bounding_box_mirrored, &self.collision_groups)
                .count()
                > 0
            || bounding_box.intersects(&bounding_box_mirrored)
        {
            None
        } else {
            self.add_raspberry(position);
            self.add_raspberry(mirrored);
            Some(())
        }
    }

    pub fn add_smell(&mut self, position: Isometry2<f32>, code: u32, team: Team) {
        let (handle, _) = self.world.add(
            position,
            self.smell_cloud_shape.clone(),
            self.smell_collision_groups,
            GeometricQueryType::Contacts(0.0, 0.0),
            MapItem::SmellCloud(SmellCloud {
                position: position.convert(),
                code,
                age: 0,
                team,
            }),
        );
        self.smell_clouds.push(handle);
    }

    pub fn snapshot(&self) -> Frame {
        let mut anthills = Vec::new();
        let mut sugar_hills = Vec::new();
        let mut ants = Vec::new();
        let mut raspberries = Vec::new();
        let mut smells_clouds = Vec::new();
        for (_, object) in self.world.collision_objects() {
            match object.data() {
                MapItem::Anthill(anthill) => {
                    let mut candidate = anthill.clone();
                    candidate.pose = object.position().convert();
                    anthills.push(candidate)
                }
                MapItem::Ant(ant) => {
                    let mut candidate = ant.clone();
                    candidate.pose = object.position().convert();
                    ants.push(candidate)
                }
                MapItem::SugarHill(sugar_hill) => {
                    let mut candidate = sugar_hill.clone();
                    candidate.pose = object.position().convert();
                    sugar_hills.push(candidate)
                }
                MapItem::SmellCloud(smells_cloud) => {
                    let mut candidate = smells_cloud.clone();
                    candidate.position = object.position().convert();
                    smells_clouds.push(candidate)
                }
                MapItem::Raspberry(_) => raspberries.push(object.position().convert()),
            }
        }
        Frame {
            ants,
            anthills,
            raspberries,
            sugar_hills,
            smells_clouds,
        }
    }
}

#[cfg(test)]
mod specs {
    use super::*;
    #[test]
    fn mirrored_self_collision() {
        let mut world = World::new(32., 32.);
        let result = world.try_add_ant_hill_mirrored(
            Isometry2::new(nalgebra::Vector2::new(0., 0.), nalgebra::zero()),
            0,
        );
        assert!(result.is_none());
    }

    #[test]
    fn mirrored_fine() {
        let mut world = World::new(32., 32.);
        let result = world.try_add_ant_hill_mirrored(
            Isometry2::new(nalgebra::Vector2::new(3., 2.), nalgebra::zero()),
            0,
        );
        assert!(result.is_some());
    }
}

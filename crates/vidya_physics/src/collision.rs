use bevy_ecs::prelude::*;
use bevy_math::prelude::*;
//use bevy_macro_utils::*;
use bevy_reflect::prelude::*;

use crate::{PhysObj, AABB, Shape, VoxelChunk};

/// Represents a group that a physics object can belong to.
pub type CollisionGroups = u32;

pub const GROUP_NONE: CollisionGroups =                 0;
pub const GROUP_ALL: CollisionGroups =                  u32::MAX;
pub const GROUP_PARTICLES: CollisionGroups =            0b00000000_00000000_00000000_00000001;
pub const GROUP_STATIC_TERRAIN: CollisionGroups =       0b00000000_00000000_00000000_00000010;
pub const GROUP_MOVING_TERRAIN: CollisionGroups =       0b00000000_00000000_00000000_00000100;
pub const GROUP_BASIC: CollisionGroups =                0b00000000_00000000_00000000_00001000;


/// Represents a collision that occurred between two physics objects
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Collision {
    /// Value between 0 and 1 describing when during a substep the collision happened
    pub t: f32,
    /// Amount object B's position should change
    pub position_delta: Vec3,
    /// Amount object B's velocity should change
    pub velocity_delta: Vec3,
    /// Normal of the surface hit on object A
    pub normal_a: Vec3,
    /// Normal of the surface hit on object B
    pub normal_b: Vec3
}

impl CollisionResponse {
    pub fn weighted(collision: &Collision, weight_a: f32, weight_b: f32) -> (CollisionResponse, CollisionResponse) {
        let total = weight_a + weight_b;
        let a_ratio = weight_a / total;
        let b_ratio = weight_b / total;
        let a_response = CollisionResponse::Value {
            t: collision.t,
            position_delta: -collision.position_delta * b_ratio,
            velocity_delta: -collision.velocity_delta * b_ratio,
            surface_normal: collision.normal_b
        };
        let b_response = CollisionResponse::Value {
            t: collision.t,
            position_delta: collision.position_delta * a_ratio,
            velocity_delta: collision.velocity_delta * a_ratio,
            surface_normal: collision.normal_a
        };
        (a_response, b_response)
    }
    pub fn for_a(collision: &Collision) -> CollisionResponse {
        CollisionResponse::Value {
            t: collision.t,
            position_delta: -collision.position_delta,
            velocity_delta: -collision.velocity_delta,
            surface_normal: collision.normal_b
        }
    }
    pub fn for_b(collision: &Collision) -> CollisionResponse {
        CollisionResponse::Value {
            t: collision.t,
            position_delta: collision.position_delta,
            velocity_delta: collision.velocity_delta,
            surface_normal: collision.normal_a
        }
    }
}

/// Represents the response to a collision
#[derive(Component, Copy, Clone, PartialEq, Debug, Default, Reflect)]
pub enum CollisionResponse {
    #[default]
    Empty,
    Value {
        /// Value between 0 and 1 describing when during a substep the collision happened
        t: f32,
        /// Amount position should change
        position_delta: Vec3,
        /// Amount object B's velocity should change
        velocity_delta: Vec3,
        /// Normal of the surface of the other object hit
        surface_normal: Vec3
    }
}

impl CollisionResponse {
    pub fn is_closer(&self, other: &Self) -> bool {
        match (self, other) {
            (CollisionResponse::Value { .. }, CollisionResponse::Empty) => true,
            (CollisionResponse::Value { t: ta, .. }, CollisionResponse::Value { t: tb, ..}) => ta < tb,
            _ => false
        }
    }
}


/// Stores information about how a physics object should behave during a collision.
#[derive(Component, Copy, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub struct CollisionConfig {
    /// Group(s) a physics object belongs to. It's typically only one.
    pub groups: CollisionGroups,
    /// Groups this physics object is affected by.
    pub affected_by: CollisionGroups
}
impl CollisionConfig {
    pub fn new(groups: CollisionGroups, affected_by: CollisionGroups) -> Self {
        Self {
            groups,
            affected_by
        }
    }
    pub fn not_affected_by(&self, affected_by: CollisionGroups) -> Self {
        Self {
            groups: self.groups,
            affected_by: self.affected_by & !affected_by
        }
    }
    pub fn affected_by(&self, groups: CollisionGroups) -> bool {
        self.affected_by & groups != 0
    }
}

pub(crate) fn collide(a: PhysObj<'_>, b: PhysObj<'_>) -> Option<Collision> {
    let b_vel = b.vel - a.vel;
    match (a.shape, b.shape) {
        (Shape::Cuboid, Shape::Cuboid) => collide_cuboid_cuboid(a.aabb, b.aabb, b_vel),
        (Shape::VoxelChunk(chunk), Shape::Cuboid) => collide_chunk_cuboid(a.aabb, chunk, b.aabb, b_vel),
        _ => None
    }
}

pub(crate) fn collide_cuboid_cuboid(a_bounds: AABB, b_bounds: AABB, bevy_vel: Vec3) -> Option<Collision> {
    None
}

pub(crate) fn collide_chunk_cuboid(a_bounds: AABB, a_chunk: &VoxelChunk, b_bounds: AABB, bevy_vel: Vec3) -> Option<Collision> {
    None
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn affected_by() {
        let config = CollisionConfig::new(GROUP_BASIC, GROUP_ALL)
            .not_affected_by(GROUP_BASIC | GROUP_PARTICLES);

        assert!(!config.affected_by(GROUP_BASIC));
        assert!(!config.affected_by(GROUP_PARTICLES));

        assert!(config.affected_by(GROUP_STATIC_TERRAIN));
        assert!(config.affected_by(GROUP_MOVING_TERRAIN));
    }
    
}
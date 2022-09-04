use std::collections::HashMap;
use std::ops::{Neg, Sub, Add};

use vidya_fixed_timestep::FixedTimestepStages;
pub use vidya_fixed_timestep::{CurrentTransform, PreviousTransform};
use bevy_transform::prelude::*;
use bevy_ecs::prelude::*;
use bevy_app::prelude::*;
use bevy_math::prelude::*;
mod voxel;
pub use voxel::*;

#[cfg(feature = "debug")]
pub mod debug;

/// Adds a simple platformer voxel-based physics engine.
pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        // Runs physics systems before interpolation
        app
            .add_system_set_to_stage(FixedTimestepStages::PostFixedUpdate, SystemSet::new()
                .with_system(apply_gravity
                    .label(PhysicsSystems::ApplyGravity)
                )
                .with_system(apply_friction
                    .label(PhysicsSystems::ApplyFriction)
                    .after(PhysicsSystems::ApplyGravity)
                )
                .with_system(apply_velocity.label(PhysicsSystems::ApplyVelocity)
                    .after(PhysicsSystems::ApplyFriction)
                )
                .with_system(apply_voxel_collisions
                    .label(PhysicsSystems::ApplyVoxelCollisions)
                    .after(PhysicsSystems::ApplyVelocity)
                )
            );
    }
}

//////////////////////////////////////////////// Labels ////////////////////////////////////////////////
#[derive(Debug, Copy, Clone, Eq, PartialEq, SystemLabel)]
pub enum PhysicsSystems {
    /// Applies friction to velocity
    ApplyFriction,
    /// Applies gravity to velocity
    ApplyGravity,
    /// Applies velocity to position
    ApplyVelocity,
    /// Applies voxel collisions (moving entities w/ static terrain chunks)
    ApplyVoxelCollisions,
    /// Linearly interpolates transform components between Positions and PreviousPositions
    LerpTransform
}


//////////////////////////////////////////////// Resources ////////////////////////////////////////////////

/// Resource that stores the gravity of the situation ;)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Gravity(pub Vec3);
impl Default for Gravity {
    fn default() -> Self {
        Self(Vec3::new(0.0, -1.0, 0.0))
    }
}

/// Resource that organizes entities into chunks that they occupy
pub struct SpatialHash {
    chunk_size: UVec3,
    chunks: HashMap<IVec3, SpatialChunk>
}

impl SpatialHash {

    // Adds an entity to the hash
    fn add_entity(&mut self, chunk_coords: IVec3, entity: Entity) {
        let chunk = self.chunks
            .entry(chunk_coords)
            .or_insert_with(|| SpatialChunk(Vec::new()));
        chunk.0.push(entity);
    }

    // Removes an entity from the hash
    fn remove_entity(&mut self, chunk_coords: IVec3, entity: Entity) {
        let chunk = match self.chunks.get_mut(&chunk_coords) {
            Some(chunk) => chunk,
            None => return
        };
        let index = chunk.0.iter().position(|e| *e == entity);
        match index {
            Some(index) => {
                chunk.0.remove(index);
                if chunk.0.is_empty() {
                    self.chunks.remove(&chunk_coords);
                }
            },
            None => {}
        }
    }
}

/// Chunk in a spatial hash
struct SpatialChunk(Vec<Entity>);


//////////////////////////////////////////////// Components ////////////////////////////////////////////////

/// Velocity of an [`Entity`].
#[derive(Component, Debug, Copy, Clone, PartialEq, Default)]
pub struct Velocity(pub Vec3);

/// Represents the shape of an [`Entity`].
#[derive(Component, Debug, Clone, PartialEq, Default)]
pub enum PhysicsShape {
    #[default]
    Cuboid,
    Capsule
}

/// Represents the bounds of an unscaled [`Entity`].
#[derive(Component, Debug, Copy, Clone, PartialEq, Default)]
pub struct Bounds {
    /// Size of half the width, half the height and half the depth of the AABB.
    pub half_extents: Vec3
}
impl Bounds {
    pub fn new(size: Vec3) -> Self {
        Self { half_extents: size / 2.0 }
    }
    pub fn size(&self) -> Vec3 {
        self.half_extents * 2.0
    }
}


/// Frictional value of an [`Entity`].
/// Used to dampen movement.
#[derive(Component, Debug, Copy, Clone, PartialEq)]
pub struct Friction(pub Vec3);
impl Friction {
    pub fn new(value: f32) -> Self {
        Self(Vec3::new(value, value, value))
    }
}
impl Default for Friction {
    fn default() -> Self {
        Self(Vec3::ONE)
    }
}

/// Marker component that lets the interpolation plugin select the correct entities.
/// If an [`Entity`] has this, users of that entity should not manipulate [`Transform`]
/// directly and should instead manipulate [`CurrentTransform`] (and sometimes [`PreviousTransform`]).
#[derive(Component, Default, Debug, Copy, Clone, PartialEq)]
pub struct PhysicsInterpolate;

//////////////////////////////////////////////// Bundle(s) ////////////////////////////////////////////////

/// Bundle of all the components needed for an [`Entity`] to partake in a physics simulation
#[derive(Bundle, Default, Debug, Clone, PartialEq)]
pub struct PhysicsBundle {
    pub current_transform: CurrentTransform,
    pub previous_transform: PreviousTransform,
    pub bounds: Bounds,
    pub shape: PhysicsShape,
    pub velocity: Velocity,
    pub friction: Friction,
    pub physics_marker: PhysicsInterpolate
}
impl PhysicsBundle {
    pub fn new(transform: Transform, bounds: Bounds, shape: PhysicsShape) -> Self {
        Self {
            current_transform: CurrentTransform(transform),
            previous_transform: PreviousTransform(transform),
            bounds,
            shape,
            ..Self::default()
        }
    }
    pub fn with_velocity(mut self, velocity: Velocity) -> Self {
        self.velocity = velocity;
        self
    }
    pub fn with_friction(mut self, friction: Friction) -> Self {
        self.friction = friction;
        self
    }
}

//////////////////////////////////////////////// Helper struct(s) ////////////////////////////////////////////////

/// Represents the movement of an [`Entity`] through 3D space
#[derive(Debug, Clone, PartialEq)]
pub struct Movement<'a> {
    /// Bounding box of the moving object
    pub aabb: AABB,
    /// Shape of the body
    pub shape: &'a PhysicsShape,
    /// Velocity of the object
    pub vel: Vec3,
}

/// Helper struct that defines an axis-aligned bounding box
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AABB {
    pub center: Vec3,
    pub half_extents: Vec3
}
impl AABB {
    /// Width, height and depth of AABB
    pub fn size(&self) -> Vec3 {
        self.half_extents * 2.0
    }
    pub fn left(&self) -> f32 {
        self.center.x - self.half_extents.x
    }
    pub fn right(&self) -> f32 {
        self.center.x + self.half_extents.x
    }
    pub fn bottom(&self) -> f32 {
        self.center.y - self.half_extents.y
    }
    pub fn top(&self) -> f32 {
        self.center.y + self.half_extents.y
    }
    pub fn near(&self) -> f32 {
        self.center.z + self.half_extents.z
    }
    pub fn far(&self) -> f32 {
        self.center.z - self.half_extents.z
    }
    pub fn set_left(&mut self, left: f32) {
        self.center.x = left + self.half_extents.x;
    }
    pub fn set_right(&mut self, right: f32) {
        self.center.x = right - self.half_extents.x;
    }
    pub fn set_bottom(&mut self, bottom: f32) {
        self.center.y = bottom + self.half_extents.y;
    }
    pub fn set_top(&mut self, top: f32) {
        self.center.y = top - self.half_extents.y;
    }
    pub fn set_near(&mut self, near: f32) {
        self.center.z = near - self.half_extents.z;
    }
    pub fn set_far(&mut self, far: f32) {
        self.center.z = far + self.half_extents.z;
    }
}

//////////////////////////////////////////////// Systems ////////////////////////////////////////////////

/// Applies gravity if there is a gravity resource.
/// Should not run if gravity resource not present.
fn apply_gravity(
    gravity: Option<Res<Gravity>>,
    mut velocities: Query<&mut Velocity>
) {
    let gravity = match gravity {
        Some(gravity) => gravity,
        None => return
    };
    for mut vel in &mut velocities {
        vel.0 += gravity.0;
    }
}

/// Dampens velocities by frictional value
fn apply_friction(mut entities: Query<(&mut Velocity, &Friction)>) {
    for (mut vel, fric) in &mut entities {
        vel.0 *= fric.0;
    }
}

/// Moves entities by velocities
fn apply_velocity(mut entities: Query<(&Velocity, &mut CurrentTransform)>) {
    for (vel, mut transform) in &mut entities {
        transform.0.translation += vel.0;
    }
}

/// Applies voxel collision code
fn apply_voxel_collisions() {
    // TODO
}
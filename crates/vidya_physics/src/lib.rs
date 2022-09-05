mod collision;
mod voxel;

use std::collections::HashMap;
use std::ops::{Neg, Sub, Add};

use vidya_fixed_timestep::FixedTimestepStages;
pub use vidya_fixed_timestep::{CurrentTransform, PreviousTransform};
use bevy_transform::prelude::*;
use bevy_ecs::prelude::*;
use bevy_app::prelude::*;
use bevy_math::prelude::*;

pub use voxel::*;
pub use collision::*;


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
                .with_system(shape_to_chunk_collisions
                    .label(PhysicsSystems::ShapeToChunkCollisions)
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
    ShapeToChunkCollisions,
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
#[derive(Component, Debug, Copy, Clone, PartialEq, Default)]
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

/// Applies collisions of [`PhysicsShape`]s to [`VoxelChunk`]s.
/// Only [`PhysicsShape`]s are affected by collisions.
/// [`VoxelChunk`]s do not change their path due to collisions.
fn shape_to_chunk_collisions(
    mut shape_query: Query<(&PhysicsShape, &mut PreviousTransform, &mut CurrentTransform, &mut Velocity, &mut Bounds)>,
    chunk_query: Query<(&VoxelChunk, &PreviousTransform, &CurrentTransform, &mut Bounds), Without<PhysicsShape>>
) {
    const SHAPE_RETRIES: usize = 8;
    
    // For all "shapes"...
    for (shape, mut shape_prev_transf, mut shape_curr_transf, mut shape_vel, shape_bounds) in &mut shape_query {

        // Makes local copies of shape data and starts the retry loop
        let mut sh_vel = shape_vel.0;
        let mut sh_prev_transl = shape_prev_transf.0.translation;
        let mut sh_curr_transl = shape_curr_transf.0.translation;
        for _ in 0..SHAPE_RETRIES {

            let sh_dir = sh_curr_transl - sh_prev_transl;   // Direction/velocity of current shape of this iteration
            let mut closest_coll: Option<Collision> = None; // Closest collision of current shape of this iteration

            // Collide all voxel chunks with the current shape and store the closest one found
            for (chunk, chunk_prev_transf, chunk_curr_transf, chunk_bounds) in &chunk_query {

                // Package shape/chunk data into structs
                let shape_data = ShapeData {
                    shape: *shape,
                    src: sh_prev_transl,
                    dir: sh_dir,
                    half_extents: shape_bounds.half_extents
                };
                let chunk_data = VoxelChunkData {
                    chunk,
                    src: chunk_prev_transf.0.translation,
                    dir: chunk_curr_transf.0.translation - chunk_prev_transf.0.translation,
                    half_extents: chunk_bounds.half_extents
                };

                // Perform shape/chunk collision and store it if it's closer than the existing collision.
                let coll = collide_shape_with_chunk(shape_data, chunk_data);
                closest_coll = match (&coll, &closest_coll) {
                    (Some(coll_ref), None) => Some(*coll_ref),
                    (Some(coll_ref), Some(closest_coll_ref)) => {
                        if coll_ref.t < closest_coll_ref.t { Some(*coll_ref) }
                        else { continue }
                    },
                    _ => continue
                }
            }

            // Apply closest collision to local shape data.
            // If no collision, break out of the retry loop since we're done with this shape.
            let closest_coll = match &closest_coll {
                Some(coll) => coll,
                None => break
            };
            let travel_vec = {  // Direction shape should travel after collision occurs. Length is between 0.0 and 1.0 depending on angle.
                let up = closest_coll.normal;
                let left = sh_dir.cross(up);
                let travel_dir = up.cross(left).normalize();
                let cos_angle = closest_coll.normal.dot(-sh_dir.normalize());
                travel_dir * (1.0 - cos_angle.abs())
            };
            let remaining_t = 1.0 - closest_coll.t.clamp(0.0, 1.0);
            sh_vel = travel_vec * sh_vel.length();                  // Sets local velocity
            sh_prev_transl += sh_dir * closest_coll.t;              // Moves local shape's prev_trans to collision point
            sh_curr_transl = sh_prev_transl + sh_vel*remaining_t;   // Moves local shape's curr_trans somewhere ahead of prev_trans
        }

        // Copies local shape data back into original shape data
        shape_prev_transf.0.translation = sh_prev_transl;
        shape_curr_transf.0.translation = sh_curr_transl;
        shape_vel.0 = sh_vel;
    }
}
mod voxel;

use std::ops::{Neg, Sub, Add};
use std::time::Duration;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;

use bevy::time::{FixedTimestep, FixedTimesteps};

use voxel::*;

const PHYSICS_TIMESTEP: &str = "PHYSICS_TIMESTEP";


/// Adds a simple platformer voxel-based physics engine.
/// All systems are added to the [`CoreStage::PostUpdate`] stage, so the setting of positions, velocities, etc
/// should be done in [`CoreStage::Update`] or prior for optimal results.
pub struct PhysicsPlugin {
    pub timestep_duration: Duration
}
impl Default for PhysicsPlugin {
    fn default() -> Self {
        Self {
            timestep_duration: Duration::from_secs_f64(1.0/60.0)
        }
    }
}
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        let timestep = self.timestep_duration.as_secs_f64();
        app
            .add_system_to_stage(CoreStage::PostUpdate, sync_positions
                .label(PhysicsSystems::SyncPositions)
                .with_run_criteria(FixedTimestep::step(timestep).with_label(PHYSICS_TIMESTEP))
            )
            .add_system_to_stage(CoreStage::PostUpdate, apply_gravity
                .label(PhysicsSystems::ApplyGravity)
                .with_run_criteria(FixedTimestep::step(timestep))
            )
            .add_system_to_stage(CoreStage::PostUpdate, apply_friction
                .label(PhysicsSystems::ApplyFriction)
                .after(PhysicsSystems::ApplyGravity)
                .with_run_criteria(FixedTimestep::step(timestep))
            )
            .add_system_to_stage(CoreStage::PostUpdate, apply_velocity.label(PhysicsSystems::ApplyVelocity)
                .after(PhysicsSystems::SyncPositions)
                .after(PhysicsSystems::ApplyFriction)
                .with_run_criteria(FixedTimestep::step(timestep))
            )
            .add_system_to_stage(CoreStage::PostUpdate, apply_voxel_collisions
                .label(PhysicsSystems::ApplyVoxelCollisions)
                .after(PhysicsSystems::ApplyVelocity)
                .with_run_criteria(FixedTimestep::step(timestep))
            )
            .add_system_to_stage(CoreStage::PostUpdate, lerp_transform
                .label(PhysicsSystems::LerpTransform)
                .after(PhysicsSystems::ApplyVoxelCollisions)
            );
    }
}

//////////////////////////////////////////////// Labels ////////////////////////////////////////////////

#[derive(StageLabel)]
struct PhysicsStage;

#[derive(Debug, Copy, Clone, Eq, PartialEq, SystemLabel)]
pub enum PhysicsSystems {
    /// Syncs previous position with current position
    SyncPositions,
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


//////////////////////////////////////////////// Components ////////////////////////////////////////////////

/// Transform state of an [`Entity`] in the current game tick
#[derive(Component, Debug, PartialEq, Clone, Copy, Reflect)]
pub struct CurrentTransform(pub Transform);

/// Transform state of an [`Entity`] in the previous game tick
#[derive(Component, Debug, PartialEq, Clone, Copy, Reflect)]
pub struct PreviousTransform(pub Transform);

/// Previous center of an [`Entity`] during the last tick.
#[derive(Component, Debug, Copy, Clone, PartialEq, Default)]
pub struct PreviousPosition(pub Vec3);

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

//////////////////////////////////////////////// Bundle(s) ////////////////////////////////////////////////

/// Bundle of all the components needed for an [`Entity`]
/// to partake in a physics simulation.
#[derive(Bundle, Debug, Copy, Clone, PartialEq)]
pub struct PhysicsBundle {
    pub current_transform: CurrentTransform,
    pub previous_transform: PreviousPosition,
    pub bounds: Bounds,
    pub shape: PhysicsShape,
    pub velocity: Velocity,
    pub friction: Friction
}

//////////////////////////////////////////////// Helper struct(s) ////////////////////////////////////////////////

/// Represents the movement of an [`Entity`] through 3D space.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Movement {
    /// Bounding box of the moving object
    pub aabb: AABB,
    /// Shape of the body
    pub shape: PhysicsShape,
    /// Velocity of the object
    pub vel: Vec3,
}

/// Helper struct that defines an axis-aligned bounding box.
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

/// Synchronizes previous position with current position.
fn sync_positions(mut entities: Query<(&mut CurrentTransform, &mut PreviousTransform)>) {
    for (transform, mut prev_transform) in &mut entities {
        prev_transform.0 = transform.0;
    }
}

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

/// Dampens velocities by frictional value.
fn apply_friction(mut entities: Query<(&mut Velocity, &Friction)>) {
    for (mut vel, fric) in &mut entities {
        vel.0 *= fric.0;
    }
}

/// Moves entities by velocities.
fn apply_velocity(mut entities: Query<(&Velocity, &mut CurrentTransform)>) {
    for (vel, mut transform) in &mut entities {
        transform.0.translation += vel.0;
    }
}

/// Applies voxel collision code
fn apply_voxel_collisions() {
    // TODO
}

/// Linearly interpolates transforms between [`PreviousPosition`] and [`Position`] components.
fn lerp_transform(
    timesteps: Res<FixedTimesteps>,
    mut entities: Query<(&mut Transform, &PreviousPosition, &CurrentTransform)>
) {
    let t = timesteps
        .get(PHYSICS_TIMESTEP)
        .unwrap()
        .overstep_percentage() as f32;
    for (mut transform, prev_pos, pos) in &mut entities {
        transform.translation = prev_pos.0.lerp(pos.0, t);
    }
}
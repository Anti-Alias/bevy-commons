use std::ops::{Neg, Sub, Add};

use vidya_fixed_timestep::FixedTimestepStages;
pub use vidya_fixed_timestep::{CurrentTransform, PreviousTransform};
use bevy_transform::prelude::*;
use bevy_app::prelude::*;
use bevy_math::prelude::*;
use bevy_reflect::prelude::*;
use bevy_ecs::prelude::*;


mod voxel;
mod collision;
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
            .register_type::<Velocity>()
            .register_type::<Shape>()
            .register_type::<Weight>()
            .register_type::<HalfExtents>()
            .register_type::<Friction>()
            .register_type::<PhysicsInterpolate>()
            .register_type::<CollisionResponse>()
            .register_type::<AntiGravity>()
            .init_resource::<PhysicsConfig>()
            .add_system_set_to_stage(FixedTimestepStages::PostFixedUpdate, SystemSet::new()
                .with_system(apply_gravity
                    .label(PhysicsSystems::ApplyGravity)
                )
                .with_system(apply_friction
                    .label(PhysicsSystems::ApplyFriction)
                    .after(PhysicsSystems::ApplyGravity)
                )
                .with_system(update
                    .label(PhysicsSystems::Update)
                    .after(PhysicsSystems::ApplyFriction)
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
    Update,
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

/// Velocity of an [`Entity`].
#[derive(Component, Debug, Copy, Clone, PartialEq, Default, Reflect)]
#[reflect(Component)]
pub struct Velocity(pub Vec3);

/// Represents the shape of an [`Entity`].
#[derive(Component, Debug, Default, Clone, Reflect)]
#[reflect(Component)]
pub enum Shape {
    #[default]
    Cuboid,
    Capsule,
    VoxelChunk(VoxelChunk)
}

/// Weight of an [`Entity`].
#[derive(Component, Debug, Copy, Clone, PartialEq, Reflect)]
pub struct Weight(pub f32);
impl Default for Weight {
    fn default() -> Self {
        Weight(1.0)
    }
}

/// Represents the bounds of an unscaled [`Entity`].
#[derive(Component, Debug, Copy, Clone, PartialEq, Default, Reflect)]
#[reflect(Component)]
pub struct HalfExtents(pub Vec3);
impl HalfExtents {
    pub fn new(width: f32, height: f32, depth: f32) -> Self {
        Self(Vec3::new(width, height, depth) / 2.0)
    }
    pub fn width(&self) -> f32 {
        self.0.x * 2.0
    }
    pub fn height(&self) -> f32 {
        self.0.y * 2.0
    }
    pub fn depth(&self) -> f32 {
        self.0.z * 2.0
    }
    pub fn size(&self) -> Vec3 {
        self.0 * 2.0
    }
}

// Marker component that prevents an [`Entity`] from being affected by gravity.
#[derive(Component, Debug, Copy, Clone, Default, Reflect)]
#[reflect(Component)]
pub struct AntiGravity;


/// Frictional value of an [`Entity`].
/// Used to dampen movement.
#[derive(Component, Debug, Copy, Clone, PartialEq, Reflect)]
#[reflect(Component)]
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
#[derive(Component, Default, Debug, Copy, Clone, PartialEq, Reflect)]
#[reflect(Component)]
pub struct PhysicsInterpolate;

//////////////////////////////////////////////// Bundle(s) ////////////////////////////////////////////////

/// Bundle of all the components needed for an [`Entity`] to partake in a physics simulation
#[derive(Bundle, Default, Debug, Clone)]
pub struct PhysicsBundle {
    pub current_transform: CurrentTransform,
    pub previous_transform: PreviousTransform,
    pub bounds: HalfExtents,
    pub shape: Shape,
    pub weight: Weight,
    pub config: CollisionConfig,
    pub velocity: Velocity,
    pub friction: Friction,
    pub physics_marker: PhysicsInterpolate,
    pub collision_response: CollisionResponse
}
impl PhysicsBundle {
    pub fn new(transform: Transform, bounds: HalfExtents, shape: Shape) -> Self {
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

/// Represents a moving physics object
#[derive(Debug, Clone)]
pub struct PhysObj<'a> {
    /// Bounding box of the moving object
    pub aabb: AABB,
    /// Shape of the body
    pub shape: &'a Shape,
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
    pub fn new(center: Vec3, half_extents: Vec3) -> Self {
        Self { center, half_extents }
    }
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
    pub fn interp(&self, t: f32, dir: Vec3) -> Self {
        Self::new(self.center + dir * t, self.half_extents)
    }
    pub fn intersects_xz(&self, other: &Self) -> bool {
        self.left() < other.right() &&
        self.right() > other.left() &&
        self.far() < other.near() &&
        self.near() > other.far()
    }
    pub fn intersects(&self, other: &Self) -> bool {
        self.left() < other.right() &&
        self.right() > other.left() &&
        self.bottom() < other.top() &&
        self.top() > other.bottom() &&
        self.far() < other.near() &&
        self.near() > other.far()
    }
}

//////////////////////////////////////////////// Systems ////////////////////////////////////////////////

/// Applies gravity to all physics objects.
fn apply_gravity(
    gravity: Option<Res<Gravity>>,
    mut velocities: Query<&mut Velocity, Without<AntiGravity>>
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

/// Moves entities with substeps, then applies collisions.
fn update(
    config: Res<PhysicsConfig>,
    mut physics_objects: Query<(
        &mut CurrentTransform,
        &mut Velocity,
        &mut HalfExtents,
        &Shape,
        &Weight,
        &CollisionConfig,
        &mut CollisionResponse
    )>
) {

    // For each substep...
    let steps = config.substeps as f32;
    let inv_steps = 1.0 / steps;
    for _ in 0..config.substeps {

        // Computes collisions between objects
        let mut combinations = physics_objects.iter_combinations_mut();
        while let Some([obj_a, obj_b]) = combinations.fetch_next() {
            let (a_trans, a_vel, a_ext, a_shape, a_weight, a_cfg, mut a_resp) = obj_a;
            let (b_trans, b_vel, b_ext, b_shape, b_weight, b_cfg, mut b_resp) = obj_b;

            // Quits early if neither object are affected by each other
            let a_affected = a_cfg.affected_by(b_cfg.groups);
            let b_affected = b_cfg.affected_by(a_cfg.groups);
            if !a_affected && !b_affected {
                continue;
            }

            // Computes collision between a and b
            let coll = collide(
                PhysObj {
                    aabb: AABB::new(a_trans.0.translation, a_ext.0),
                    shape: a_shape,
                    vel: a_vel.0 * inv_steps
                },
                PhysObj {
                    aabb: AABB::new(b_trans.0.translation, b_ext.0),
                    shape: b_shape,
                    vel: b_vel.0 * inv_steps
                }
            );

            // If collision found, distribute the response to a and b
            if let Some(coll) = coll {
                let (resp_a, resp_b) = match (a_affected, b_affected) {
                    (false, false) => continue,
                    (false, true) => (CollisionResponse::Empty, CollisionResponse::for_b(&coll)),
                    (true, false) => (CollisionResponse::for_a(&coll), CollisionResponse::Empty),
                    (true, true) => CollisionResponse::weighted(&coll, a_weight.0, b_weight.0)
                };
                if resp_a.is_closer(&a_resp) {
                    *a_resp = resp_a;
                }
                if resp_b.is_closer(&b_resp) {
                    *b_resp = resp_b;
                }
            }
        }

        // Applies collision responses and updates velocities
        for (mut trans, mut vel, _, _, _, _, mut resp) in &mut physics_objects {
            match *resp {
                CollisionResponse::Empty => {
                    trans.0.translation += vel.0 * inv_steps;
                },
                CollisionResponse::Value { position_delta, velocity_delta, .. } => {
                    trans.0.translation += vel.0 * inv_steps + position_delta;
                    vel.0 += velocity_delta;
                    *resp = CollisionResponse::Empty;
                }
            }
        }
    }
}

/// Configuration for the physics engine
#[derive(Copy, Clone, PartialEq)]
pub struct PhysicsConfig {
    pub substeps: usize
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        Self { substeps: 4 }
    }
}
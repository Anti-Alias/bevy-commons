    mod collision;

use std::ops::{Neg, Sub, Add};
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::time::{FixedTimestep, FixedTimesteps};

use collision::cuboid_collision;
use fixed_timestep::FixedTimestepConfig;

const PHYSICS_TIMESTEP: &str = "PHYSICS_TIMESTEP";


/// Adds a simple platformer voxel-based physics engine.
/// All systems are added to the [`CoreStage::PostUpdate`] stage, so the setting of positions, velocities, etc
/// should be done in [`CoreStage::Update`] or prior for optimal results.
pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {

        let timestep = app.world
            .get_resource::<FixedTimestepConfig>()
            .expect("Missing config 'FixedTimestepConfig'")
            .timestep_duration
            .as_secs_f64();
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
                .after(apply_voxel_collisions)
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

/// Center of an [`Entity`] in 3D space.
#[derive(Component, Debug, Copy, Clone, PartialEq, Default)]
pub struct Position(pub Vec3);

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

/// Together, with [`Position`] as the center, this component represents the axis-aligned bounding box of an [`Entity`].
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
    pub position: Position,
    pub previous_position: PreviousPosition,
    pub aabb: Bounds,
    pub shape: PhysicsShape,
    pub velocity: Velocity,
    pub friction: Friction
}
impl PhysicsBundle {
    /// Creates a new physics bundle
    pub fn new(
        position: Vec3,
        size: Vec3,
        shape: PhysicsShape
    ) -> Self {
        Self {
            position: Position(position),
            previous_position: PreviousPosition(position),
            aabb: Bounds { half_extents: size / 2.0 },
            shape,
            velocity: Velocity(Vec3::ZERO),
            friction: Friction(Vec3::ONE)
        }
    }
    pub fn with_friction(mut self, friction: Vec3) -> Self {
        self.friction = Friction(friction);
        self
    }
    pub fn with_velocity(mut self, velocity: Vec3) -> Self {
        self.velocity = Velocity(velocity);
        self
    }
}

//////////////////////////////////////////////// Helper struct(s) ////////////////////////////////////////////////

/// Represents the movement of an [`Entity`] through 3D space.
pub struct Movement {
    /// Position of the body
    pub pos: Vec3,
    /// Velocity of the body
    pub vel: Vec3,
    /// Size of the body as an AABB
    pub size: Vec3,
    /// Shape of the body
    pub shape: PhysicsShape
}

/// A collider stored in a [`VoxelChunk`].
#[derive(Copy, Clone)]
pub struct Voxel {
    pub collision_fn: fn(&Bounds, &Movement)
}
impl Voxel {
    pub fn cuboid() -> Self {
        Self {
            collision_fn: cuboid_collision
        }
    }
}

/// Stores both a [`Voxel`] and its orientation.
#[derive(Copy, Clone)]
struct VoxelData {
    voxel: Voxel,
    orientation: Orientation
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Orientation {
    /// Rotation along x axis
    pub x_rot: Degree,
    /// Rotation along y axis
    pub y_rot: Degree,
    /// Rotation along z axis
    pub z_rot: Degree
}
impl Orientation {
    pub fn relative_to(&self, other: Orientation) -> Orientation {
        Orientation {
            x_rot: self.x_rot - other.x_rot,
            y_rot: self.y_rot - other.y_rot,
            z_rot: self.z_rot - other.z_rot
        }
    }
    pub fn rotate_vec(&self, mut vec: Vec3) -> Vec3 {
        vec = self.z_rot.rotate_z(vec);
        vec = self.y_rot.rotate_y(vec);
        vec = self.x_rot.rotate_x(vec);
        vec
    }
}

/// Degree of an [`Orientation`] at perfect 90 degree angles.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
pub enum Degree {
    #[default]
    Zero,
    Ninty,
    OneEighty,
    TwoSeventy
}

impl Degree {
    fn rotate(self, vec: Vec2) -> Vec2 {
        match self {
            Degree::Zero => vec,
            Degree::Ninty => Vec2 {
                x: -vec.y,
                y: vec.x
            },
            Degree::OneEighty => Vec2 {
                x: -vec.x,
                y: -vec.y
            },
            Degree::TwoSeventy => Vec2 {
                x: vec.y,
                y: -vec.x
            }
        }
    }
    fn rotate_x(self, mut vec: Vec3) -> Vec3 {
        let rotated = self.rotate(vec.yz());
        Vec3::new(vec.x, rotated.x, rotated.y)
    }
    fn rotate_y(self, mut vec: Vec3) -> Vec3 {
        let rotated = self.rotate(vec.xz());
        Vec3::new(rotated.x, vec.y, rotated.y)
    }
    fn rotate_z(self, mut vec: Vec3) -> Vec3 {
        let rotated = self.rotate(vec.xy());
        Vec3::new(rotated.x, rotated.y, vec.z)
    }
    fn to_num(self) -> usize {
        match self {
            Zero => 0,
            Ninty => 1,
            OneEighty => 2,
            TwoSeventy => 3
        }
    }
    fn from_num(num: usize) -> Degree {
        let num = num % 4;
        match num {
            0 => Degree::Zero,
            1 => Degree::Ninty,
            2 => Degree::OneEighty,
            3 => Degree::TwoSeventy,
            _ => panic!("Invalid number {}", num)
        }
    }
}
impl Add for Degree {
    type Output = Degree;
    fn add(self, rhs: Self) -> Self::Output {
        Degree::from_num(self.to_num() + rhs.to_num())
    }
}
impl Sub for Degree {
    type Output = Degree;
    fn sub(self, rhs: Self) -> Self::Output {
        Degree::from_num(self.to_num() - rhs.to_num())
    }
}
impl Neg for Degree {
    type Output = Degree;
    fn neg(self) -> Self::Output {
        match self {
            Self::Zero => Self::OneEighty,
            Self::Ninty => Self::TwoSeventy,
            Self::OneEighty => Self::Zero,
            Self::TwoSeventy => Self::Ninty
        }
    }
}

/// Represents a chunk of [`Voxel`]s.
/// Metadata about each chunk spawned i in [`VoxelChunks`].
#[derive(Component)]
pub struct VoxelChunk(Vec<Option<VoxelData>>);

/// Resource that keeps track of entities with [`VoxelChunk`]s.
pub struct VoxelChunks {
    size: UVec3,
    voxel_size: Vec3
}
impl VoxelChunks {
    // Creates a new terrain chunk, allocating empty voxels.
    pub fn new(size: UVec3, voxel_size: Vec3) -> Self {
        if voxel_size.x < 0.0 || voxel_size.y < 0.0 || voxel_size.z < 0.0 {
            panic!("Invalid collider size: {}", voxel_size);
        }
        Self {
            size,
            voxel_size
        }
    }

    /// Size of each voxel in units.
    pub fn voxel_size(&self) -> Vec3 {
        self.voxel_size
    }

    /// Size of chunk in voxels.
    pub fn size(&self) -> UVec3 {
        self.size
    }

    /// Size of chunk in units.
    pub fn unit_size(&self) -> Vec3 {
        self.size.as_vec3() * self.voxel_size
    }

    // /// Gets voxel from this chunk.
    // /// Returns None if out of bounds, or voxel was not present at the coordinates specified.
    // pub fn get_voxel(&self, coords: UVec3, chunk: &VoxelChunk) -> Option<&Voxel> {
    //     let idx = self.to_voxel_index(coords);
    //     self.voxels
    //         .get(idx)
    //         .and_then(|vox_opt| vox_opt.as_ref())
    // }

    // /// Gets mutable voxel from this chunk.
    // /// Returns None if out of bounds, or voxel was not present at the coordinates specified.
    // pub fn get_voxel_mut(&mut self, coords: UVec3) -> Option<&mut Voxel> {
    //     let idx = self.to_voxel_index(coords);
    //     self.voxels
    //         .get_mut(idx)
    //         .and_then(|vox_opt| vox_opt.as_mut())
    // }

    // /// Converts coordinates to voxel index
    // fn to_voxel_index(&self, coords: UVec3) -> usize {
    //     let x = coords.x;
    //     let y = coords.y;
    //     let z = coords.z;
    //     let w = self.size.x;
    //     let h = self.size.y;
    //     let index = x + y*w + z*(w + h);
    //     index as usize
    // }
}

//////////////////////////////////////////////// Systems ////////////////////////////////////////////////

/// Synchronizes previous position with current position.
fn sync_positions(mut entities: Query<(&mut Position, &mut PreviousPosition)>) {
    for (pos, mut prev_pos) in &mut entities {
        prev_pos.0 = pos.0;
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
fn apply_velocity(mut entities: Query<(&Velocity, &mut Position)>) {
    for (vel, mut pos) in &mut entities {
        pos.0 += vel.0;
    }
}

/// Applies voxel collision code
fn apply_voxel_collisions() {
    // TODO
}

/// Linearly interpolates transforms between [`PreviousPosition`] and [`Position`] components.
fn lerp_transform(
    timesteps: Res<FixedTimesteps>,
    mut entities: Query<(&mut Transform, &PreviousPosition, &Position)>
) {
    let t = timesteps
        .get(PHYSICS_TIMESTEP)
        .unwrap()
        .overstep_percentage() as f32;
    for (mut transform, prev_pos, pos) in &mut entities {
        transform.translation = prev_pos.0.lerp(pos.0, t);
    }
}
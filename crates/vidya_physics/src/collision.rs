use bevy_math::Vec3;

use crate::{PhysicsShape, VoxelChunk};


/// Represents information about a collision.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Collision {
    /// T-value between 0.0, and 1.0.
    /// Determines "when" a collision occurred in a single linear motion.
    pub t: f32,
    /// Normal of the surface of the object hit on the RHS.
    pub normal: Vec3,
    /// Side hit of the surface of the object hit on the RHS.
    pub side_hit: Option<SideHit>
}

/// When hitting terrain, determines what "kind" of collision it was.
/// Useful for platforming logic.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum SideHit {
    Floor,
    Wall,
    Ceiling
}

/// Contains info about a moving shape through space
pub struct ShapeData {
    pub shape: PhysicsShape,
    pub src: Vec3,
    pub dir: Vec3,
    pub half_extents: Vec3
}
impl ShapeData {
    pub fn aabb(&self) -> AABB {
        AABB::from_motion(self.src, self.dir, self.half_extents)
    }
}

/// Contains info about a moving voxel chunk through space.
pub struct VoxelChunkData<'a> {
    pub chunk: &'a VoxelChunk,
    pub src: Vec3,
    pub dir: Vec3,
    pub half_extents: Vec3
}
impl<'a> VoxelChunkData<'a> {
    pub fn aabb(&self) -> AABB {
        AABB::from_motion(self.src, self.dir, self.half_extents)
    }
}

pub struct AABB {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
    pub near: f32,
    pub far: f32
}
impl AABB {
    pub fn from_motion(src: Vec3, dir: Vec3, half_extents: Vec3) -> Self {
        let dest = src + dir;
        let half = half_extents;
        let (left, right) =
            if dir.x < 0.0 {
                (dest.x - half.x, src.x + half.x)
            }
            else {
                (src.x - half.x, dest.x + half.x)
            };
        let (bottom, top) =
            if dir.y < 0.0 {
                (dest.y - half.y, src.y + half.y)
            }
            else {
                (src.y - half.y, dest.y + half.y)
            };
        let (far, near) =
            if dir.z < 0.0 {
                (dest.z - half.z, src.z + half.z)
            }
            else {
                (src.z - half.z, dest.z + half.z)
            };
        AABB { left, right, bottom, top, near, far }
    }
    pub fn intersects(&self, other: &AABB) -> bool {
        self.left <= other.right &&
        self.right >= other.left &&
        self.bottom <= other.top &&
        self.top >= other.bottom &&
        self.far <= other.near &&
        self.near >= other.far
    }
}

pub(crate) fn collide_shape_with_chunk(mut shape: ShapeData, mut chunk: VoxelChunkData<'_>) -> Option<Collision> {
    
    // Broad AABB check
    let shape_aabb = shape.aabb();
    let chunk_aabb = chunk.aabb();
    if !shape_aabb.intersects(&chunk_aabb) {
        return None;
    }


    None
}
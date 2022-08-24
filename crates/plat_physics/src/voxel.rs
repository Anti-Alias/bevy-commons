use bevy::prelude::*;
use super::*;

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
    pub fn rotate(self, vec: Vec2) -> Vec2 {
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

    pub fn rotate_x(self, vec: Vec3) -> Vec3 {
        let rotated = self.rotate(vec.yz());
        Vec3::new(vec.x, rotated.x, rotated.y)
    }

    pub fn rotate_y(self, vec: Vec3) -> Vec3 {
        let rotated = self.rotate(vec.xz());
        Vec3::new(rotated.x, vec.y, rotated.y)
    }

    pub fn rotate_z(self, vec: Vec3) -> Vec3 {
        let rotated = self.rotate(vec.xy());
        Vec3::new(rotated.x, rotated.y, vec.z)
    }

    fn to_num(self) -> usize {
        match self {
            Degree::Zero => 0,
            Degree::Ninty => 1,
            Degree::OneEighty => 2,
            Degree::TwoSeventy => 3
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

pub(crate) fn cuboid_collision(_voxel_bounds: &Bounds, _movement: &Movement) {

}


#[cfg(test)]
mod degree_tests {

    use bevy::prelude::*;
    use crate::Degree;

    #[test]
    fn rotate() {
        let degree = Degree::Zero;
        assert_eq!(Vec2::new(1.0, 0.0), degree.rotate(Vec2::new(1.0, 0.0)));
        assert_eq!(Vec2::new(0.0, 2.0), degree.rotate(Vec2::new(0.0, 2.0)));

        let degree = Degree::Ninty;
        assert_eq!(Vec2::new(0.0, 1.0), degree.rotate(Vec2::new(1.0, 0.0)));
        assert_eq!(Vec2::new(-2.0, 0.0), degree.rotate(Vec2::new(0.0, 2.0)));

        let degree = Degree::OneEighty;
        assert_eq!(Vec2::new(-3.0, 0.0), degree.rotate(Vec2::new(3.0, 0.0)));
        assert_eq!(Vec2::new(0.0, 4.0), degree.rotate(Vec2::new(0.0, -4.0)));

        let degree = Degree::TwoSeventy;
        assert_eq!(Vec2::new(0.0, -5.0), degree.rotate(Vec2::new(5.0, 0.0)));
        assert_eq!(Vec2::new(-6.0, 0.0), degree.rotate(Vec2::new(0.0, -6.0)));
    }

    #[test]
    fn add_sub() {
        assert_eq!(Degree::Ninty, Degree::Ninty + Degree::Zero);
        assert_eq!(Degree::Ninty, Degree::Ninty - Degree::Zero);
        assert_eq!(Degree::OneEighty, Degree::Ninty + Degree::Ninty);
        assert_eq!(Degree::TwoSeventy, Degree::OneEighty + Degree::Ninty);
        assert_eq!(Degree::Ninty, Degree::TwoSeventy + Degree::OneEighty);
        assert_eq!(Degree::Ninty, Degree::TwoSeventy - Degree::OneEighty);
    }
}
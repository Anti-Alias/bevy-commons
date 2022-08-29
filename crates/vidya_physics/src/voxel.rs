use std::ops::Mul;

use bevy_ecs::prelude::*;
use bevy_math::{prelude::*, Vec3Swizzles};

use super::*;

//////////////////////////////////////////////// Voxel-related ////////////////////////////////////////////////

/// A collider stored in a [`VoxelChunk`].
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub enum Voxel {
    /// No voxel
    #[default]
    Empty,
    /// Cuboid shaped voxel
    Cuboid,
    /// Slope shaped voxel. Default orientation has the slope's normal facing (0, 1, 1).
    Slope
}

/// Stores both a [`Voxel`] and its orientation.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct VoxelData {
    pub voxel: Voxel,
    pub orientation: Orientation
}
impl VoxelData {
    pub fn new(voxel: Voxel) -> Self {
        Self {
            voxel,
            orientation: Orientation::default()
        }
    }

    pub fn with_orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }
}

/// Represents a chunk of [`Voxel`]s stored in an [`Entity`].
#[derive(Component, Debug)]
pub struct VoxelChunk {
    size: UVec3,
    voxels: Vec<VoxelData>
}
impl VoxelChunk {

    /// Allocates an empty voxel chunk.
    pub fn new(size: UVec3) -> Self {
        Self {
            size,
            voxels: vec![VoxelData::default(); (size.x * size.y * size.z) as usize]
        }
    }

    /// Size of the chunk measured in voxels
    pub fn size(&self) -> UVec3 {
        self.size
    }

    /// Gets voxel from this chunk.
    /// Returns None if out of bounds.
    pub fn get_voxel(&self, coords: UVec3) -> Option<&VoxelData> {
        if !self.in_bounds(coords) {
            return None;
        }
        let idx = self.to_voxel_index(coords);
        self.voxels.get(idx)
    }

    /// Gets mutable voxel from this chunk.
    /// Returns None if out of bounds.
    pub fn get_voxel_mut(&mut self, coords: UVec3) -> Option<&mut VoxelData> {
        if !self.in_bounds(coords) {
            return None;
        }
        let idx = self.to_voxel_index(coords);
        self.voxels.get_mut(idx)
    }

    /// Sets the value of a voxel and returns self.
    /// Helpful when setting multiple voxels at once.
    pub fn set_voxel(&mut self, coords: UVec3, voxel_data: VoxelData) -> &mut Self {
        let idx = self.to_voxel_index(coords);
        if !self.in_bounds(coords) {
            panic!("Coordiantes out of bounds");
        }
        let current_voxel = self.voxels.get_mut(idx).expect("Voxel coordinates out of bounds");
        *current_voxel = voxel_data;
        self
    }

    /// Sets the value of a voxel and returns self.
    /// Helpful when setting multiple voxels at once.
    pub fn set_voxel_plane(
        &mut self,
        xyz: u32,
        src: UVec2,
        dest: UVec2,
        axis: PlaneAxis,
        voxel_data: VoxelData
    ) -> &mut Self {
        match axis {
            PlaneAxis::XY => {
                for x in src.x..dest.x {
                    for y in src.y..dest.y {
                        let z = xyz;
                        self.set_voxel(UVec3::new(x, y, z), voxel_data);
                    }
                }
            },
            PlaneAxis::YZ => {
                for y in src.x..dest.x {
                    for z in src.y..dest.y {
                        let x = xyz;
                        self.set_voxel(UVec3::new(x, y, z), voxel_data);
                    }
                }
            },
            PlaneAxis::XZ => {
                for x in src.x..dest.x {
                    for z in src.y..dest.y {
                        let y = xyz;
                        self.set_voxel(UVec3::new(x, y, z), voxel_data);
                    }
                }
            }
        }
        self
    }

    fn in_bounds(&self, coords: UVec3) -> bool {
        coords.x < self.size.x && coords.y < self.size.y && coords.z < self.size.z
    }

    /// Converts coordinates to voxel index
    fn to_voxel_index(&self, coords: UVec3) -> usize {
        let x = coords.x;
        let y = coords.y;
        let z = coords.z;
        let w = self.size.x;
        let h = self.size.y;
        //let index = x + y*w + z*w*h;
        let index = x + w*(y + z*h);
        index as usize
    }

    // Produces iterator over voxels in chunk
    pub fn iter(&self) -> VoxelChunkIterator<'_> {
        VoxelChunkIterator {
            chunk: self,
            position: UVec3::ZERO,
            index: 0
        }
    }
}

/// Axis an axis-aligned plane can sit on
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum PlaneAxis { XY, YZ, XZ }

pub struct VoxelChunkIterator<'a> {
    chunk: &'a VoxelChunk,
    position: UVec3,
    index: usize
}
impl<'a> Iterator for VoxelChunkIterator<'a> {
    type Item = (&'a VoxelData, UVec3);
    fn next(&mut self) -> Option<Self::Item> {

        // Quits if at end
        if self.index == self.chunk.voxels.len() {
            return None;
        }

        // Gets voxel and updates position and index
        let voxel = self.chunk.get_voxel(self.position).unwrap();
        let size = self.chunk.size;
        let pos = self.position;
        self.position.x += 1;
        if self.position.x == size.x {
            self.position.x = 0;
            self.position.y += 1;
            if self.position.y == size.y {
                self.position.y = 0;
                self.position.z += 1;
            }
        }
        self.index += 1;

        // Done
        Some((voxel, pos))
    }
}

/// Bundle of all the components needed for voxel chunk [`Entity`] to partake in a physics simulation
#[derive(Bundle)]
pub struct VoxelChunkBundle {
    pub voxel_chunk: VoxelChunk,
    pub current_transform: CurrentTransform,
    pub previous_transform: PreviousTransform,
    pub bounds: Bounds,
    pub velocity: Velocity,
    pub physics_marker: PhysicsMarker
}
impl VoxelChunkBundle {
    pub fn new(voxel_chunk: VoxelChunk, transform: Transform, bounds: Bounds) -> Self {
        Self {
            voxel_chunk,
            current_transform: CurrentTransform(transform),
            previous_transform: PreviousTransform(transform),
            bounds,
            velocity: Velocity::default(),
            physics_marker: PhysicsMarker
        }
    }
}


//////////////////////////////////////////////// Helper structs ////////////////////////////////////////////////

/// Similar to a euler rotation in the order of XYZ, except constrained to 90 degree angles
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Orientation {
    /// Rotation along x axis
    pub x_rot: Degree,
    /// Rotation along y axis
    pub y_rot: Degree,
    /// Rotation along z axis
    pub z_rot: Degree
}
impl Orientation {
    pub const ZERO: Orientation = Orientation {
        x_rot: Degree::Zero,
        y_rot: Degree::Zero,
        z_rot: Degree::Zero,
    };
    pub fn new(x_rot: Degree, y_rot: Degree, z_rot: Degree) -> Self {
        Self { x_rot, y_rot, z_rot }
    }
    pub const fn with_x_rot(mut self, x_rot: Degree) -> Self {
        self.x_rot = x_rot;
        self
    }
    pub const fn with_y_rot(mut self, y_rot: Degree) -> Self {
        self.y_rot = y_rot;
        self
    }
    pub const fn with_z_rot(mut self, z_rot: Degree) -> Self {
        self.z_rot = z_rot;
        self
    }
}
impl Add for Orientation {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Orientation {
            x_rot: self.x_rot + rhs.x_rot,
            y_rot: self.y_rot + rhs.y_rot,
            z_rot: self.z_rot + rhs.z_rot,
        }
    }
}
impl Sub for Orientation {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Orientation {
            x_rot: self.x_rot - rhs.x_rot,
            y_rot: self.y_rot - rhs.y_rot,
            z_rot: self.z_rot - rhs.z_rot,
        }
    }
}
impl Mul<Vec3> for Orientation {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        let mut result = rhs;
        result = self.z_rot.rotate_z(result);
        result = self.y_rot.rotate_y(result);
        result = self.x_rot.rotate_x(result);
        result
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

    /// Rotates a vec around the x axis
    pub fn rotate_x(self, vec: Vec3) -> Vec3 {
        let rotated = self.rotate(vec.yz());
        Vec3::new(vec.x, rotated.x, rotated.y)
    }

    /// Rotates a vec around the y axis
    pub fn rotate_y(self, vec: Vec3) -> Vec3 {
        let rotated = (-self).rotate(vec.xz());
        Vec3::new(rotated.x, vec.y, rotated.y)
    }

    /// Rotates a vec around the z axis
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
            _ => panic!("This error is impossible")
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
            Self::Zero => Self::Zero,
            Self::Ninty => Self::TwoSeventy,
            Self::OneEighty => Self::OneEighty,
            Self::TwoSeventy => Self::Ninty
        }
    }
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

#[cfg(test)]
mod orientation_tests {

    use bevy::prelude::*;

    use crate::{Orientation, Degree};

    #[test]
    fn rotate() {
        let orientation = Orientation::new(Degree::Zero, Degree::Ninty, Degree::Ninty);
        assert_eq!(
            Vec3::new(0.0, 1.0, 0.0),
            orientation * Vec3::new(1.0, 0.0, 0.0)
        );

        let orientation = Orientation::new(Degree::Zero, Degree::Zero, Degree::Ninty);
        assert_eq!(
            Vec3::new(-1.0, 0.0, 0.0),
            orientation * Vec3::new(0.0, 1.0, 0.0)
        );
    }
}

#[cfg(test)]
mod voxel_chunk_tests {

    use bevy_math::UVec3;

    use crate::{ Voxel, VoxelChunk, VoxelData };

    #[test]
    fn build() {
        // Builds chunk
        let mut chunk = VoxelChunk::new(UVec3::new(16, 16, 16));
        chunk
            .set_voxel(UVec3::new(0, 0, 0), VoxelData::new(Voxel::Cuboid))
            .set_voxel(UVec3::new(1, 0, 0), VoxelData::new(Voxel::Slope))
            .set_voxel(UVec3::new(0, 1, 0), VoxelData::new(Voxel::Cuboid))
            .set_voxel(UVec3::new(0, 0, 1), VoxelData::new(Voxel::Slope))
            .set_voxel(UVec3::new(5, 6, 7), VoxelData::new(Voxel::Cuboid))
            .set_voxel(UVec3::new(15, 15, 15), VoxelData::new(Voxel::Slope));

        // Sets voxel using manual api
        let voxel = chunk.get_voxel_mut(UVec3::new(8, 8, 8)).unwrap();
        *voxel = VoxelData::new(Voxel::Cuboid);

        // Validates that chunk values are the same
        assert_eq!(Some(&VoxelData::new(Voxel::Cuboid)), chunk.get_voxel(UVec3::new(0, 0, 0)));
        assert_eq!(Some(&VoxelData::new(Voxel::Slope)), chunk.get_voxel(UVec3::new(1, 0, 0)));
        assert_eq!(Some(&VoxelData::new(Voxel::Cuboid)), chunk.get_voxel(UVec3::new(0, 1, 0)));
        assert_eq!(Some(&VoxelData::new(Voxel::Slope)), chunk.get_voxel(UVec3::new(0, 0, 1)));
        assert_eq!(Some(&VoxelData::new(Voxel::Cuboid)), chunk.get_voxel(UVec3::new(5, 6, 7)));
        assert_eq!(Some(&VoxelData::new(Voxel::Slope)), chunk.get_voxel(UVec3::new(15, 15, 15)));
        assert_eq!(Some(&VoxelData::new(Voxel::Cuboid)), chunk.get_voxel(UVec3::new(8, 8, 8)));

        // Checks out-of-bounds returns None
        assert_eq!(None, chunk.get_voxel(UVec3::new(16, 0, 0)));
        assert_eq!(None, chunk.get_voxel(UVec3::new(0, 16, 0)));
        assert_eq!(None, chunk.get_voxel(UVec3::new(0, 0, 16)));
        assert_eq!(None, chunk.get_voxel(UVec3::new(1337, 1337, 1337)));
    }
}
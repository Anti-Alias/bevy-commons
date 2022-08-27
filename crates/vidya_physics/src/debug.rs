use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_asset::prelude::*;
use bevy_math::Vec3;
use bevy_render::mesh::Indices;
use bevy_render::prelude::*;
use bevy_pbr::prelude::*;
use bevy_render::render_resource::PrimitiveTopology;
use vidya_interp::InterpolationSystems;

use crate::{VoxelChunk, Bounds, VoxelData, Voxel, Orientation};

/// Plugin that adds debug graphics objects in the physics engine.
pub struct PhysicsDebugPlugin;
impl Plugin for  PhysicsDebugPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app
            .add_startup_system(create_materials)
            .add_system_to_stage(CoreStage::PostUpdate,
                add_mesh_to_voxel_chunks
                    .label(PhysicsDebugSystems::AddMeshToVoxelChunks)
                    .before(InterpolationSystems::Interpolate)
            );
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, SystemLabel)]
pub enum PhysicsDebugSystems {
    AddMeshToVoxelChunks
}


/// Resource that stores the materials used by the debug plugin.
pub struct DebugMaterials {
    chunk_material: Handle<StandardMaterial>
}

fn create_materials(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.insert_resource(DebugMaterials {
        chunk_material: materials.add(Color::GREEN.into())
    });
}

/// Scans for voxel chunk entities without a mesh + material, and if found, generates those and inserts them
fn add_mesh_to_voxel_chunks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    debug_materials: Res<DebugMaterials>,
    mut meshless_chunks: Query<
        (Entity, &VoxelChunk, &Bounds),
        (Without<Handle<Mesh>>, Without<Handle<StandardMaterial>>)
    >
) {
    for (entity, chunk, bounds) in &mut meshless_chunks {
        commands.entity(entity).insert_bundle(PbrBundle {
            mesh: meshes.add(create_mesh_from_chunk(
                chunk,
                bounds.size()
            )),
            material: debug_materials.chunk_material.clone(),
            ..Default::default()
        });
    }
}

fn create_mesh_from_chunk(chunk: &VoxelChunk, size: Vec3) -> Mesh {
    
    // Creates vertex data
    let voxel_size = size / chunk.size().as_vec3();
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let half_size = size / 2.0;
    for (voxel_data, coords) in chunk.iter() {
        let VoxelData { voxel, orientation } = *voxel_data;
        let voxel_pos = coords.as_vec3() * voxel_size - half_size;
        match voxel {
            Voxel::Cuboid => write_cuboid(
                &mut vertices,
                &mut indices,
                voxel_pos,
                voxel_size
            ),
            Voxel::Slope => write_slope(
                &mut vertices,
                &mut indices,
                voxel_pos,
                voxel_size,
                orientation
            ),
            Voxel::Empty => {}
        }
    }

    // Adds vertex data to mesh
    let positions: Vec<[f32; 3]> = vertices.iter().map(|vertex| vertex.pos).collect();
    let normals: Vec<[f32; 3]> = vertices.iter().map(|vertex| vertex.norm).collect();
    let indices = Indices::U32(indices);
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_indices(Some(indices));
    mesh
}

fn write_cuboid(
    vertices: &mut Vec<Vertex>,
    indices: &mut Vec<u32>,
    pos: Vec3,
    size: Vec3
) {
    // Normal constants
    const N_LEFT: [f32; 3] = [-1.0, 0.0, 0.0];
    const N_RIGHT: [f32; 3] = [1.0, 0.0, 0.0];
    const N_BOTTOM: [f32; 3] = [0.0, -1.0, 0.0];
    const N_TOP: [f32; 3] = [0.0, 1.0, 0.0];
    const N_NEAR: [f32; 3] = [0.0, 0.0, 1.0];
    const N_FAR: [f32; 3] = [0.0, 0.0, -1.0];
    
    // Writes vertices
    let start = vertices.len();
    vertices.extend_from_slice(&[
        Vertex::new([0.0, 0.0, 0.0], N_LEFT),
        Vertex::new([0.0, 0.0, 1.0], N_LEFT),
        Vertex::new([0.0, 1.0, 1.0], N_LEFT),
        Vertex::new([0.0, 1.0, 0.0], N_LEFT),

        Vertex::new([1.0, 1.0, 0.0], N_RIGHT),
        Vertex::new([1.0, 1.0, 1.0], N_RIGHT),
        Vertex::new([1.0, 0.0, 1.0], N_RIGHT),
        Vertex::new([1.0, 0.0, 0.0], N_RIGHT),

        Vertex::new([1.0, 0.0, 0.0], N_BOTTOM),
        Vertex::new([1.0, 0.0, 1.0], N_BOTTOM),
        Vertex::new([0.0, 0.0, 1.0], N_BOTTOM),
        Vertex::new([0.0, 0.0, 0.0], N_BOTTOM),

        Vertex::new([0.0, 1.0, 0.0], N_TOP),
        Vertex::new([0.0, 1.0, 1.0], N_TOP),
        Vertex::new([1.0, 1.0, 1.0], N_TOP),
        Vertex::new([1.0, 1.0, 0.0], N_TOP),

        Vertex::new([0.0, 0.0, 1.0], N_NEAR),
        Vertex::new([1.0, 0.0, 1.0], N_NEAR),
        Vertex::new([1.0, 1.0, 1.0], N_NEAR),
        Vertex::new([0.0, 1.0, 1.0], N_NEAR),

        Vertex::new([0.0, 1.0, 0.0], N_FAR),
        Vertex::new([1.0, 1.0, 0.0], N_FAR),
        Vertex::new([1.0, 0.0, 0.0], N_FAR),
        Vertex::new([0.0, 0.0, 0.0], N_FAR)
    ]);

    // Offsets/scales vertices
    for v in &mut vertices[start..start+24] {
        v.pos[0] *= size.x;
        v.pos[0] += pos.x;
        v.pos[1] *= size.y;
        v.pos[1] += pos.y;
        v.pos[2] *= size.z;
        v.pos[2] += pos.z;
    }

    // Writes indices
    let s = start as u32;
    indices.extend_from_slice(&[
        s+0, s+1, s+2, s+2, s+3, s+0,       // LEFT
        s+4, s+5, s+6, s+6, s+7, s+4,       // RIGHT
        s+8, s+9, s+10, s+10, s+11, s+8,    // BOTTOM
        s+12, s+13, s+14, s+14, s+15, s+12, // TOP
        s+16, s+17, s+18, s+18, s+19, s+16, // NEAR
        s+20, s+21, s+22, s+22, s+23, s+20  // FAR
    ])
}

fn write_slope(
    _vertices: &mut Vec<Vertex>,
    _indices: &mut Vec<u32>,
    _pos: Vec3,
    _size: Vec3,
    _orientation: Orientation
) {
    // TODO
}

#[derive(Clone)]
struct Vertex {
    pos: [f32; 3],
    norm: [f32; 3]
}
impl Vertex {
    pub fn new(pos: [f32; 3], norm: [f32; 3]) -> Self {
        Self { pos, norm }
    }
}
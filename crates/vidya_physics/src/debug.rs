use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_asset::prelude::*;
use bevy_render::prelude::*;
use bevy_pbr::prelude::*;
use vidya_interp::InterpolationSystems;

use crate::VoxelChunk;

/// Plugin that adds debug graphics objects in the physics engine.
pub struct PhysicsDebugPlugin;
impl Plugin for  PhysicsDebugPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.add_system_to_stage(CoreStage::PostUpdate,
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

/// Scans for voxel chunk entities without a mesh + material, and if found, generates those and inserts them
fn add_mesh_to_voxel_chunks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshless_chunks: Query<
        &VoxelChunk,
        (Without<Handle<Mesh>>, Without<Handle<StandardMaterial>>)
    >
) {
    for chunk in &mut meshless_chunks {

    }
}

fn create_mesh_from_chunk() {

}
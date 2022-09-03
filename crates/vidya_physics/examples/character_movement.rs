//#[cfg(feature = "debug")]
mod example {
    use vidya_fixed_timestep::FixedTimestepPlugin;
    use vidya_physics::*;
    use vidya_physics::debug::*;
    use bevy::prelude::*;


    /// Example where three chunks are spawned side by side.
    /// Chunks are not rotated via a [`Transform`], but rather are constructed differently.
    pub fn start() {
        App::new()
            .add_plugins(DefaultPlugins)
            .add_plugin(FixedTimestepPlugin::default())
            .add_plugin(PhysicsPlugin)
            .add_plugin(PhysicsDebugPlugin)         // Added to enable debug rendering, in this case, for the chunks
            .add_startup_system(startup)
            .run();
    }

    /// Spawns light, chunks and camera
    fn startup(mut commands: Commands) {

        // Spawns light above scene
        commands.spawn_bundle(PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(7.0, 7.0, 7.0),
            ..default()
        });

        // Spawns 3 voxel chunks, all double pyramids of a different orientation
        let chunk = generate_chunk();
        commands.spawn_bundle(VoxelChunkBundle::new(
            chunk,                                      // Raw chunk data
            Transform::from_xyz(0.0, 0.0, 0.0),         // Center of the chunk in units
            Bounds::new(Vec3::new(8.0, 2.0, 8.0))       // Size of the chunk in units
        )).insert(DebugRender);                         // Allows debug info of chunk to be rendered

        // Spawns camera
        commands.spawn()
            .insert_bundle(Camera3dBundle {
                transform: Transform::from_xyz(0.0, 2.0, 7.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
                ..default()
            });
    }

    fn generate_chunk() -> VoxelChunk {
        let mut chunk = VoxelChunk::new(UVec3::new(16, 4, 16));
        chunk
            // Bottom plane
            .set_voxel_plane(
                0,
                UVec2::new(0, 0),
                UVec2::new(16, 16),
                PlaneAxis::XZ,
                VoxelData::new(Voxel::Cuboid)
            )
            // Left rim
            .set_voxel_plane(
                1,
                UVec2::new(0, 0),
                UVec2::new(1, 16),
                PlaneAxis::XZ,
                VoxelData::new(Voxel::Cuboid)
            )
            // Right rim
            .set_voxel_plane(
                1,
                UVec2::new(15, 0),
                UVec2::new(16, 16),
                PlaneAxis::XZ,
                VoxelData::new(Voxel::Cuboid)
            )
            // Near rim
            .set_voxel_plane(
                1,
                UVec2::new(1, 15),
                UVec2::new(15, 16),
                PlaneAxis::XZ,
                VoxelData::new(Voxel::Cuboid)
            )
            // Far rim
            .set_voxel_plane(
                1,
                UVec2::new(1, 0),
                UVec2::new(15, 1),
                PlaneAxis::XZ,
                VoxelData::new(Voxel::Cuboid)
            );
        chunk
    }
}

#[cfg(feature = "debug")]
fn main() {
    example::start();
}

#[cfg(not(feature = "debug"))]
fn main() {
    panic!("Feature 'debug' not set");
}

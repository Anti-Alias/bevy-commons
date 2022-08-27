use vidya_physics::*;
use bevy::prelude::*;


/// Example where only a single falling entity is spawned.
pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(PhysicsDebugPlugin)
        .add_startup_system(startup)
        .run();
}

fn startup(mut commands: Commands) {

    // Adds gravity
    // commands.insert_resource(Gravity(Vec3::new(0.0, -0.01, 0.0)));

    // Spawns light above scene
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 10.0, 0.0),
        ..default()
    });

    // Spawns voxel chunk
    let mut chunk = VoxelChunk::new(UVec3::new(8, 8, 8));
    chunk.set_voxel(UVec3::new(0, 0, 0), VoxelData::new(Voxel::Cuboid));
    chunk.set_voxel(UVec3::new(7, 0, 0), VoxelData::new(Voxel::Cuboid));
    chunk.set_voxel(UVec3::new(7, 0, 7), VoxelData::new(Voxel::Cuboid));
    chunk.set_voxel(UVec3::new(0, 0, 7), VoxelData::new(Voxel::Cuboid));
    chunk.set_voxel(UVec3::new(0, 7, 0), VoxelData::new(Voxel::Cuboid));
    chunk.set_voxel(UVec3::new(7, 7, 0), VoxelData::new(Voxel::Cuboid));
    chunk.set_voxel(UVec3::new(7, 7, 7), VoxelData::new(Voxel::Cuboid));
    chunk.set_voxel(UVec3::new(0, 7, 7), VoxelData::new(Voxel::Cuboid));
    commands.spawn_bundle(VoxelChunkBundle::new(
        chunk,
        Transform::from_xyz(0.0, 0.0, 0.0),
        Bounds::new(Vec3::new(4.0, 4.0, 4.0))
    ));

    // Spawns camera
    commands.spawn()
        .insert_bundle(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 7.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..default()
        });
}
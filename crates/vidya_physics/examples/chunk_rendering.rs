use vidya_physics::*;
use bevy::prelude::*;


/// Example where only a single falling entity is spawned.
pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())   // Only added to ensure that Transform gets synced with CurrentTransform and PreviousTransform in PhysicsBundle
        .add_plugin(PhysicsDebugPlugin)         // Added to enable debug rendering, in this case, for the chunks
        .add_startup_system(startup)
        .add_system(spin_camera)
        .run();
}

/// Marker for this example's camera
#[derive(Component)]
struct MyCamera;


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
    let chunk_xy = generate_chunk(PlaneAxis::XY);
    let chunk_yz = generate_chunk(PlaneAxis::YZ);
    let chunk_xz = generate_chunk(PlaneAxis::XZ);
    commands.spawn_bundle(VoxelChunkBundle::new(
        chunk_xy,                                   // Raw chunk data
        Transform::from_xyz(0.0, 0.0, 0.0),         // Center of the chunk in units
        Bounds::new(Vec3::new(2.0, 2.0, 2.0))       // Size of the chunk in units
    )).insert(DebugRender);                         // Allows debug info of chunk to be rendered
    commands.spawn_bundle(VoxelChunkBundle::new(
        chunk_yz,
        Transform::from_xyz(-3.0, 0.0, 0.0),
        Bounds::new(Vec3::new(2.0, 2.0, 2.0))
    )).insert(DebugRender);
    commands.spawn_bundle(VoxelChunkBundle::new(
        chunk_xz,
        Transform::from_xyz(3.0, 0.0, 0.0),
        Bounds::new(Vec3::new(2.0, 2.0, 2.0))
    )).insert(DebugRender);

    // Spawns camera
    commands.spawn()
        .insert_bundle(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 7.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..default()
        })
        .insert(MyCamera);
}

/// Spins camera around the center of the screen while looking at it
fn spin_camera(
    mut camera: Query<&mut Transform, With<MyCamera>>,
    time: Res<Time>
) {
    const RADIUS: f32 = 10.0;
    let theta = time.time_since_startup().as_secs_f32();
    let mut cam_trans = camera.get_single_mut().unwrap();
    cam_trans.translation = Vec3::new(
        theta.cos() * RADIUS,
        -theta.sin() * RADIUS / 2.0,
        theta.sin() * RADIUS
    );
    cam_trans.look_at(Vec3::ZERO, Vec3::Y);  
}

/// Util function that generates a double-pyramid chunk
fn generate_chunk(axis: PlaneAxis) -> VoxelChunk {
    let mut chunk = VoxelChunk::new(UVec3::new(8, 8, 8));
    chunk.set_voxel_plane(0, UVec2::new(0, 0), UVec2::new(8, 8), axis, VoxelData::new(Voxel::Cuboid));
    chunk.set_voxel_plane(1, UVec2::new(1, 1), UVec2::new(7, 7), axis, VoxelData::new(Voxel::Cuboid));
    chunk.set_voxel_plane(2, UVec2::new(2, 2), UVec2::new(6, 6), axis, VoxelData::new(Voxel::Cuboid));
    chunk.set_voxel_plane(3, UVec2::new(3, 3), UVec2::new(5, 5), axis, VoxelData::new(Voxel::Cuboid));
    chunk.set_voxel_plane(4, UVec2::new(3, 3), UVec2::new(5, 5), axis, VoxelData::new(Voxel::Cuboid));
    chunk.set_voxel_plane(5, UVec2::new(2, 2), UVec2::new(6, 6), axis, VoxelData::new(Voxel::Cuboid));
    chunk.set_voxel_plane(6, UVec2::new(1, 1), UVec2::new(7, 7), axis, VoxelData::new(Voxel::Cuboid));
    chunk.set_voxel_plane(7, UVec2::new(0, 0), UVec2::new(8, 8), axis, VoxelData::new(Voxel::Cuboid));
    chunk
}
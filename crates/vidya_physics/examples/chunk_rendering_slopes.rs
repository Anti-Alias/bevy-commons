use vidya_fixed_timestep::FixedTimestepPlugin;
use vidya_physics::*;
use vidya_physics::debug::*;
use bevy::prelude::*;

/// Example where a single chunk with slope voxels are spawned
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FixedTimestepPlugin::default())
        .add_plugin(PhysicsPlugin)
        .add_plugin(PhysicsDebugPlugin)         // Added to enable debug rendering, in this case, for the chunk
        .add_startup_system(startup)
        .add_system(spin_camera)
        .run();
}

/// Marker for this example's camera
#[derive(Component)]
struct MyCamera;

/// Spawns light, chunk and camera
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

    // Spawns voxel chunk
    let slope_chunk = generate_slope_chunk();
    commands.spawn_bundle(PhysicsBundle {
        shape: Shape::VoxelChunk(slope_chunk),                                      // Raw chunk data
        current_transform: CurrentTransform(Transform::from_xyz(0.0, 0.0, 0.0)),    // Center of the chunk in units
        bounds: HalfExtents::new(5.0, 4.0, 1.0),                                    // Size of the chunk in units
        ..default()
    }).insert(DebugRender::default());                                                         // Allows debug info of chunk to be rendered

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

/// Util function that generates a slope chunk
fn generate_slope_chunk() -> VoxelChunk {
    let mut chunk = VoxelChunk::new(UVec3::new(5, 4, 1));

    let yaw_90 = Orientation { y_rot: Degree::Ninty, ..Default::default() };
    let yaw_180 = Orientation { y_rot: Degree::OneEighty, ..Default::default() };
    let yaw_270 = Orientation { y_rot: Degree::TwoSeventy, ..Default::default() };

    let roll_90 = Orientation { x_rot: Degree::Ninty, ..Default::default() };
    let roll_180 = Orientation { x_rot: Degree::OneEighty, ..Default::default() };
    let roll_270 = Orientation { x_rot: Degree::TwoSeventy, ..Default::default() };

    let pitch_90 = Orientation { z_rot: Degree::Ninty, ..Default::default() };
    let pitch_180 = Orientation { z_rot: Degree::OneEighty, ..Default::default() };
    let pitch_270 = Orientation { z_rot: Degree::TwoSeventy, ..Default::default() };
    
    // Makes columns left -> right
    chunk
        // First column is yaw test
        .set_voxel(UVec3::new(0, 0, 0), VoxelData::new(Voxel::Slope))
        .set_voxel(UVec3::new(0, 1, 0), VoxelData::new(Voxel::Slope).with_orientation(yaw_90))
        .set_voxel(UVec3::new(0, 2, 0), VoxelData::new(Voxel::Slope).with_orientation(yaw_180))
        .set_voxel(UVec3::new(0, 3, 0), VoxelData::new(Voxel::Slope).with_orientation(yaw_270))

        // First column is roll test
        .set_voxel(UVec3::new(2, 0, 0), VoxelData::new(Voxel::Slope))
        .set_voxel(UVec3::new(2, 1, 0), VoxelData::new(Voxel::Slope).with_orientation(roll_90))
        .set_voxel(UVec3::new(2, 2, 0), VoxelData::new(Voxel::Slope).with_orientation(roll_180))
        .set_voxel(UVec3::new(2, 3, 0), VoxelData::new(Voxel::Slope).with_orientation(roll_270))

        // Third column is pitch test
        .set_voxel(UVec3::new(4, 0, 0), VoxelData::new(Voxel::Slope))
        .set_voxel(UVec3::new(4, 1, 0), VoxelData::new(Voxel::Slope).with_orientation(pitch_90))
        .set_voxel(UVec3::new(4, 2, 0), VoxelData::new(Voxel::Slope).with_orientation(pitch_180))
        .set_voxel(UVec3::new(4, 3, 0), VoxelData::new(Voxel::Slope).with_orientation(pitch_270));
    chunk
}
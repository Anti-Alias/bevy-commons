use vidya_camera_target::prelude::*;
use vidya_fixed_timestep::FixedTimestepPlugin;
use vidya_physics::*;
use vidya_physics::debug::*;
use bevy::prelude::*;

/// Example where three chunks are spawned side by side.
/// Chunks are not rotated via a [`Transform`], but rather are constructed differently.
pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FixedTimestepPlugin::default())
        .add_plugin(PhysicsPlugin)
        .add_plugin(PhysicsDebugPlugin)         // Added to enable debug rendering, in this case, for the chunks
        .add_plugin(CameraTargetPlugin)
        .add_startup_system(startup)
        .add_system(move_player)
        .run();
}

/// Player marker component
#[derive(Component)]
struct Player {
    move_speed: f32,
    jump_speed: f32
}

/// Spawns light, chunks and camera
fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {

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
    let chunk = generate_chunk();
    commands.spawn_bundle(VoxelChunkBundle::new(
        chunk,                                      // Raw chunk data
        Transform::from_xyz(0.0, 0.0, 0.0),         // Center of the chunk in units
        Bounds::new(Vec3::new(8.0, 2.0, 8.0))       // Size of the chunk in units
    )).insert(DebugRender);                         // Allows debug info of chunk to be rendered

    // Spawns player
    let player = commands.spawn()
        .insert_bundle(PbrBundle {
            mesh: meshes.add(shape::Box::new(0.5, 1.0, 0.5).into()),
            material: materials.add(Color::RED.into()),
            ..default()
        })
        .insert_bundle(
            PhysicsBundle {
                current_transform: CurrentTransform(Transform::from_xyz(0.0, 0.0, 0.0)),
                bounds: Bounds::new(Vec3::new(0.5, 1.0, 0.5)),
                velocity: Velocity(Vec3::new(0.0, 0.0, 0.0)),
                friction: Friction(Vec3::new(0.7, 0.0, 0.7)),
                ..default()
            }
        )
        .insert(Player {
            move_speed: 0.01,
            jump_speed: 0.1
        })
        .id();

    // Spawns camera
    commands.spawn()
        .insert_bundle(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 2.0, 7.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..default()
        })
        .insert_bundle(CameraTargetBundle {
            target: Target::Entity(player),
            target_style: TargetStyle::Offset(Vec3::new(0.0, 5.0, 7.0)),
            ..default()
        });
}

fn move_player(
    mut input: Res<Input<KeyCode>>,
    mut player_query: Query<(&Player, &mut Velocity)>
) {

    // Reads keyboard input and determines direction vector
    let mut x = 0;
    let mut z = 0;
    if input.pressed(KeyCode::Left) {
        x -= 1;
    }
    if input.pressed(KeyCode::Right) {
        x += 1;
    }
    if input.pressed(KeyCode::Up) {
        z += 1;
    }
    if input.pressed(KeyCode::Down) {
        z -= 1;
    }
    use std::f32::consts::FRAC_1_SQRT_2;
    let dir = match (x, z) {
        (1, 0) => Vec2::new(1.0, 0.0),
        (1, 1) => Vec2::new(FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
        (0, 1) => Vec2::new(0.0, -1.0),
        (-1, 1) => Vec2::new(-FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
        (-1, 0) => Vec2::new(-1.0, 0.0),
        (-1, -1) => Vec2::new(-FRAC_1_SQRT_2, FRAC_1_SQRT_2),
        (0, -1) => Vec2::new(0.0, 1.0),
        (1, -1) => Vec2::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2),
        _ => Vec2::ZERO
    };

    // Applies direction vector
    for (player, mut velocity) in &mut player_query {
        velocity.0.x += dir.x * player.move_speed;
        velocity.0.z += dir.y * player.move_speed;
    }
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

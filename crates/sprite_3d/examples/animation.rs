use bevy::prelude::*;
use bevy::prelude::shape::Cube;
use bevy::render::camera::Projection;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(startup)
        .run();
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {

    // Spawns red cube
    let mesh = meshes.add(Cube::new(16.0).into());
    let material = materials.add(StandardMaterial {
        base_color: Color::rgb(1.0, 0.1, 0.1),
        ..default()
    });
    commands.spawn_bundle(PbrBundle {
        mesh,
        material,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    // Spawns point light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            color: Color::WHITE,
            intensity: 30_000.0,
            range: 50.0,
            ..default()
        },
        transform: Transform::from_xyz(10.0, 20.0, 15.0),
        ..default()
    });

    // Spawns camera
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(30.0, 50.0, 50.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        projection: Projection::Perspective(PerspectiveProjection {
            fov: 70.0,
            aspect_ratio: 16.0/9.0,
            near: 0.1,
            far: 10000.0,
        }),
        ..default()
    });
}
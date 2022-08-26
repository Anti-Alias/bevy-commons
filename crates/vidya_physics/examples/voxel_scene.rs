use bevy::prelude::shape::{Plane, Icosphere};
use vidya_interp::{CurrentTransform, PreviousTransform};
use vidya_physics::*;
use bevy::prelude::*;

/// Example where only a single falling entity is spawned.
pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .add_startup_system(startup)
        .run();
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {

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

    // Spawns plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Plane { size: 10.0 }.into()),
        material: materials.add(Color::GREEN.into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    // Spawns camera
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 2.0, 10.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..default()
    });
}
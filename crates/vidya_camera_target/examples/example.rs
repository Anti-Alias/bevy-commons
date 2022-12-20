use std::time::Duration;

use bevy::prelude::*;
use vidya_camera_target::prelude::*;
use vidya_fixed_timestep::prelude::*;
use vidya_fixed_timestep::FixedTimestepStages;
use bevy::prelude::shape::{ Plane, Icosphere };


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FixedTimestepPlugin::new(Duration::from_secs_f64(1.0/20.0)))
        .add_plugin(CameraTargetPlugin)
        .add_startup_system(startup)
        .add_system_to_stage(FixedTimestepStages::PostFixedUpdate, move_ball)
        .run();
}

/// Marker component for a moving ball
#[derive(Component, Debug, Copy, Clone, Eq, PartialEq)]
pub struct MovingBall;

// Defines the ring of the ball
const RADIUS: f32 = 2.0;
const Y: f32 = 1.0;
const SPEED: f32 = 0.05;

/// Spawns ball, floor plane and camera
fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    // Spawns light above scene
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 10.0, 0.0),
        ..default()
    });

    // Spawns plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane { size: 10.0 }.into()),
        material: materials.add(Color::GREEN.into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    // Spawns moving ball
    let ball = commands.spawn(
        PbrBundle {
            mesh: meshes.add(Icosphere { radius: 0.5, subdivisions: 3 }.into()),
            material: materials.add(Color::RED.into()),
            transform: Transform::from_xyz(0.0, 2.0, 0.0),
            ..default()
        })
        .insert((
            MovingBall,
            CurrentTransform(Transform::from_xyz(RADIUS, Y, 0.0)),
            PreviousTransform::default(),
        ))
        .id();

    // Spawns camera
    commands.spawn(
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 2.0, 10.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..default()
        })
        .insert(CameraTargetBundle {
            target: Target::Entity(ball),
            target_style: TargetStyle::Offset(Vec3::new(0.0, 7.0, 7.0)),
            ..default()
        });
}

fn move_ball(
    mut ball_query: Query<&mut CurrentTransform, With<MovingBall>>,
    mut radians: Local<f32>
) {
    *radians += SPEED;
    for mut ball_trans in &mut ball_query {
        ball_trans.0.translation = Vec3::new(radians.cos() * RADIUS, Y, radians.sin() * RADIUS);
    }
}
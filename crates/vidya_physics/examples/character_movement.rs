use vidya_camera_target::prelude::*;
use vidya_fixed_timestep::{CurrentTransform, PreviousTransform, FixedTimestepPlugin};
use vidya_physics::*;
use bevy::prelude::*;

// Marks ball entity
#[derive(Component, Debug)]
struct Ball;

// Wall constants
const FLOOR: f32 = 0.0;
const LEFT_WALL: f32 = -5.0;
const RIGHT_WALL: f32 = 5.0;
const NEAR_WALL: f32 = 5.0;
const FAR_WALL: f32 = -5.0;
const JUMP_SPEED: f32 = 0.2;

/// Example where only a single falling entity is spawned.
pub fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 512.0,
            height: 512.0,
            position: WindowPosition::At(Vec2::new(1024.0, 0.0)),
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(FixedTimestepPlugin::default())
        .add_plugin(PhysicsPlugin)
        .add_plugin(CameraTargetPlugin)
        .add_startup_system(startup)
        .add_system(bounce_ball)
        .run();
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {

    // Adds gravity
    commands.insert_resource(Gravity(Vec3::new(0.0, -0.01, 0.0)));

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
        mesh: meshes.add(shape::Plane { size: 10.0 }.into()),
        material: materials.add(Color::GREEN.into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    // Spawns player
    let start_transform = Transform::from_xyz(0.0, 0.5, 0.0);
    let player = commands
        .spawn()
        .insert_bundle(PbrBundle {
            mesh: meshes.add(shape::Box::new(1.0, 1.0, 1.0).into()),
            material: materials.add(Color::RED.into()),
            ..default()
        })
        .insert_bundle(
            PhysicsBundle {
                current_transform: CurrentTransform(start_transform),
                previous_transform: PreviousTransform(start_transform),
                bounds: Bounds::new(Vec3::new(1.0, 1.0, 1.0)),
                velocity: Velocity(Vec3::new(0.05, JUMP_SPEED, 0.025)),
                ..default()
            }
        )
        .insert(Ball)
        .id();

    // Spawns camera
    commands
        .spawn()
        .insert_bundle(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 2.0, 10.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..default()
        })
        .insert_bundle(CameraTargetBundle {
            target: Target::Entity(player),
            target_style: TargetStyle::Offset(Vec3::new(0.0, 7.0, 7.0)),
            ..default()
        });
}

fn bounce_ball(mut entities: Query<
    (
        &mut CurrentTransform,
        &Bounds,
        &mut Velocity
    ),
    With<Ball>>
) {
    for (mut trans, bounds, mut vel) in &mut entities {

        // Bounces off floor
        let trans = &mut trans.0.translation;
        if trans.y - bounds.half_extents.y <= FLOOR {
            trans.y = FLOOR + bounds.half_extents.y;
            vel.0.y = JUMP_SPEED;
        }

        // Bounces off left wall
        if trans.x - bounds.half_extents.x <= LEFT_WALL {
            trans.x = LEFT_WALL + bounds.half_extents.x;
            vel.0.x *= -1.0;
        }

        // Bounces off right wall
        if trans.x + bounds.half_extents.x >= RIGHT_WALL {
            trans.x = RIGHT_WALL - bounds.half_extents.x;
            vel.0.x *= -1.0;
        }

        // Bounces off near wall
        if trans.z + bounds.half_extents.z >= NEAR_WALL {
            trans.z = NEAR_WALL - bounds.half_extents.z;
            vel.0.z *= -1.0;
        }

        // Bounces off far wall
        if trans.z - bounds.half_extents.z <= FAR_WALL {
            trans.z = FAR_WALL + bounds.half_extents.z;
            vel.0.z *= -1.0;
        }
    }
}
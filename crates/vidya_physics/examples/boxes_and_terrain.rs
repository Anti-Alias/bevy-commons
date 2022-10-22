use rand::{SeedableRng, RngCore, Rng};
use vidya_camera_target::prelude::*;
use vidya_fixed_timestep::{CurrentTransform, FixedTimestepPlugin};
use vidya_physics::*;
use vidya_physics::debug::*;

use rand::rngs::StdRng;

use bevy::prelude::*;

use bevy_inspector_egui::WorldInspectorPlugin;

// Marks ball entity
#[derive(Component, Debug, Reflect)]
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
        .register_type::<Ball>()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(FixedTimestepPlugin::default())
        .add_plugin(PhysicsPlugin)
        .add_plugin(PhysicsDebugPlugin)
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

    // Random number generator
    let mut rng = StdRng::from_seed([
        42,42,42,42,42,42,42,42,
        42,42,42,42,42,42,42,42,
        42,42,42,42,42,42,42,42,
        42,42,42,42,42,42,42,42
    ]);

    // Adds gravity
    commands.insert_resource(Gravity(Vec3::new(0.0, -0.005, 0.0)));

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

    // Spawns boxes
    const SPEED: f32 = 0.05;
    for _ in 0..10 {
        commands.spawn_bundle(PhysicsBundle {
            current_transform: CurrentTransform(Transform::from_translation(Vec3::new(
                rng.gen_range(-3.0..3.0),
                rng.gen_range(-3.0..3.0),
                rng.gen_range(-3.0..3.0)
            ))),
            bounds: HalfExtents::new(0.5, 0.5, 0.5),
            shape: Shape::Cuboid,
            weight: Weight(1.0),
            config: CollisionConfig::new(GROUP_BASIC, GROUP_ALL),
            velocity: Velocity(Vec3::new(
                rng.gen_range(-SPEED..SPEED),
                rng.gen_range(-SPEED..SPEED),
                rng.gen_range(-SPEED..SPEED),
            )),
            friction: Friction::new(1.0),
            ..default()
        })
        .insert(DebugRender::default());
    }
    

    // Spawns camera
    commands
        .spawn()
        .insert_bundle(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 2.0, 10.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..default()
        });
}

fn bounce_ball(mut entities: Query<
    (
        &mut CurrentTransform,
        &HalfExtents,
        &mut Velocity
    ),
    With<Ball>>
) {
    for (mut trans, bounds, mut vel) in &mut entities {

        // Bounces off floor
        let trans = &mut trans.0.translation;
        if trans.y - bounds.0.y <= FLOOR {
            trans.y = FLOOR + bounds.0.y;
            vel.0.y = JUMP_SPEED;
        }

        // Bounces off left wall
        if trans.x - bounds.0.x <= LEFT_WALL {
            trans.x = LEFT_WALL + bounds.0.x;
            vel.0.x *= -1.0;
        }

        // Bounces off right wall
        if trans.x + bounds.0.x >= RIGHT_WALL {
            trans.x = RIGHT_WALL - bounds.0.x;
            vel.0.x *= -1.0;
        }

        // Bounces off near wall
        if trans.z + bounds.0.z >= NEAR_WALL {
            trans.z = NEAR_WALL - bounds.0.z;
            vel.0.z *= -1.0;
        }

        // Bounces off far wall
        if trans.z - bounds.0.z <= FAR_WALL {
            trans.z = FAR_WALL + bounds.0.z;
            vel.0.z *= -1.0;
        }
    }
}
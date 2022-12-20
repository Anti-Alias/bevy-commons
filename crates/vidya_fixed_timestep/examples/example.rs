use std::time::Duration;

use bevy::prelude::*;
use bevy::prelude::shape::Icosphere;
use vidya_fixed_timestep::{FixedTimestepPlugin, FixedTimestepStages, CurrentTransform, PreviousTransform};

const SRC: Transform = Transform::from_xyz(-2.0, 0.0, 0.0);
const DEST: Transform = Transform::from_xyz(2.0, 0.0, 0.0)
    .with_scale(Vec3::new(2.0, 0.5, 1.0))
    .with_rotation(Quat::from_xyzw(1.0, 2.0, 3.0, 1.0));

#[derive(Component)]
struct Ball;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FixedTimestepPlugin::new(Duration::from_secs(1)))
        .add_system_to_stage(FixedTimestepStages::PostFixedUpdate, move_ball)
        .add_startup_system(startup)
        .run();
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {

    // Spawns camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 10.0),
        ..default()
    });

    // Creates ball mesh / material
    let ball_mesh: Mesh = Icosphere {
        radius: 1.0,
        subdivisions: 3
    }.into();
    let ball_material: StandardMaterial = Color::RED.into();

    // Spawns ball to interpolate
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(ball_mesh),
            material: materials.add(ball_material),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(Ball)
        .insert(PreviousTransform(SRC))
        .insert(CurrentTransform(DEST));

    // Spawns light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 5.0, 0.0),
        ..default()
    });
}

fn move_ball(mut query: Query<&mut CurrentTransform>, mut toggle: Local<bool>) {
    for mut trans in &mut query {
        if *toggle {
            trans.0 = DEST;
        }
        else {
            trans.0 = SRC;
        }
    }

    *toggle = !*toggle;
}
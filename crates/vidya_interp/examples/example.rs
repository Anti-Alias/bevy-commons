use bevy::prelude::*;
use bevy::prelude::shape::Icosphere;
use bevy_time::FixedTimestep;
use vidya_interp::{InterpolationPlugin, PreviousTransform, CurrentTransform, sync_transforms};

const MY_TIMESTEP: &str = "MY_TIMESTEP";
const SRC: Transform = Transform::from_xyz(-2.0, 0.0, 0.0);
const DEST: Transform = Transform::from_xyz(2.0, 0.0, 0.0)
    .with_scale(Vec3::new(2.0, 0.5, 1.0))
    .with_rotation(Quat::from_xyzw(1.0, 2.0, 3.0, 1.0));

#[derive(Component)]
struct Ball;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)

        // Fixed timestep stage that runs once a second
        .add_stage_before(
            CoreStage::PostUpdate,                              // Movement/syncing systems must run before PostUpdate, as that's when the InterpolationPlugin runs it's systems
            MY_TIMESTEP,
            SystemStage::parallel()
                .with_run_criteria(FixedTimestep::step(1.0).with_label(MY_TIMESTEP))
                .with_system(sync_transforms.label("SYNC"))     // Syncs previous transform state with current
                .with_system(move_ball.after("SYNC"))           // Updates current transform state after transforms have been synced
        )
        .add_plugin(InterpolationPlugin::new(MY_TIMESTEP))      // Plugin must know the name of the timestep to get correct interpolation value
        .add_startup_system(startup)
        .run();
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {

    // Spawns camera
    commands.spawn_bundle(Camera3dBundle {
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
        .spawn()
        .insert_bundle(PbrBundle {
            mesh: meshes.add(ball_mesh),
            material: materials.add(ball_material),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(PreviousTransform(SRC))
        .insert(CurrentTransform(DEST));

    // Spawns light
    commands.spawn_bundle(PointLightBundle {
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
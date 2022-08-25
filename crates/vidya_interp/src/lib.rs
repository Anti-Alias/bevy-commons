use bevy_app::{ App, Plugin, CoreStage };
use bevy_time::FixedTimesteps;
use bevy_transform::prelude::*;
use bevy_ecs::prelude::*;


/// Plugin that interpolates [`Transform`] components between
/// [`PreviousTransform`] and [`CurrentTransform`] components during the [`CoreStage::PostUpdate`] stage.
pub struct InterpolationPlugin {
    timestep_label: String
}
impl InterpolationPlugin {
    pub fn new(timestep_label: impl Into<String>) -> Self {
        Self { timestep_label: timestep_label.into() }
    }
}
impl Plugin for InterpolationPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(InterpolationLabel(self.timestep_label.clone()))
            .add_system_to_stage(CoreStage::PostUpdate, interpolate);
    }
}

/// Resource that stores the label of the timestep we wish to interpolate across
struct InterpolationLabel(String);

/// Transform of [`Entity`] during current game tick
#[derive(Component, Debug, PartialEq, Clone, Copy)]
pub struct CurrentTransform(pub Transform);

/// Transform of [`Entity`] during previous game tick
#[derive(Component, Debug, PartialEq, Clone, Copy)]
pub struct PreviousTransform(pub Transform);

/// Interpolates [`Transform`] components between [`PreviousTransform`] and [`CurrentTransform`]1
fn interpolate(
    label: Res<InterpolationLabel>,
    timesteps: Res<FixedTimesteps>,
    mut query: Query<(&PreviousTransform, &CurrentTransform, &mut Transform)>
) {
    // Gets interpolation value from specified timestep
    let t = timesteps
        .get(&label.0)
        .expect("Missing timestep")
        .overstep_percentage() as f32;

    for (prev, current, mut trans) in &mut query {
        trans.translation = prev.0.translation.lerp(current.0.translation, t);
        trans.scale = prev.0.scale.lerp(current.0.scale, t);
        trans.rotation = prev.0.rotation.lerp(current.0.rotation, t);
    }
}

/// Reusable system that syncs the previous transform state with the current    .
/// Should run before updating [`CurrentTransform`].
pub fn sync_transforms(mut query: Query<(&mut PreviousTransform, &CurrentTransform)>) {
    for (mut prev, current) in &mut query {
        prev.0 = current.0;
    }
}
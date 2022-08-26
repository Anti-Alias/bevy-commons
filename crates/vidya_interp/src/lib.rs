use std::marker::PhantomData;

use bevy_app::{ App, Plugin, CoreStage };
use bevy_time::FixedTimesteps;
use bevy_transform::{prelude::*, TransformSystem};
use bevy_ecs::prelude::*;


/// Plugin that interpolates [`Transform`] components between
/// [`PreviousTransform`] and [`CurrentTransform`] components during the [`CoreStage::PostUpdate`] stage.
/// It is the reponsibility of the user of this plugin to properly set those components during the fixed update.
/// The user should also ensure that their fixed timestep runs prior to the [`CoreStage::PostUpdate`] stage for
/// maximum responsiveness.
pub struct InterpolationPlugin<M: Component> {
    timestep_label: String,
    phantom: PhantomData<M>
}
impl<M: Component> InterpolationPlugin<M> {
    /// Creates plugin that listens for updates during the specified labelled timestep
    pub fn new(timestep_label: impl Into<String>) -> Self {
        Self {
            timestep_label: timestep_label.into(),
            phantom: PhantomData
        }
    }
}
impl<M: Component> Plugin for InterpolationPlugin<M> {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(InterpolationLabel {
                value: self.timestep_label.clone(),
                phantom: PhantomData::<M>
            })
            .add_system_to_stage(CoreStage::PostUpdate,
                interpolate::<M>
                    .label(InterpolationSystems::Interpolate)
                    .before(TransformSystem::TransformPropagate)    // Interpolating transforms should happen before being propagated to GlobalTransform to prevent a 1-frame delay
            );
    }
}

#[derive(SystemLabel, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum InterpolationSystems {
    Interpolate
}

/// Resource that stores the label of the timestep we wish to interpolate across
struct InterpolationLabel<M: Component> {
    value: String,
    phantom: PhantomData<M>
}

/// Transform of [`Entity`] during current game tick
#[derive(Component, Default, Debug, PartialEq, Clone, Copy)]
pub struct CurrentTransform(pub Transform);

/// Transform of [`Entity`] during previous game tick
#[derive(Component, Default, Debug, PartialEq, Clone, Copy)]
pub struct PreviousTransform(pub Transform);

/// Interpolates [`Transform`] components between [`PreviousTransform`] and [`CurrentTransform`]1
fn interpolate<M: Component>(
    label: Res<InterpolationLabel<M>>,
    timesteps: Res<FixedTimesteps>,
    mut query: Query<(&PreviousTransform, &CurrentTransform, &mut Transform), With<M>>
) {
    let t = timesteps
        .get(&label.value)
        .expect("Missing timestep")
        .overstep_percentage() as f32;
    for (prev, current, mut trans) in &mut query {
        trans.translation = prev.0.translation.lerp(current.0.translation, t);
        trans.scale = prev.0.scale.lerp(current.0.scale, t);
        trans.rotation = prev.0.rotation.lerp(current.0.rotation, t);
    }
}

/// Reusable system that syncs the previous transform state with the current.
/// Should run before updating [`CurrentTransform`].
pub fn sync_transforms<M: Component>(mut query: Query<(&mut PreviousTransform, &CurrentTransform), With<M>>) {
    for (mut prev, current) in &mut query {
        prev.0 = current.0;
    }
}
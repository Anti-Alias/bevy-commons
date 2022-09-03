use std::time::Duration;

use bevy_app::prelude::*;
use bevy_ecs::schedule::IntoSystemDescriptor;
use bevy_transform::prelude::*;
use bevy_ecs::prelude::*;
use bevy_time::{FixedTimestep, FixedTimesteps};

/// Label for fixed timestep
static VIDYA_FIXED: &str = "VIDYA_FIXED";


/// Plugin that interpolates [`Transform`] components between
/// [`PreviousTransform`] and [`CurrentTransform`] components during the [`CoreStage::PostUpdate`] stage.
/// It is the reponsibility of the user of this plugin to properly set those components during the fixed update.
/// The user should also ensure that their fixed timestep runs prior to the [`CoreStage::PostUpdate`] stage for
/// maximum responsiveness.
pub struct FixedTimestepPlugin {
    step: Duration
}
impl FixedTimestepPlugin {
    /// Creates the plugin with the desired timestep duration.
    pub fn new(step: Duration) -> Self {
        Self { step }
    }
}
impl Default for FixedTimestepPlugin {
    fn default() -> Self {
        Self { step: Duration::from_secs_f64(1.0/60.0) }
    }
}
impl Plugin for FixedTimestepPlugin {
    fn build(&self, app: &mut App) {

        // Sync stage
        let step = self.step.as_secs_f64();
        app
            .add_stage_after(
                CoreStage::Update,
                FixedTimestepStages::FixedUpdate,
                SystemStage::parallel()
                    .with_run_criteria(
                        FixedTimestep::step(step).with_label(VIDYA_FIXED)
                    )
            )
            .add_stage_after(
                FixedTimestepStages::FixedUpdate,
                FixedTimestepStages::SyncTransforms,
                SystemStage::single(sync_transforms).with_run_criteria(FixedTimestep::step(step))
            )
            .add_stage_after(
                FixedTimestepStages::SyncTransforms,
                FixedTimestepStages::PostFixedUpdate,
                SystemStage::parallel().with_run_criteria(FixedTimestep::step(step))
            )
            .add_stage_after(
                FixedTimestepStages::PostFixedUpdate,
                FixedTimestepStages::InterpolateTransforms,
                SystemStage::single_threaded()
                    .with_system(sync_added_transforms.label(FixedTimestepSystems::SyncAddedTransforms))
                    .with_system(interpolate_transforms
                        .label(FixedTimestepSystems::InterpolateTransforms)
                        .after(FixedTimestepSystems::SyncAddedTransforms)
                    )
            );
    }
}

/// Labels for stages used by the fixed timestep plugin.
/// Each stage is positioned between [`CoreStage::Update`] and [`CoreStage::PostUpdate`] and in the order specified.
#[derive(StageLabel, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum FixedTimestepStages {
    /// Fixed-timestep version of [`CoreStage::Update`].
    /// For general game logic.
    /// Manipulating a [`CurrentTransform`] here will cause the entity to "teleport".
    FixedUpdate,
    /// Simple stage where [`PreviousTransform`]s get synced with [`CurrentTransform`]s.
    /// Not to be trifled with! (Psst, don't add systems to it!)
    SyncTransforms,
    /// Fixed-timestep version of [`CoreStage::PostUpdate`].
    /// Great place to put a physics engine.
    /// Manipulating a [`CurrentTransform`] here will cause the entity to "interpolate".
    PostFixedUpdate,
    /// Stage where [`Transform`]s are interpolated between [`PreviousTransform`]s and [`CurrentTransform`]s.
    InterpolateTransforms
}

/// Labels for systems.
#[derive(SystemLabel, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum FixedTimestepSystems {
    SyncAddedTransforms,
    InterpolateTransforms
}

/// Transform of [`Entity`] during current game tick
#[derive(Component, Default, Debug, PartialEq, Clone, Copy)]
pub struct CurrentTransform(pub Transform);

/// Transform of [`Entity`] during previous game tick
#[derive(Component, Default, Debug, PartialEq, Clone, Copy)]
pub struct PreviousTransform(pub Transform);

/// Interpolates [`Transform`] components between [`PreviousTransform`] and [`CurrentTransform`]1
fn interpolate_transforms(
    timesteps: Res<FixedTimesteps>,
    mut query: Query<(&PreviousTransform, &CurrentTransform, &mut Transform)>
) {
    let t = timesteps
        .get(VIDYA_FIXED)
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
fn sync_transforms(mut query: Query<(&mut PreviousTransform, &CurrentTransform)>) {
    for (mut prev, current) in &mut query {
        prev.0 = current.0;
    }
}

/// Reusable system that syncs the previous transform state with the current.
/// Ensures that newly added entities with both a PreviousTransform and CurrentTransform
/// are synced before use to prevent odd interpolation errors.
fn sync_added_transforms(mut query: Query<
    (
        &mut PreviousTransform,
        &CurrentTransform
    ),
    (
        Added<PreviousTransform>,
        Added<CurrentTransform>
    )>
) {
    for (mut prev, current) in &mut query {
        prev.0 = current.0;
    }
}


/// This trait adds a helper method for adding fixed systems
pub trait AppExt {
    fn add_fixed_system<Params>(&mut self, system: impl IntoSystemDescriptor<Params>) -> &mut Self;
    fn add_fixed_system_set(&mut self, system_set: SystemSet) -> &mut Self;
}
impl AppExt for App {
    fn add_fixed_system<Params>(&mut self, system: impl IntoSystemDescriptor<Params>) -> &mut Self {
        self.add_system_to_stage(FixedTimestepStages::FixedUpdate, system);
        self
    }
    fn add_fixed_system_set(&mut self, system_set: SystemSet) -> &mut Self {
        self.add_system_set_to_stage(FixedTimestepStages::FixedUpdate, system_set);
        self
    }
}

/// Prelude module
pub mod prelude {
    pub use crate::{
        FixedTimestepPlugin,
        CurrentTransform,
        PreviousTransform,
        AppExt
    };
}
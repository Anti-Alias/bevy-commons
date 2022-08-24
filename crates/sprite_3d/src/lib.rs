use std::time::Duration;

use bevy::prelude::*;
use bevy::reflect::TypeUuid;

#[derive(Copy, Clone, Eq, PartialEq, Hash, SystemLabel)]
enum Labels {
    UPDATE_ANIMATIONS
}

pub struct AnimationPlugin;
impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_asset::<Animation>()
            .add_asset::<AnimationSet>()
            .add_system_to_stage(CoreStage::PostUpdate, update_animations.label(Labels::UPDATE_ANIMATIONS));
    }
}

// Represents a single frame in an animation.
// References a [`TextureAtlas`].
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Frame {
    index: usize,
    duration: Duration
}

/// Asset storing frames in an animation.
#[derive(TypeUuid)]
#[uuid="1ca32196-202e-11ed-861d-0242ac120002"]
pub struct Animation(Vec<Frame>);

/// A set of grouped animations.
#[derive(TypeUuid, Default)]
#[uuid = "0b833742-202f-11ed-861d-0242ac120002"]
pub struct AnimationSet {
    animations: Vec<Handle<Animation>>,
    groups: Vec<Vec<usize>>
}
impl AnimationSet {

    /// Adds an ungrouped animation
    pub fn add_animation(&mut self, animation: Handle<Animation>) {
        self.animations.push(animation);
    }

    /// Adds a group of animations
    pub fn add_group(&mut self, group: &[Handle<Animation>]) -> &mut Self {
        let mut group_indices = Vec::new();
        for i in 0..group.len() {
            group_indices.push(self.animations.len() + i);
        }
        for handle in group {
            self.animations.push(handle.clone());
        }
        self.groups.push(group_indices);
        self
    }

    /// Gets animation by group and animation index
    pub fn get_grouped_animation(&self, group_index: usize, animation_index: usize) -> Option<&Handle<Animation>> {
        self.groups
            .get(group_index)
            .and_then(|group_vec| group_vec.get(animation_index))
            .and_then(|anim_index| self.animations.get(*anim_index))
    }
}

/// Component that stores instance data about an animation.
#[derive(Component)]
pub struct AnimationState {
    current_animation: Handle<Animation>,
    current_frame_index: usize,
    accum: Duration
}
impl AnimationState {
    pub fn new(current_animation: Handle<Animation>) -> Self {
        Self {
            current_animation,
            current_frame_index: 0,
            accum: Duration::ZERO
        }
    }
}

/// Repeating timer that updates the current frame in an [`Animation`] when it repeats.
#[derive(Component)]
pub struct AnimationTimer(Timer);
impl AnimationTimer {
    pub fn new(duration: Duration) -> Self {
        Self(Timer::new(duration, true))
    }
}

// Updates the animation types
fn update_animations(
    time: Res<Time>,
    mut animations: Query<(
        &mut AnimationTimer
    )>
) {
    for (mut animation, mut timer) in &mut animations {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {

        }
    }
}
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;

/// Plugin that adds the capability to spawn dialog boxes
pub struct DialogPlugin {
    camera_index: i32
}
impl DialogPlugin {
    pub fn new(camera_index: i32) -> Self {
        Self { camera_index }
    }
}
impl Plugin for DialogPlugin {
    fn build(&self, app: &mut App) {

    }
}

#[derive(Component)]
pub struct DialogBox {
    pub text: String,
    pub char_index: usize
}
use bevy::prelude::*;
use crate::screen::{ScreenLoader, ScreenLoaders};

/// Event that instructs the game to load a load the title screen
#[derive(Clone, Debug)]
pub struct LoadTitleScreenEvent(&'static str);

struct UIScreenLoader;
impl ScreenLoader for UIScreenLoader {
    fn load(&self, name: &'static str, commands: &mut Commands) {
        commands.add(FireEvent(LoadTitleScreenEvent(name)));
    }
}

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ScreenLoaders::default()
            .insert("title_screen", UIScreenLoader)
            .insert("options_screen", UIScreenLoader)
        )
        .run();
}
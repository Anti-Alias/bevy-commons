use bevy::prelude::*;
use bevy::utils::HashMap;

pub struct ScreenLoaderPlugin;
impl Plugin for ScreenLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PostUpdate, handle_load_requests);
    }
}


/// Resource that can be used to load named screens
#[derive(Default)]
pub struct ScreenLoaders {
    load_requests: Vec<&'static str>,
    loaders: HashMap<&'static str, ScreenLoader>
}
impl ScreenLoaders {
    /// Inserts a named screen loader.
    pub fn insert(mut self, name: &'static str, loader: ScreenLoader) -> Self {
        self.loaders.insert(name, loader);
        self
    }

    /// Unloads current screen if applicable, and loads screen specified.
    pub fn load_screen(&mut self, name: &'static str) {
        self.load_requests.push(name);
    }
}

/// Resource that names the current screen.
pub struct CurrentScreen(&'static str);

/// Responsible for loading/unloading a screen.
#[derive(Copy, Clone)]
pub struct ScreenLoader {
    /// Function pointer responsible for loading the screen.
    pub load_fn: fn(&'static str, world: &mut World),

    /// Function pointer reponsible for unloading the screen.
    pub unload_fn: fn(&'static str, world: &mut World)
}
impl ScreenLoader {
    /// Creates a screen loader instance with a default unloader function.
    /// Upon unloading, entities not marked with the [`Retain`] component will be despawned.
    pub fn new(load_fn: fn(&'static str, world: &mut World)) -> Self {
        Self {
            load_fn,
            unload_fn: unload_default
        }
    }
}

/// Unload function that despawns entities without the [`Retain`] marker component.
pub fn unload_default(_name: &'static str, world: &mut World) {
    let mut query = world.query_filtered::<Entity, Without<Retain>>();
    let entities: Vec<Entity> = query.iter(world).collect();
    for entity in entities {
        world.despawn(entity);
    }
}

/// Marker trait that tells screen loaders to not depsawn a particular entity when loading a new screen.
#[derive(Component)]
pub struct Retain;

/// Reads screen load requests, and kicks them off
fn handle_load_requests(
    loaders: Res<ScreenLoaders>,
    current_screen: Option<ResMut<CurrentScreen>>,
    mut commands: Commands
) {
    // For all requests...
    for screen_name in &loaders.load_requests {

        let screen_name = screen_name.clone();
        
        // If we're currently on a screen, unload the current screen first
        if let Some(ref current_screen) = current_screen {
            let current_screen_name = current_screen.0;
            let unload_fn = loaders.loaders[current_screen_name].load_fn;
            commands.add(move |world: &mut World| {
                unload_fn(current_screen_name, world);
            });
        }

        // Load requested screen
        let load_fn = loaders.loaders[screen_name].load_fn;
        commands.add(move |world: &mut World| {
            load_fn(screen_name, world);
        });

        // Set current screen
        commands.insert_resource(CurrentScreen);
    }
}
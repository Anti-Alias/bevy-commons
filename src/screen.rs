use bevy::prelude::*;
use bevy::utils::HashMap;

/// Plugin that implements screen loading/unloading.
#[derive(Default)]
pub struct ScreenPlugin {
    screens: Screens
}
impl ScreenPlugin {

    /// Inserts a named screen loader with the default unload function.
    pub fn insert(mut self, name: &'static str, load_fn: fn(&'static str, &mut World)) -> Self {
        self.screens.loaders.insert(name, Screen {
            load_fn,
            unload_fn: unload_default
        });
        self
    }

    /// Inserts a named screen loader.
    pub fn insert_screen(mut self, name: &'static str, loader: Screen) -> Self {
        self.screens.loaders.insert(name, loader);
        self
    }
}
impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(self.screens.clone())
            .add_system_to_stage(CoreStage::PostUpdate, handle_load_requests);
    }
}


/// Resource that can be used to load named screens
#[derive(Default, Clone)]
pub struct Screens {
    load_requests: Vec<&'static str>,
    loaders: HashMap<&'static str, Screen>
}
impl Screens {

    /// Unloads current screen if applicable, and loads screen specified.
    pub fn load_screen(&mut self, name: &'static str) {
        self.load_requests.push(name);
    }
}

/// Resource that names the current screen.
pub struct CurrentScreen(&'static str);

/// Responsible for loading/unloading a screen.
#[derive(Copy, Clone)]
pub struct Screen {
    /// Function pointer responsible for loading the screen.
    pub load_fn: fn(&'static str, world: &mut World),

    /// Function pointer reponsible for unloading the screen.
    pub unload_fn: fn(&'static str, world: &mut World)
}

/// Unload function that despawns entities without the [`Retain`] marker component.
pub fn unload_default(_name: &'static str, world: &mut World) {
    let mut query = world.query_filtered::<Entity, Without<Retain>>();
    let entities: Vec<Entity> = query.iter(world).collect();
    for entity in entities {
        world.despawn(entity);
    }
}

/// Marker trait that marks an [`Entity`] as "retained", meaning that it should not be despawned during a screen unload.
/// Screen unloaders are not required to honor this, though the default unloader does.
#[derive(Component)]
pub struct Retain;


/// System that reads screen load requests and kicks them off.
fn handle_load_requests(
    screens: Res<Screens>,
    current_screen: Option<ResMut<CurrentScreen>>,
    mut commands: Commands
) {
    for request_name in &screens.load_requests {
        let screen_name = request_name.clone();
        
        // If we're currently on a screen, unload it
        if let Some(ref current_screen) = current_screen {
            let current_screen_name = current_screen.0;
            let unload_fn = screens.loaders[current_screen_name].load_fn;
            commands.add(move |world: &mut World| {
                unload_fn(current_screen_name, world);
            });
        }

        // Load requested screen
        let load_fn = screens.loaders[screen_name].load_fn;
        commands.add(move |world: &mut World| {
            load_fn(screen_name, world);
        });

        // Set current screen
        commands.insert_resource(CurrentScreen);
    }
}
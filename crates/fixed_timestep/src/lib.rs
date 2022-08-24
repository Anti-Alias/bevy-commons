use std::time::Duration;
use bevy::prelude::*;


// Tiny configuration resource that holds timestep info.
// To be shared by multiple plugins.
pub struct FixedTimestepConfig {
    pub timestep_duration: Duration
}
impl Default for FixedTimestepConfig {
    fn default() -> Self {
        Self {
            timestep_duration: Duration::from_secs_f64(1.0/60.0),
        }
    }
}
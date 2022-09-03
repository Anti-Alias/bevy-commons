use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_ecs::query::QueryEntityError;
use bevy_transform::prelude::*;
use bevy_math::Vec3;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PostUpdate, update_cameras);
    }
}

/// Bundle to be inserted into a 3d camera
#[derive(Bundle)]
pub struct CameraTargetBundle {
    pub target: Target,
    pub target_style: TargetStyle,
    pub up: Up
}

impl Default for CameraTargetBundle {
    fn default() -> Self {
        Self {
            target: Target::Point {
                position: Vec3::ZERO,
                up: Vec3::Y
            },
            target_style: TargetStyle::default(),
            up: Up(Vec3::Y)
        }
    }
}

/// Component to be added to a 3d camera.
/// Determines what should be followed.
#[derive(Component, Debug, Copy, Clone, PartialEq)]
pub enum Target {
    Point {
        position: Vec3,
        up: Vec3
    },
    Entity(Entity)
}

/// Component that determines how a camera should follow its target.
#[derive(Component, Debug, Copy, Clone, PartialEq)]
pub enum TargetStyle {
    Offset(Vec3)
}
impl Default for TargetStyle {
    fn default() -> Self {
        Self::Offset(Vec3::ZERO)
    }
}

/// Optional component to add to targets. Determines the up vector of the camera when being targetted.
/// If not included, camera's up vector will be [0.0, 1.0, 0.0].
#[derive(Component, Debug, Copy, Clone, PartialEq)]
pub struct Up(pub Vec3);


/// Has cameras with a target follow their target
fn update_cameras(
    mut cameras: Query<(&Target, &TargetStyle, &Up, &mut Transform)>,
    target_query: Query<(&Transform, Option<&Up>)>
) {
    for (cam_target, cam_style, cam_up, mut cam_trans) in &mut cameras {
        
        // Gets position / up vectors of camera's target
        let (target_pos, target_up) = match *cam_target {

            // If target is a point, just grab the raw values
            Target::Point { position, up } => (position, up),

            // Otherwise, grab the transform and optional up vector from the entity
            Target::Entity(entity) => {
                let (target_trans, target_up) = match target_query.get(entity) {
                    Ok((trans, up)) => (trans, up),
                    Err(error) => match error {
                        QueryEntityError::NoSuchEntity(_) => {
                            return;
                        }
                        QueryEntityError::QueryDoesNotMatch(_) | QueryEntityError::AliasedMutability(_) => {
                            bevy_log::error!("Camera target did not meet criteria");
                            return;
                        }
                    }
                };
                match target_up {
                    Some(target_up) => (target_trans.translation, target_up.0),
                    None => (target_trans.translation, cam_up.0)
                }
            }
        };

        // Follows target
        match *cam_style {
            TargetStyle::Offset(offset) => {
                cam_trans.translation = target_pos + offset;
                cam_trans.look_at(target_pos, target_up);
            }
        }        
    }
}
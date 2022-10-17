pub use bevy_ecs::prelude::*;
use bevy_math::Vec3;

/// Represents a group that a physics object can belong to.
pub type CollisionGroups = u32;

pub const GROUP_NONE: CollisionGroups =                 0;
pub const GROUP_ALL: CollisionGroups =                  u32::MAX;
pub const GROUP_PARTICLES: CollisionGroups =            0b00000000_00000000_00000000_00000001;
pub const GROUP_STATIC_TERRAIN: CollisionGroups =       0b00000000_00000000_00000000_00000010;
pub const GROUP_MOVING_TERRAIN: CollisionGroups =       0b00000000_00000000_00000000_00000100;
pub const GROUP_PLAYERS: CollisionGroups =              0b00000000_00000000_00000000_00001000;
pub const GROUP_NPCS: CollisionGroups =                 0b00000000_00000000_00000000_00010000;
pub const GROUP_MOBS: CollisionGroups =                 0b00000000_00000000_00000000_00100000;
pub const GROUP_BOSSES: CollisionGroups =               0b11111111_11111111_11111111_11111111;


/// Represents a collision that occurred between two physics objects
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Collision {
    /// Time value between 0 and 1
    pub t: f32,
    /// Amount object B's position should change
    pub position_delta: Vec3,
    /// Amount object B's velocity should change
    pub velocity_delta: Vec3
}


/// Represents the "group" a physics object belongs to, as well as the groups that affect this physics object
#[derive(Component, Copy, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub struct CollisionConfig {
    /// Group(s) a game object belongs to. It's typically only one.
    pub groups: CollisionGroups,
    /// Groups this game object is affected by.
    pub affected_by: CollisionGroups
}
impl CollisionConfig {
    pub fn new(groups: CollisionGroups, affected_by: CollisionGroups) -> Self {
        Self {
            groups,
            affected_by
        }
    }
    pub fn not_affected_by(&self, affected_by: CollisionGroups) -> Self {
        Self {
            groups: self.groups,
            affected_by: self.affected_by & !affected_by
        }
    }
    pub fn affected_by(&self, groups: CollisionGroups) -> bool {
        self.affected_by & groups != 0
    }
    
    pub fn collision_type(a: &CollisionConfig, b: &CollisionConfig) -> CollisionType {
        let a_affected = a.affected_by(b.groups);
        let b_affected = b.affected_by(a.groups);
        match (a_affected, b_affected) {
            (false, false) => CollisionType::NoPush,
            (false, true) => CollisionType::APushesB,
            (true, false) => CollisionType::BPushesA,
            (true, true) => CollisionType::WeightedPush
        }
    }
}

/// Describes how a collision response should be divvyed up between two physics objects should they collide.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum CollisionType {
    /// A and B phase through each other.
    NoPush,
    /// A affects B, but B does not affect A
    APushesB,
    /// B affects A, but A does not affect B
    BPushesA,
    /// A and B push each other, with their weight differences deciding how much.
    WeightedPush,
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn affected_by() {
        let config = CollisionConfig::new(GROUP_PLAYERS, GROUP_ALL)
            .not_affected_by(GROUP_PLAYERS | GROUP_PARTICLES | GROUP_NPCS);

        assert!(!config.affected_by(GROUP_PLAYERS));
        assert!(!config.affected_by(GROUP_PARTICLES));
        assert!(!config.affected_by(GROUP_NPCS));

        assert!(config.affected_by(GROUP_STATIC_TERRAIN));
        assert!(config.affected_by(GROUP_MOVING_TERRAIN));
        assert!(config.affected_by(GROUP_MOBS));
    }
    
}
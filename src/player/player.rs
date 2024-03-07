// Main Player Struct

use bevy::prelude::{Component, Reflect, Vec2, Vec3};

pub const DEFAULT_MODEL: &str = "models/source/balkan romanov.glb";

#[derive(Component, Default, Reflect)]
pub struct Player {
    // Player position in the game world
    pub position: Vec3,
    // Direction of view
    pub view_direction: Vec2,
    // Speed of movement
    pub speed: f32,
    // Remaining health
    pub health: f32,
    // TODO: remove
    pub name: String,
}

/// Main Player Struct
use bevy::prelude::{Component, Entity, KeyCode, Reflect, Vec2, Vec3};
use bevy::prelude::KeyCode::*;

pub const DEFAULT_MODEL: &str = "models/source/low_poly_humanoid_robot.glb";

#[derive(Reflect)]
pub struct KeyboardLayout {
    pub forward: KeyCode,
    pub back: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub turn_left: KeyCode,
    pub turn_right: KeyCode,
}

impl Default for KeyboardLayout {
    fn default() -> Self {
        return Self {
            forward: KeyW,
            back: KeyS,
            left: KeyQ,
            right: KeyE,
            turn_left: KeyA,
            turn_right: KeyD,
        }
    }
    
}

#[derive(Component, Default, Reflect)]
pub struct Player {
    // Player position in the game world
    pub position: Vec3,
    // Direction of view
    pub view_direction: Vec2,
    // Speed of movement
    pub speed: f32,
    // Rotational speed
    pub rotational_speed: f32,
    // Remaining health
    pub health: f32,
    // TODO: remove
    pub name: String,
    pub keyboard_layout: KeyboardLayout,
    pub animation_player: Option<Entity>,
}

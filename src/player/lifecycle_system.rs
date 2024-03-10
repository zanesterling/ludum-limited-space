/// In the lifecycle_system.rs file handles systems related to the lifecycle events of the player
/// entity, such as: Player Spawning/Despawning/Respawning, Handling Player Death and Resources.
use bevy::asset::AssetServer;
use bevy::prelude::*;
use crate::player::player;

#[derive(Component)]
pub struct Player;

// Initial setup of the scene
pub fn initial_spawn(mut commands: Commands,
                     asset_server: Res<AssetServer>,
) {
    commands.spawn(SceneBundle {
        scene: asset_server.load(format!("{}{}", player::DEFAULT_MODEL, "#Scene0")),
        ..Default::default()
    })
        .insert(player::Player{
            position: Vec3::ZERO,
            view_direction: Vec2::ZERO,
            speed: 0.01,
            health: 100.0,
            name: "Player1".parse().unwrap()
        });
}

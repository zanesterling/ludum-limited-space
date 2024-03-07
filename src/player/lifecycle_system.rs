// In the lifecycle_system.rs file handles systems related to the lifecycle events of the player
// entity, such as:
// * Player Spawning;
// * Player Despawning;
// * Player Respawning;
// * Handling Player Death;
// * Managing Player Resources.

use bevy::asset::AssetServer;
use bevy::prelude::*;
use crate::player::player;

#[derive(Component)]
pub(crate) struct Player;

// Initial setup of the scene
pub fn initial_spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    // TODO: what os PbrBundle?
    commands.spawn(
        SceneBundle {
            scene: asset_server.load(format!("{}{}", player::DEFAULT_MODEL, "#Scene0")),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        // .insert(AnimationPlayer::default())
        // TODO: Move to the global state
        .insert(player::Player{position: Vec3::ZERO,
                                      view_direction: Vec2::ZERO,
                                      speed: 0.0,
                                      health: 100.0,
                                      name: "Player1".parse().unwrap()
        });
}

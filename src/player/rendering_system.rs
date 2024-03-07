// Systems responsible for rendering entities:
// * Rendering Sprites;
// * Rendering Meshes;
// * Animating Entities;
// * UI Rendering;
// * Handling Visibility and Occlusion;
// * Post-processing Effects.

use bevy::animation::{AnimationClip, AnimationPlayer};
use bevy::asset::{AssetServer, Handle};
use bevy::prelude::{Commands, Query, Res, Resource};
use crate::player::player;

#[derive(Resource)]
pub struct PlayerAnimations(pub(crate) Vec<Handle<AnimationClip>>);

// Initial setup of the scene
pub fn initialize(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(PlayerAnimations(vec![
        asset_server.load(format!("{}{}", player::DEFAULT_MODEL, "#Animation13")),
        asset_server.load(format!("{}{}", player::DEFAULT_MODEL, "#Animation1")),
    ]));
}

// Once the scene is loaded, start the animation
pub fn setup(
    animations: Res<PlayerAnimations>,
    mut players: Query<&mut AnimationPlayer /*, &player::Player */>,
) {
    for mut animation_player in players.iter_mut() {
        animation_player.play(animations.0[0].clone_weak()).repeat();
    }
}
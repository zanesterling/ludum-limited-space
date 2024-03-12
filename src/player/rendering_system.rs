/// Systems responsible for rendering entities: Rendering Sprites, Meshes, Animating Entities,
/// UI Rendering, Handling Visibility and Occlusion and Post-processing Effects.
use bevy::animation::{AnimationClip, AnimationPlayer};
use bevy::asset::{AssetServer, Handle};
use bevy::input::ButtonInput;
use bevy::prelude::{Added, Commands, KeyCode, Query, Res, Resource};
use crate::player::player;

#[derive(Resource)]
pub struct PlayerAnimations(pub Vec<Handle<AnimationClip>>);

// Initial setup of the scene
pub fn initialize(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(PlayerAnimations(vec![
        asset_server.load(format!("{}{}", player::DEFAULT_MODEL, "#Animation2")),
        asset_server.load(format!("{}{}", player::DEFAULT_MODEL, "#Animation1")),
        asset_server.load(format!("{}{}", player::DEFAULT_MODEL, "#Animation0")),
    ]));
}

// Once the scene is loaded, start the animation
pub fn setup(
    animations: Res<PlayerAnimations>,
    mut players: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    for mut animation_player in players.iter_mut() {
        animation_player.set_speed(1.0);
        animation_player.pause();
        animation_player.play(animations.0[0].clone_weak()).repeat();
    }
}

pub fn move_models(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&player::Player, &mut AnimationPlayer)>,
) {
    for (player_state, mut animation_player) in &mut query {
        if keyboard_input.just_pressed(player_state.keyboard_layout.forward) {
            if animation_player.speed() < 0.0 {
                let current_speed = animation_player.speed();
                animation_player.set_speed(-current_speed);
            }
            animation_player.resume();
        }
        if keyboard_input.just_released(player_state.keyboard_layout.forward) {
            animation_player.pause();
        }

        if keyboard_input.just_pressed(player_state.keyboard_layout.back) {
            if animation_player.speed() > 0.0 {
                let current_speed = animation_player.speed();
                animation_player.set_speed(-current_speed);
            }
            animation_player.resume();
        }
        if keyboard_input.just_released(player_state.keyboard_layout.back) {
            animation_player.pause();
        }
    }
}
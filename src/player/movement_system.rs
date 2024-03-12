/// Player Movement System
use bevy::prelude::*;
use std::f32::consts::PI;
use crate::player::player;
use crate::player::rendering_system::PlayerAnimations;

#[derive(Resource)]
pub struct Animations(Vec<Handle<AnimationClip>>);

pub fn keyboard_animation_control(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&player::Player, &mut AnimationPlayer, &mut Transform)>,
    animations: Res<PlayerAnimations>,
) {
    for (player_state, mut animation_player, mut transform) in &mut query {
        if keyboard_input.just_pressed(KeyCode::Space) {
            if animation_player.is_paused() {
                animation_player.play(animations.0[0].clone_weak()).repeat();
                animation_player.resume();
            } else {
                animation_player.play(animations.0[1].clone_weak()).repeat();
                animation_player.pause()
            }
        }

        animation_player.set_speed(player_state.speed);

        let euler = transform.rotation.to_euler(EulerRot::YXZ);
        let yaw = -euler.0 + PI / 2.0;
        let movement = Vec3::new(yaw.cos(), 0.0, yaw.sin()) * animation_player.speed();

        if keyboard_input.pressed(player_state.keyboard_layout.forward) {
            transform.translation += movement;
        }
        if keyboard_input.pressed(player_state.keyboard_layout.back) {
            transform.translation -= movement;
        }

        let yaw = -euler.0;
        let movement = Vec3::new(yaw.cos(), 0.0, yaw.sin()) * animation_player.speed();
        if keyboard_input.pressed(player_state.keyboard_layout.left) {
            transform.translation += movement;
        }
        if keyboard_input.pressed(player_state.keyboard_layout.right) {
            transform.translation -= movement;
        }

        if keyboard_input.pressed(player_state.keyboard_layout.turn_right) {
            transform.rotate_y(-player_state.rotational_speed);
        }
        if keyboard_input.pressed(player_state.keyboard_layout.turn_left) {
            transform.rotate_y(player_state.rotational_speed);
        }
    }
}
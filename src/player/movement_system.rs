/// Player Movement System
use bevy::prelude::*;
use std::f32::consts::PI;
use crate::player::player;
use crate::player::rendering_system::PlayerAnimations;
use crate::world::controllers::CursorPosition;

#[derive(Resource)]
pub struct Animations(Vec<Handle<AnimationClip>>);

pub fn keyboard_animation_control(
    cursor_position: Res<CursorPosition>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut AnimationPlayer, &mut Transform)>,
    player_query: Query<&player::Player>,
    animations: Res<PlayerAnimations>,
) {
    let player_state = player_query.get_single().unwrap();
    for (mut animation_player, mut transform) in &mut query {
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
        if keyboard_input.pressed(KeyCode::KeyW) {
            transform.translation.x += (cursor_position.global.x - transform.translation.x) * animation_player.speed();
            transform.translation.y += (cursor_position.global.y - transform.translation.y) * animation_player.speed();
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            transform.translation.x -= (cursor_position.global.x - transform.translation.x) * animation_player.speed();
            transform.translation.y -= (cursor_position.global.y - transform.translation.y) * animation_player.speed();
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            transform.translation.y -= (cursor_position.global.x - transform.translation.x) * animation_player.speed();
            transform.translation.x += (cursor_position.global.y - transform.translation.y) * animation_player.speed();
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            transform.translation.y += (cursor_position.global.x - transform.translation.x) * animation_player.speed();
            transform.translation.x -= (cursor_position.global.y - transform.translation.y) * animation_player.speed();
        }


        let direction = cursor_position.global - transform.translation;
        let angle_to_target = direction.y.atan2(direction.x) + PI/2.0;
        transform.rotation = Quat::from_rotation_z(angle_to_target);
    }
}
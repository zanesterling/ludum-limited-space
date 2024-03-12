/// Player Movement System
use bevy::prelude::*;
use std::f32::consts::PI;
use crate::player::player;

#[derive(Resource)]
pub struct Animations(Vec<Handle<AnimationClip>>);

pub fn keyboard_animation_control(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&player::Player, &mut Transform)>,
) {
    for (player_state, mut transform) in &mut query {

        let euler = transform.rotation.to_euler(EulerRot::YXZ);
        // let yaw = -euler.0 + PI / 2.0;
        let yaw = -euler.0 + PI;
        let movement = Vec3::new(yaw.cos(), 0.0, yaw.sin()) * player_state.speed;

        if keyboard_input.pressed(player_state.keyboard_layout.forward) {
            transform.translation += movement;
        }
        if keyboard_input.pressed(player_state.keyboard_layout.back) {
            transform.translation -= movement;
        }

        // let yaw = -euler.0;
        let yaw = -euler.0 + PI / 2.0;
        let movement = Vec3::new(yaw.cos(), 0.0, yaw.sin()) * player_state.speed;
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
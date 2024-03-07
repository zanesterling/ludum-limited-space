// Player Movement System
use bevy::prelude::*;
use crate::player::rendering_system::PlayerAnimations;

// TODO: implement
#[derive(Default)]
pub struct CursorPos(Vec2);

#[derive(Resource)]
pub(crate) struct Animations(Vec<Handle<AnimationClip>>);

pub(crate) fn keyboard_animation_control(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(/*&mut player::Player,*/ &mut AnimationPlayer, &mut Transform)>,
    animations: Res<PlayerAnimations>,
) {
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

        // TODO: move to the player state
        animation_player.set_speed(0.1);
        let mut direction = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::KeyW) {
            direction += transform.local_x().normalize();
        }

        if keyboard_input.pressed(KeyCode::KeyS) {
            direction -= transform.local_x().normalize();
        }

        if keyboard_input.pressed(KeyCode::KeyA) {
            direction += transform.local_y().normalize();
        }

        if keyboard_input.pressed(KeyCode::KeyD) {
            direction -= transform.local_y().normalize();
        }

        transform.translation += direction * animation_player.speed();
    }
}

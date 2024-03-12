/// In the lifecycle_system.rs file handles systems related to the lifecycle events of the player
/// entity, such as: Player Spawning/Despawning/Respawning, Handling Player Death and Resources.
use std::f32::consts::PI;
use bevy::asset::AssetServer;
use bevy::prelude::*;
use bevy::prelude::KeyCode::*;
use bevy_rapier3d::prelude::{Collider, Restitution};
use bevy_scene_hook::{HookedSceneBundle, SceneHook};
use crate::player::player;
use crate::player::player::KeyboardLayout;

// Initial setup of the scene
pub fn initial_spawn(mut commands: Commands,
                     asset_server: Res<AssetServer>,
) {
    commands.spawn(HookedSceneBundle {
        scene: SceneBundle {
            scene: asset_server.load(format!("{}{}", player::DEFAULT_MODEL, "#Scene0")),
            transform: Transform {
                translation: Vec3::new(4.0, 4.0, 0.0),
                rotation: Quat::from_rotation_x(PI / 2.0),
                ..Default::default()
            },
            ..Default::default()
        },
        hook: SceneHook::new(|entity, cmds| {
            let e = entity.get::<AnimationPlayer>();
            if e.is_some() {
                cmds
                    .insert(Collider::capsule_z(1.0, 0.3))
                    .insert(Restitution::coefficient(0.7))
                    .insert(player::Player{
                    position: Vec3::new(2.0, 2.0, 0.0),
                    view_direction: Vec2::ZERO,
                    // speed: 0.03,
                    speed: 0.02,
                    rotational_speed: 0.1,
                    health: 100.0,
                    name: "Player1".parse().unwrap(),
                    keyboard_layout: KeyboardLayout {
                        forward: KeyW,
                        back: KeyS,
                        left: KeyQ,
                        right: KeyE,
                        turn_left: KeyA,
                        turn_right: KeyD,
                    },
                    ..default()
                });
            }
        }),
    });

    eprintln!("{}", Quat::from_rotation_z(PI / 2.0));
    commands.spawn(HookedSceneBundle {
        scene: SceneBundle {
            scene: asset_server.load(format!("{}{}", player::DEFAULT_MODEL, "#Scene0")),
            transform: Transform {
                translation: Vec3::new(-4.0, -4.0, 0.0),
                // rotation: Quat::from_rotation_x(PI / 2.0),
                rotation: Quat::from_euler(EulerRot::XYZ, PI / 2.0, PI, 0.0),
                ..Default::default()
            },
            ..Default::default()
        },
        hook: SceneHook::new(|entity, cmds| {
            let e = entity.get::<AnimationPlayer>();
            if e.is_some() {
                cmds
                    .insert(Collider::capsule_z(1.0, 0.3))
                    .insert(Restitution::coefficient(0.7))
                    .insert(player::Player{
                    position: Vec3::new(2.0, 2.0, 0.0),
                    view_direction: Vec2::ZERO,
                    // speed: 0.03,
                    speed: 0.02,
                    rotational_speed: 0.1,
                    health: 100.0,
                    name: "Player2".parse().unwrap(),
                    keyboard_layout: KeyboardLayout {
                        forward: KeyI,
                        back: KeyK,
                        left: KeyU,
                        right: KeyO,
                        turn_left: KeyJ,
                        turn_right: KeyL,
                    },
                    ..default()
                });
            }
        }),
    });
}

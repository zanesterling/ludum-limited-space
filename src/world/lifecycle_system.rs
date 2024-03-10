use std::f32::consts::PI;
use bevy::prelude::*;
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy_rapier3d::prelude::*;

/// Used to help identify our ground plane
#[derive(Component)]
pub struct Ground;

pub fn setup_graphics(mut commands: Commands) {
    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 16.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    // Light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, -PI / 4.,1.0)),
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 200.0,
            maximum_distance: 400.0,
            ..default()
        }
            .into(),
        ..default()
    });
}

pub fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn((Ground, Collider::cuboid(100.0, 100.0, 0.1)))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, -2.0)));
}

#[derive(Debug, Default, Resource)]
pub struct WindowSettings {
    pub height: f32,
    pub width: f32,
}

pub fn update_window_settings(query: Query<&Window>, mut settings: ResMut<WindowSettings>) {
    let primary_window = query.single();
    settings.height = primary_window.height();
    settings.width = primary_window.width();
}


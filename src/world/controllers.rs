use bevy::prelude::*;
use bevy::math::Vec3;
use crate::world::lifecycle_system::WindowSettings;

#[derive(Resource, Default)]
pub struct CursorPosition {
    pub normalized: Vec3,
    pub global: Vec3,
}

pub fn update_cursor_position(
    cursor_moved_events: Res<Events<CursorMoved>>,
    mut cursor_position: ResMut<CursorPosition>,
    window_settings: Res<WindowSettings>,
) {
    for event in cursor_moved_events.iter_current_update_events() {
        let cursor_position_world = Vec3::new(
            event.position.x / window_settings.width * 2.0 - 1.0,
            -event.position.y / window_settings.height * 2.0 + 1.0,
            0.0,
        );

        cursor_position.normalized = cursor_position_world;
        // TODO: fix these magic constants
        cursor_position.global = Vec3::new(cursor_position_world.x * 10.0,
                                           cursor_position_world.y * 6.0,
                                           0.0);
    }
}

mod player;
mod world;
mod ui;
use ui::onscreen;
use player::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use world::controllers;

const BACKGROUND_COLOR: Color = Color::GRAY;

fn main() {
    let mut app = App::new();

    app
        .init_resource::<controllers::CursorPosition>()
        .init_resource::<world::lifecycle_system::WindowSettings>()
        .insert_resource(ClearColor(BACKGROUND_COLOR));

    #[cfg(debug_assertions)] // debug/dev builds only
    {
        use bevy::diagnostic::LogDiagnosticsPlugin;
        app
            .add_plugins(LogDiagnosticsPlugin::default())
            .add_systems(Startup, onscreen::setup_onscreen)
            .add_systems(Update, (
                onscreen::fps_text_update_system,
                onscreen::onscreen_debug_showhide,
                onscreen::cursor_global_coordinates_update_system,
                onscreen::cursor_normalized_coordinates_update_system,
            ));
    }
    app
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())

        .add_systems(Startup, (
          world::lifecycle_system::setup_graphics,
          world::lifecycle_system::setup_physics,
          lifecycle_system::initial_spawn,
          rendering_system::initialize,
        ))
        .add_systems(PostStartup, rendering_system::setup)
        .add_systems(Update, (movement_system::keyboard_animation_control,
                              controllers::update_cursor_position,
                              world::lifecycle_system::update_window_settings,
        ))

        .run();
}

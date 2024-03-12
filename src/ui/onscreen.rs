/// UI for debugging
use bevy::diagnostic::DiagnosticsStore;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use crate::world::controllers::CursorPosition;

/// Marker to find the container entity, so we can show/hide the information
#[derive(Component)]
pub struct OnscreenInfo;

/// Toggle the onscreen information when pressing F12
pub fn onscreen_debug_showhide(
    mut query: Query<&mut Visibility, With<OnscreenInfo>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    gizmos: Gizmos,
) {
    let mut vis = query.single_mut();
    if keyboard_input.just_pressed(KeyCode::F12) {
        *vis = match *vis {
            Visibility::Hidden => Visibility::Visible,
            _ => Visibility::Hidden,
        };
    }
    if *vis != Visibility::Hidden {
        show_coordinate_axes(gizmos);
    }
}

pub fn setup_onscreen(mut commands: Commands) {
    let mut style = Style::default();
    style.position_type = PositionType::Absolute;
    let root = commands.spawn((
        OnscreenInfo,
        NodeBundle {
            background_color: BackgroundColor(Color::BLACK.with_a(0.5)),
            // Should be always on top
            z_index: ZIndex::Global(i32::MAX),
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Auto,
                min_width: Val::Percent(40.0),
                min_height: Val::Percent(20.0),
                top: Val::Percent(1.),
                bottom: Val::Auto,
                left: Val::Percent(1.),
                padding: UiRect::all(Val::Px(10.0)),
                ..Default::default()
            },
            ..Default::default()
        }
    )).id();
    let text_fps = commands.spawn((
        FpsText,
        TextBundle {
            text: Text::from_sections([
                TextSection {
                    value: "FPS: ".into(),
                    style: TextStyle {
                        font_size: 16.0,
                        color: Color::WHITE,
                        ..default()
                    }
                },
                TextSection {
                    value: " N/A\n".into(),
                    style: TextStyle {
                        font_size: 16.0,
                        color: Color::WHITE,
                        ..default()
                    }
                },
            ]),
            ..Default::default()
        }
    )).id();
    let text_global_cursor_coordinates = commands.spawn((
        GlobalCursorText,
        TextBundle {
            text: Text::from_sections([
                TextSection {
                    value: "\nWorld coordinates: ".into(),
                    style: TextStyle {
                        font_size: 16.0,
                        color: Color::WHITE,
                        ..default()
                    }
                },
                TextSection {
                    value: " N/A\n".into(),
                    style: TextStyle {
                        font_size: 16.0,
                        color: Color::WHITE,
                        ..default()
                    }
                },
            ]),
            ..Default::default()
        }.with_style(style.clone()),
    )).id();
    let text_normalized_cursor_coordinates = commands.spawn((
        NormalizedCursorText,
        TextBundle {
            text: Text::from_sections([
                TextSection {
                    value: "\n\nNormalized coordinates: ".into(),
                    style: TextStyle {
                        font_size: 16.0,
                        color: Color::WHITE,
                        ..default()
                    }
                },
                TextSection {
                    value: " N/A\n".into(),
                    style: TextStyle {
                        font_size: 16.0,
                        color: Color::WHITE,
                        ..default()
                    }
                },
            ]),
            ..Default::default()
        }.with_style(style.clone()),
    )).id();

    let help_text = commands.spawn((
        TextBundle {
            text: Text::from_sections([
                TextSection {
                    value: "\n\n\nMove forward: W or I\n \
                    Move back: S or K\n \
                    Move left/right: Q/E or I/P\n \
                    Turn left/right: A/D or J/L\n \
                    Press F12 to hide".into(),
                    style: TextStyle {
                        font_size: 16.0,
                        color: Color::WHITE,
                        ..default()
                    }
                },
            ]),
            ..Default::default()
        }.with_style(style.clone()),
    )).id();
    commands.entity(root).push_children(&[
        text_fps,
        text_global_cursor_coordinates,
        text_normalized_cursor_coordinates,
        help_text]);
}

/// FPS
#[derive(Component)]
pub struct FpsText;

pub fn fps_text_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(value) = diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
        {
            text.sections[1].value = format!("{value:>4.0}\n");
            text.sections[1].style.color = if value >= 120.0 {
                Color::rgb(0.0, 1.0, 0.0)
            } else if value >= 60.0 {
                Color::rgb(
                    (1.0 - (value - 60.0) / (120.0 - 60.0)) as f32,
                    1.0,
                    0.0,
                )
            } else if value >= 30.0 {
                Color::rgb(1.0, ((value - 30.0) / (60.0 - 30.0)) as f32, 0.0)
            } else {
                Color::rgb(1.0, 0.0, 0.0)
            }
        } else {
            text.sections[1].value = " N/A\n".into();
            text.sections[1].style.color = Color::WHITE;
        }
    }
}

/// Coordinate Axes
fn show_coordinate_axes(mut gizmos: Gizmos) {
    // X axis
    gizmos.ray(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(2.0, 0.0, 0.0),
        Color::BLUE,
    );
    // Y axis
    gizmos.ray(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        Color::GREEN,
    );
    // Z axis
    gizmos.ray(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 2.0),
        Color::RED,
    );
}

/// Cursor
#[derive(Component)]
pub struct GlobalCursorText;

#[derive(Component)]
pub struct NormalizedCursorText;

pub fn cursor_global_coordinates_update_system(
    cursor_position: Res<CursorPosition>,
    mut query: Query<&mut Text, With<GlobalCursorText>>,
) {
    for mut text in &mut query {
        text.sections[1].value = format!("{}:{}:{}\n",
                                         cursor_position.global.x,
                                         cursor_position.global.y,
                                         cursor_position.global.z);
    }
}

pub fn cursor_normalized_coordinates_update_system(
    cursor_position: Res<CursorPosition>,
    mut query: Query<&mut Text, With<NormalizedCursorText>>,
) {
    for mut text in &mut query {
        text.sections[1].value = format!("{}:{}:{}\n",
                                         cursor_position.normalized.x,
                                         cursor_position.normalized.y,
                                         cursor_position.normalized.z);
    }
}
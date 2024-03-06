use bevy::prelude::*;

const BACKGROUND_COLOR: Color = Color::BLACK;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .insert_resource(ClearColor(BACKGROUND_COLOR))
    .add_systems(Startup, setup)
    .add_systems(FixedUpdate, move_sprite)
    .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands.spawn(Camera2dBundle::default());
  commands.spawn((
    SpriteBundle {
      texture: asset_server.load("bevy_bird_dark.png"),
      transform: Transform::from_xyz(100., 0., 0.),
      ..default()
    },
    Player,
  ));
}

const PLAYER_SPEED: f32 = 500.;

fn move_sprite(
  keyboard_input: Res<ButtonInput<KeyCode>>,
  mut query: Query<&mut Transform, With<Player>>,
  time: Res<Time>,
) {
  let mut player_transform = query.single_mut();
  let mut direction = Vec2::new(0., 0.);

  if keyboard_input.pressed(KeyCode::ArrowLeft) {
    direction.x -= 1.0;
  }
  if keyboard_input.pressed(KeyCode::ArrowRight) {
    direction.x += 1.0;
  }
  if keyboard_input.pressed(KeyCode::ArrowDown) {
    direction.y -= 1.0;
  }
  if keyboard_input.pressed(KeyCode::ArrowUp) {
    direction.y += 1.0;
  }

  if !(direction.x == 0.0 && direction.y == 0.0) {
    let velocity = direction.normalize() * PLAYER_SPEED * time.delta_seconds();
    player_transform.translation.x += velocity.x;
    player_transform.translation.y += velocity.y;
  }
}

#[derive(Component)]
struct Player;

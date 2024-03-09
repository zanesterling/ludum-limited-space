use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

const BACKGROUND_COLOR: Color = Color::BLACK;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugins(RapierDebugRenderPlugin::default())
    .insert_resource(ClearColor(BACKGROUND_COLOR))
    .add_systems(Startup, (setup_graphics, setup_physics))
    .add_systems(FixedUpdate, update_system)
    .run();
}

fn setup_graphics(mut commands: Commands) {
  commands.spawn(Camera3dBundle {
    transform: Transform::from_xyz(0.0, 20.0, 0.0).looking_at(Vec3::ZERO, -Vec3::Z),
    ..Default::default()
  });
}

fn setup_physics(mut commands: Commands) {
  /* Create the ground. */
  commands
    .spawn(Collider::cuboid(100.0, 0.1, 100.0))
    .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.1, 0.0)));

  /* Create some walls for the player to bump into. */
  commands
    .spawn(Collider::cuboid(7.0, 2.0, 0.1))
    .insert(TransformBundle::from(Transform::from_xyz(0.0, -1.0, 1.0)));
  commands
    .spawn(Collider::cuboid(0.1, 2.0, 1.0))
    .insert(TransformBundle::from(Transform::from_xyz(-7.0, -1.0, 0.0)));
  commands
    .spawn(Collider::cuboid(0.1, 2.0, 2.0))
    .insert(TransformBundle::from(Transform::from_xyz(7.0, -1.0, -1.0)));
  commands
    .spawn(Collider::cuboid(6.0, 2.0, 0.1))
    .insert(TransformBundle::from(Transform::from_xyz(-1.0, -1.0, -1.0)));
  commands
    .spawn(Collider::cuboid(7.0, 2.0, 0.1))
    .insert(TransformBundle::from(Transform::from_xyz(0.0, -1.0, -3.0)));

  /* Configure the character controller & collider. */
  commands
    .spawn(RigidBody::KinematicPositionBased)
    .insert(PlayerKeys::PLAYER_1)
    .insert(Collider::ball(0.5))
    .insert(TransformBundle::from(Transform::from_xyz(0.0, 1.0, 0.0)))
    .insert(KinematicCharacterController {
      offset: CharacterLength::Relative(0.05),
      ..default()
    });
  commands
    .spawn(RigidBody::KinematicPositionBased)
    .insert(PlayerKeys::PLAYER_2)
    .insert(Collider::ball(0.5))
    .insert(TransformBundle::from(Transform::from_xyz(2.0, 1.0, 0.0)))
    .insert(KinematicCharacterController {
      offset: CharacterLength::Relative(0.05),
      ..default()
    });
}

const PLAYER_SPEED: f32 = 5.0;
fn update_system(
  keyboard_input: Res<ButtonInput<KeyCode>>,
  mut controllers: Query<(&mut KinematicCharacterController, &PlayerKeys)>,
  time: Res<Time>,
) {
  for (mut controller, keybinds) in controllers.iter_mut() {
    let gravity_drop = Vec3::new(0.0, -3.0, 0.0);
    let mut direction = Vec3::new(0.0, 0.0, 0.0);
    if keyboard_input.pressed(keybinds.left) {
      direction.x -= 1.0;
    }
    if keyboard_input.pressed(keybinds.right) {
      direction.x += 1.0;
    }
    if keyboard_input.pressed(keybinds.up) {
      direction.z -= 1.0;
    }
    if keyboard_input.pressed(keybinds.down) {
      direction.z += 1.0;
    }

    let velocity = direction * PLAYER_SPEED + gravity_drop;
    controller.translation = Some(velocity * time.delta_seconds());
  }
}

#[derive(Component)]
struct PlayerKeys {
  up: KeyCode,
  down: KeyCode,
  left: KeyCode,
  right: KeyCode,
}

impl PlayerKeys {
  const PLAYER_1: PlayerKeys = PlayerKeys {
    up: KeyCode::ArrowUp,
    down: KeyCode::ArrowDown,
    left: KeyCode::ArrowLeft,
    right: KeyCode::ArrowRight,
  };
  const PLAYER_2: PlayerKeys = PlayerKeys {
    up: KeyCode::KeyW,
    down: KeyCode::KeyS,
    left: KeyCode::KeyA,
    right: KeyCode::KeyD,
  };
}

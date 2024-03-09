use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier3d::prelude::*;

const BACKGROUND_COLOR: Color = Color::BLACK;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugins(RapierDebugRenderPlugin::default())
    .insert_resource(ClearColor(BACKGROUND_COLOR))
    .add_systems(Startup, (setup_graphics, setup_physics))
    .add_systems(FixedUpdate, (update_player, point_to_cursor))
    .run();
}

fn setup_graphics(mut commands: Commands) {
  commands
    .spawn(Camera3dBundle {
      transform: Transform::from_xyz(0.0, 20.0, 0.0).looking_at(Vec3::ZERO, -Vec3::Z),
      ..Default::default()
    })
    .insert(GameCamera);
}

fn setup_physics(mut commands: Commands) {
  /* Create the ground. */
  commands
    .spawn(Collider::cuboid(100.0, 0.1, 100.0))
    .insert(GroundPlane)
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
fn update_player(
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

#[derive(Component)]
struct GameCamera;

#[derive(Component)]
struct GroundPlane;

fn point_to_cursor(
  q_window: Query<&Window, With<PrimaryWindow>>,
  q_camera: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
  q_plane: Query<&GlobalTransform, With<GroundPlane>>,
  mut q_player_transform: Query<&mut Transform, With<KinematicCharacterController>>,
) {
  let window = q_window.single();
  let (camera, camera_transform) = q_camera.single();
  let ground_transform = q_plane.single();

  // Get the cursor, but give up if it's outside the window.
  let Some(cursor_position) = window.cursor_position() else {
    return;
  };

  // Get a ray from the camera to the cursor position.
  let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
    // If it was impossible to compute, do nothing.
    return;
  };

  // Intersect the view ray and the ground plane.
  let plane_origin = ground_transform.translation();
  let plane = Plane3d::new(ground_transform.up());
  let Some(distance) = ray.intersect_plane(plane_origin, plane) else {
    // If the ray doesn't intersect the ground (camera is looking away),
    // do nothing.
    return;
  };
  let intersection = ray.get_point(distance);

  for mut transform in q_player_transform.iter_mut() {
    let mut to_mouse = transform.translation - intersection;
    to_mouse.y = 0.0;
    transform.rotation = Quat::from_rotation_arc(-Vec3::Z, to_mouse.normalize());
  }
}

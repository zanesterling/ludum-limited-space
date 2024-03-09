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
    transform: Transform::from_xyz(-3.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ..Default::default()
  });
}

fn setup_physics(mut commands: Commands) {
  /* Create the ground. */
  commands
    .spawn(Collider::cuboid(100.0, 0.1, 100.0))
    .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.1, 0.0)));

  commands
    .spawn(Collider::cuboid(10.0, 2.0, 0.1))
    .insert(TransformBundle::from(Transform::from_xyz(0.0, -1.0, 1.0)));

  /* Configure the character controller & collider. */
  commands
    .spawn(RigidBody::KinematicPositionBased)
    .insert(Collider::ball(0.5))
    .insert(TransformBundle::from(Transform::from_xyz(0.0, 1.0, 0.0)))
    .insert(KinematicCharacterController {
      offset: CharacterLength::Relative(0.01),
      ..default()
    });
}

const PLAYER_SPEED: f32 = 5.0;
fn update_system(
  keyboard_input: Res<ButtonInput<KeyCode>>,
  mut controllers: Query<&mut KinematicCharacterController>,
  time: Res<Time>,
) {
  for mut controller in controllers.iter_mut() {
    let gravity_drop = Vec3::new(0.0, -3.0, 0.0);
    let mut direction = Vec3::new(0.0, 0.0, 0.0);
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
      direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
      direction.x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowUp) {
      direction.z -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
      direction.z += 1.0;
    }

    let velocity = direction * PLAYER_SPEED + gravity_drop;
    controller.translation = Some(velocity * time.delta_seconds());
  }
}

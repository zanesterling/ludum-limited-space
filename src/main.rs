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
    .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));

  /* Create the bouncing ball. */
  commands
    .spawn(RigidBody::Dynamic)
    .insert(Collider::ball(0.5))
    .insert(Restitution::coefficient(0.7))
    .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)));
}

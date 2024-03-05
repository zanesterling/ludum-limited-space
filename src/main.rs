use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
  commands.spawn((Person, Name("Ernie".to_string())));
  commands.spawn((Person, Name("Bert".to_string())));
  commands.spawn((Person, Name("Frank".to_string())));
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
  if timer.0.tick(time.delta()).just_finished() {
    for name in &query {
      println!("hello {}!", name.0);
    }
  }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
  for mut name in &mut query {
    if name.0 == "Frank" {
      name.0 = "Big Bird".to_string();
      break;
    }
  }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
      .add_systems(Startup, add_people)
      .add_systems(Update, (greet_people, update_people).chain());
  }
}

fn main() {
  App::new().add_plugins((DefaultPlugins, HelloPlugin)).run();
}

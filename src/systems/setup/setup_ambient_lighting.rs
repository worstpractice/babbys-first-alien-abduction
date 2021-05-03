use bevy::{
  pbr::LightBundle,
  prelude::{
    Commands,
    Transform,
  },
};

pub fn setup_ambient_lighting(mut commands: Commands) {
  commands.spawn_bundle(LightBundle {
    transform: Transform::from_xyz(4.0, 5.0, 4.0),
    ..Default::default()
  });
}

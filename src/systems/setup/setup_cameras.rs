use crate::{
  constants::{
    BOARD_SIZE_I,
    BOARD_SIZE_J,
    RESET_FOCUS,
  },
  resources::game_camera::GameCamera,
};
use bevy::{
  math::Vec3,
  prelude::{
    Commands,
    PerspectiveCameraBundle,
    ResMut,
    Transform,
    UiCameraBundle,
  },
};

pub fn setup_cameras(mut camera: ResMut<GameCamera>, mut commands: Commands) {
  camera.ideal_offset = Vec3::from(RESET_FOCUS);

  camera.look_at = camera.ideal_offset;

  let x = -(BOARD_SIZE_I as f32 / 2.0);
  let y = 2.0 * BOARD_SIZE_J as f32 / 3.0;
  let z = BOARD_SIZE_J as f32 / 2.0 - 0.5;

  commands.spawn_bundle(PerspectiveCameraBundle {
    transform: Transform::from_xyz(x, y, z).looking_at(camera.look_at, Vec3::Y),
    ..Default::default()
  });

  commands.spawn_bundle(UiCameraBundle::default());
}

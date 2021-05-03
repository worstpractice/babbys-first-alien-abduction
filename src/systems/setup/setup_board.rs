use crate::{
  constants::{
    BOARD_SIZE_I,
    BOARD_SIZE_J,
  },
  resources::game::Game,
};
use bevy::prelude::{
  AssetServer,
  BuildChildren,
  Commands,
  GlobalTransform,
  Res,
  ResMut,
  SpawnSceneAsChildCommands,
  Transform,
};

pub fn setup_board(asset_server: Res<AssetServer>, mut commands: Commands, mut game: ResMut<Game>) {
  let scene = asset_server.load("models/AlienCake/tile.glb#Scene0");

  for j in 0..BOARD_SIZE_J {
    if let Some(inner_vec) = game.board.0.get_mut(j) {
      for i in 0..BOARD_SIZE_I {
        if let Some(tile) = inner_vec.get_mut(i) {
          tile.position.j = j;
          tile.position.i = i;

          tile.height = 0.;

          let x = i as f32;
          let y = tile.height;
          let z = j as f32;

          tile.entity = Some(
            commands
              .spawn_bundle((Transform::from_xyz(x, y, z), GlobalTransform::identity()))
              .with_children(|parent| {
                parent.spawn_scene(scene.clone());
              })
              .id(),
          );
        }
      }
    }
  }
}

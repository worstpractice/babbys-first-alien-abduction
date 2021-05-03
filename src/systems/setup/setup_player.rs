use crate::{
  constants::FACING_NORTH,
  resources::{
    game::Game,
    player::Player,
  },
};
use bevy::{
  math::{
    Quat,
    Vec3,
  },
  prelude::{
    AssetServer,
    BuildChildren,
    Commands,
    GlobalTransform,
    Res,
    ResMut,
    SpawnSceneAsChildCommands,
    Transform,
  },
};

pub fn setup_player(asset_server: Res<AssetServer>, mut commands: Commands, game: ResMut<Game>, mut player: ResMut<Player>) {
  let x = player.position.i as f32;
  let y = game.board.0[player.position.i][player.position.j].height;
  let z = player.position.j as f32;

  // spawn the game character
  player.entity = Some(
    commands
      .spawn_bundle((
        Transform {
          translation: Vec3::new(x, y, z),
          rotation: Quat::from_rotation_y(FACING_NORTH),
          ..Default::default()
        },
        GlobalTransform::identity(),
      ))
      .with_children(|parent| {
        parent.spawn_scene(asset_server.load("models/AlienCake/alien.glb#Scene0"));
      })
      .id(),
  );
}

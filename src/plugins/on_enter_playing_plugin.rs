use crate::{
  enums::game_state::GameState,
  systems::setup::{
    setup_ambient_lighting::setup_ambient_lighting,
    setup_board::setup_board,
    setup_cake::setup_cake,
    setup_cameras::setup_cameras,
    setup_game::setup_game,
    setup_player::setup_player,
    setup_scoreboard::setup_scoreboard,
  },
};
use bevy::prelude::{
  AppBuilder,
  IntoSystem,
  Plugin,
  SystemSet,
};

pub struct OnEnterPlayingPlugin;

impl Plugin for OnEnterPlayingPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app.add_system_set(
      SystemSet::on_enter(GameState::Playing)
        .with_system(setup_game.system())
        .with_system(setup_board.system())
        .with_system(setup_cameras.system())
        .with_system(setup_ambient_lighting.system())
        .with_system(setup_cake.system())
        .with_system(setup_player.system())
        .with_system(setup_scoreboard.system()),
    );
  }
}

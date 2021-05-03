use crate::{
  enums::game_state::GameState,
  systems::game_over::{
    clean_up_scene::clean_up_scene,
    display_score::display_score,
    handle_play_again::handle_play_again,
  },
};
use bevy::prelude::{
  AppBuilder,
  IntoSystem,
  Plugin,
  SystemSet,
};

pub struct OnGameOverPlugin;

impl Plugin for OnGameOverPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(clean_up_scene.system()))
      .add_system_set(SystemSet::on_enter(GameState::GameOver).with_system(display_score.system()))
      .add_system_set(SystemSet::on_update(GameState::GameOver).with_system(handle_play_again.system()))
      .add_system_set(SystemSet::on_exit(GameState::GameOver).with_system(clean_up_scene.system()));
  }
}

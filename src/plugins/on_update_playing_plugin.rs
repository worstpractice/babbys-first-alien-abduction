use crate::{
  enums::game_state::GameState,
  systems::{
    eat_cake::eat_cake,
    end_game_if_player_falls::end_game_if_player_falls,
    focus_camera::focus_camera,
    handle_player_movement::handle_player_movement,
    make_cake_descend_from_sky_like_ufo::make_cake_descend_from_sky_like_ufo,
    make_falling_entities_fall::make_falling_entities_fall,
    mess_with_board::mess_with_board,
    rotate_cake::rotate_cake,
    update_scoreboard::update_scoreboard,
  },
};
use bevy::prelude::{
  AppBuilder,
  IntoSystem,
  Plugin,
  SystemSet,
};

pub struct OnUpdatePlayingPlugin;

impl Plugin for OnUpdatePlayingPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app.add_system_set(
      SystemSet::on_update(GameState::Playing)
        .with_system(handle_player_movement.system())
        .with_system(eat_cake.system())
        .with_system(focus_camera.system())
        .with_system(rotate_cake.system())
        .with_system(update_scoreboard.system())
        .with_system(mess_with_board.system())
        .with_system(make_falling_entities_fall.system())
        .with_system(make_cake_descend_from_sky_like_ufo.system())
        .with_system(end_game_if_player_falls.system()),
    );
  }
}

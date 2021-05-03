use crate::enums::game_state::GameState;
use bevy::{
  input::Input,
  prelude::{
    KeyCode,
    Res,
    ResMut,
    State,
  },
};

// restart the game when pressing spacebar
pub fn handle_play_again(mut state: ResMut<State<GameState>>, keyboard_input: Res<Input<KeyCode>>) {
  if !keyboard_input.just_pressed(KeyCode::Space) {
    return;
  }

  state.set(GameState::Playing).ok();
}

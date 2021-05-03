use crate::{
  components::position::Position,
  constants::{
    FACING_NORTH,
    PLAYER_STARTING_I,
    PLAYER_STARTING_J,
  },
  newtypes::sugar::Sugar,
};
use bevy::prelude::Entity;

pub struct Player {
  pub entity: Option<Entity>,
  pub blood_sugar: Sugar,
  pub position: Position,
  pub rotation: f32,
}

impl Default for Player {
  fn default() -> Self {
    let position = Position {
      i: PLAYER_STARTING_I,
      j: PLAYER_STARTING_J,
    };

    Self {
      entity: None,
      blood_sugar: Sugar::default(),
      position,
      rotation: FACING_NORTH,
    }
  }
}

use crate::{
  components::position::Position,
  enums::tile_state::TileState,
};
use bevy::prelude::Entity;

#[derive(Debug)]
pub struct Tile {
  pub entity: Option<Entity>,
  pub height: f32,
  pub position: Position,
  pub state: TileState,
}

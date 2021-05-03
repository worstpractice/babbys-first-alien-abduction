use crate::{
  components::markers::Falling,
  resources::game::Game,
};
use bevy::prelude::{
  Query,
  ResMut,
  Transform,
  Without,
};
use rand::{
  thread_rng,
  Rng,
};

pub fn mess_with_board(mut game: ResMut<Game>, mut query: Query<&mut Transform, Without<Falling>>) {
  for inner_vec in &mut game.board.0 {
    for tile in inner_vec {
      if let Some(tile_entity) = tile.entity {
        if let Ok(mut tile_transform) = query.get_mut(tile_entity) {
          let random_delta = thread_rng().gen_range(-0.004..0.004);

          tile.height += random_delta;
          tile_transform.translation.y += random_delta;
        }
      }
    }
  }
}

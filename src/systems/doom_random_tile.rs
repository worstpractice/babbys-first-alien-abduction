use crate::{
  components::markers::Falling,
  constants::{
    BOARD_SIZE_I,
    BOARD_SIZE_J,
  },
  enums::tile_state::TileState,
  resources::{
    cake::Cake,
    game::Game,
    player::Player,
  },
};
use bevy::prelude::{
  Commands,
  Res,
  ResMut,
};
use rand::{
  thread_rng,
  Rng,
};

pub fn doom_random_tile(cake: Res<Cake>, mut commands: Commands, mut game: ResMut<Game>, player: Res<Player>) {
  loop {
    let random_i = thread_rng().gen_range(0..BOARD_SIZE_I);
    let random_j = thread_rng().gen_range(0..BOARD_SIZE_J);

    let mut random_game_cell = &mut game.board.0[random_j][random_i];

    // Ensure doomed cell has not already fallen
    let is_already_fallen = random_game_cell.state == TileState::Fallen;

    if is_already_fallen {
      continue;
    }

    // Ensure doomed cell doesn't fall out from right under the player
    let is_directly_below_player = random_i == player.position.i && random_j == player.position.j;

    if is_directly_below_player {
      continue;
    }

    // Ensure doomed cell doesn't fall out from right under the cake
    let is_directly_below_cake = random_i == cake.position.i && random_j == cake.position.j;

    if is_directly_below_cake {
      continue;
    }

    random_game_cell.state = TileState::Fallen;

    if let Some(game_cell_entity) = random_game_cell.entity {
      commands.entity(game_cell_entity).insert(Falling);
    }

    break;
  }
}

use crate::{
  components::{
    position::Position,
    tile::Tile,
  },
  constants::{
    BOARD_SIZE_I,
    BOARD_SIZE_J,
  },
  enums::tile_state::TileState,
};

pub struct Board(pub Vec<Vec<Tile>>);

impl Default for Board {
  fn default() -> Self {
    let mut board = Vec::with_capacity(BOARD_SIZE_J);

    for j in 0..BOARD_SIZE_J {
      let mut inner_board = Vec::with_capacity(BOARD_SIZE_I);

      for i in 0..BOARD_SIZE_I {
        let tile = Tile {
          entity: None,
          height: 0.,
          position: Position {
            i,
            j,
          },
          state: TileState::Normal,
        };

        inner_board.push(tile);
      }

      board.push(inner_board);
    }

    Self(board)
  }
}

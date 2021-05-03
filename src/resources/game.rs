use crate::{
  newtypes::score::Score,
  structs::board::Board,
};

#[derive(Default)]
pub struct Game {
  pub board: Board,
  pub cakes_eaten: Score,
}

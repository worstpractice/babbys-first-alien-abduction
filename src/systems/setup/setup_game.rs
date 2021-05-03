use crate::{
  newtypes::{
    score::Score,
    sugar::Sugar,
  },
  resources::{
    game::Game,
    player::Player,
  },
};
use bevy::prelude::ResMut;

pub fn setup_game(mut game: ResMut<Game>, mut player: ResMut<Player>) {
  game.cakes_eaten = Score::default();
  player.blood_sugar = Sugar::default();
}

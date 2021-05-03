use crate::{
  newtypes::{
    score::Score,
    sugar::Sugar,
  },
  resources::{
    cake::Cake,
    game::Game,
    player::Player,
  },
};
use bevy::prelude::{
  Commands,
  DespawnRecursiveExt,
  ResMut,
};

pub fn eat_cake(mut cake: ResMut<Cake>, mut commands: Commands, mut game: ResMut<Game>, mut player: ResMut<Player>) {
  if let Some(cake_entity) = cake.entity {
    let has_caught_the_cake = player.position.i == cake.position.i && player.position.j == cake.position.j;

    if !has_caught_the_cake {
      return;
    }

    player.blood_sugar += Sugar(5);
    game.cakes_eaten += Score(1);

    commands.entity(cake_entity).despawn_recursive();

    cake.entity = None;
  }
}

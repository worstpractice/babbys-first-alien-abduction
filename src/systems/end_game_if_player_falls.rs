use crate::{
  components::markers::Falling,
  enums::game_state::GameState,
  resources::player::Player,
};
use bevy::prelude::{
  Entity,
  Query,
  Res,
  ResMut,
  State,
  Transform,
  With,
};

pub fn end_game_if_player_falls(player: Res<Player>, mut state: ResMut<State<GameState>>, query: Query<(Entity, &Transform), With<Falling>>) {
  if let Some(player_entity) = player.entity {
    query.for_each(|(entity, transform)| {
      let is_player_falling = player_entity == entity;

      if is_player_falling && transform.translation.y < -200. {
        state.set(GameState::GameOver).ok();
      }
    });
  }
}

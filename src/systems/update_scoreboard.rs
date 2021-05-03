use crate::resources::player::Player;
use bevy::{
  prelude::{
    Query,
    Res,
  },
  text::Text,
};

pub fn update_scoreboard(player: Res<Player>, mut query: Query<&mut Text>) {
  if let Ok(mut text) = query.single_mut() {
    text.sections[0].value = format!("Sugar Rush: {}", player.blood_sugar);
  }
}

use crate::components::markers::Falling;
use bevy::prelude::{
  Query,
  Transform,
  With,
};

pub fn make_falling_entities_fall(query: Query<&mut Transform, With<Falling>>) {
  query.for_each_mut(|mut transform| {
    transform.translation.y -= 0.015;

    transform.translation.y *= 1.005;
  });
}

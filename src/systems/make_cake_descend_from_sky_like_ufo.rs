use crate::resources::cake::Cake;
use bevy::prelude::{
  Query,
  ResMut,
  Transform,
};

pub fn make_cake_descend_from_sky_like_ufo(cake: ResMut<Cake>, mut query: Query<&mut Transform>) {
  if let Some(cake_entity) = cake.entity {
    if let Ok(mut cake_transform) = query.get_mut(cake_entity) {
      // TODO: use a proper easing function
      cake_transform.translation.y -= 0.001;
    }
  }
}

use crate::resources::{
  cake::Cake,
  player::Player,
};
use bevy::{
  core::Time,
  math::{
    Quat,
    Vec3,
  },
  prelude::{
    Query,
    Res,
    Transform,
  },
};

pub fn rotate_cake(cake: Res<Cake>, player: Res<Player>, time: Res<Time>, mut query: Query<&mut Transform>) {
  if let Some(entity) = cake.entity {
    if let Ok(mut cake_transform) = query.get_mut(entity) {
      cake_transform.rotate(Quat::from_rotation_y(time.delta_seconds()));

      cake_transform.scale = Vec3::splat(1.0 + (player.blood_sugar.0 as f32 / 10.0 * time.seconds_since_startup().sin() as f32).abs());
    }
  }
}

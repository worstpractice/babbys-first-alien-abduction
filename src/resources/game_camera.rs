use bevy::math::Vec3;

#[derive(Default)]
pub struct GameCamera {
  pub ideal_offset: Vec3,
  pub look_at: Vec3,
}

use crate::components::position::Position;
use bevy::prelude::{
  Entity,
  Handle,
  Scene,
};

#[derive(Default)]
pub struct Cake {
  pub entity: Option<Entity>,
  pub handle: Handle<Scene>,
  pub position: Position,
}

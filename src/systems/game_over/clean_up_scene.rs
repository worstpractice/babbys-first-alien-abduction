use bevy::{
  prelude::{
    Commands,
    DespawnRecursiveExt,
    Entity,
    Query,
    Without,
  },
  render::camera::Camera,
};

// remove all entities that are not a camera
pub fn clean_up_scene(mut commands: Commands, entities: Query<Entity, Without<Camera>>) {
  entities.for_each(|entity| {
    commands.entity(entity).despawn_recursive();
  });
}

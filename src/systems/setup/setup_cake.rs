use crate::resources::cake::Cake;
use bevy::prelude::{
  AssetServer,
  Res,
  ResMut,
};

pub fn setup_cake(asset_server: Res<AssetServer>, mut cake: ResMut<Cake>) {
  cake.handle = asset_server.load("models/AlienCake/cakeBirthday.glb#Scene0");
}

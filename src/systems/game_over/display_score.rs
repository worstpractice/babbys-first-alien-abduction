use crate::resources::game::Game;
use bevy::{
  math::{
    Rect,
    Size,
  },
  prelude::{
    AssetServer,
    Assets,
    BuildChildren,
    Color,
    Commands,
    NodeBundle,
    Res,
    ResMut,
    TextBundle,
  },
  sprite::ColorMaterial,
  text::{
    Text,
    TextStyle,
  },
  ui::{
    AlignItems,
    FlexDirection,
    JustifyContent,
    Style,
    Val,
  },
};

// display the number of cake eaten before losing
pub fn display_score(mut commands: Commands, asset_server: Res<AssetServer>, game: Res<Game>, mut materials: ResMut<Assets<ColorMaterial>>) {
  commands
    .spawn_bundle(NodeBundle {
      style: Style {
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::ColumnReverse,
        justify_content: JustifyContent::SpaceEvenly,
        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
        margin: Rect::all(Val::Auto),
        ..Default::default()
      },
      material: materials.add(Color::NONE.into()),
      ..Default::default()
    })
    .with_children(|parent| {
      parent.spawn_bundle(TextBundle {
        text: Text::with_section(
          "GAME OVER".to_string(),
          TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 100.0,
            color: Color::rgb(1., 0., 0.),
          },
          Default::default(),
        ),
        ..Default::default()
      });
      parent.spawn_bundle(TextBundle {
        text: Text::with_section(
          format!("Cake eaten: {}", game.cakes_eaten),
          TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 75.0,
            color: Color::rgb(0.5, 0.5, 1.0),
          },
          Default::default(),
        ),
        ..Default::default()
      });
      parent.spawn_bundle(TextBundle {
        text: Text::with_section(
          "Press space to play again".to_string(),
          TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 50.0,
            color: Color::rgb(1., 1., 1.0),
          },
          Default::default(),
        ),
        ..Default::default()
      });
    });
}

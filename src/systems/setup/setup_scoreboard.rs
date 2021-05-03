use bevy::{
  math::Rect,
  prelude::{
    AssetServer,
    Color,
    Commands,
    Res,
    TextBundle,
  },
  text::{
    Text,
    TextStyle,
  },
  ui::{
    PositionType,
    Style,
    Val,
  },
};

pub fn setup_scoreboard(asset_server: Res<AssetServer>, mut commands: Commands) {
  commands.spawn_bundle(TextBundle {
    text: Text::with_section(
      "Score:",
      TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 40.0,
        color: Color::rgb(0.5, 0.5, 1.0),
      },
      Default::default(),
    ),
    style: Style {
      position_type: PositionType::Absolute,
      position: Rect {
        top: Val::Px(5.0),
        left: Val::Px(5.0),
        ..Default::default()
      },
      ..Default::default()
    },
    ..Default::default()
  });
}

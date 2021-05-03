mod components;
mod constants;
mod enums;
mod newtypes;
mod plugins;
mod resources;
mod structs;
mod systems;
mod utils;

use crate::{
  enums::game_state::GameState,
  plugins::{
    on_enter_playing_plugin::OnEnterPlayingPlugin,
    on_game_over_plugin::OnGameOverPlugin,
    on_update_playing_plugin::OnUpdatePlayingPlugin,
  },
  resources::{
    cake::Cake,
    game::Game,
    game_camera::GameCamera,
    player::Player,
  },
  systems::{
    doom_random_tile::doom_random_tile,
    spawn_cake::spawn_cake,
  },
  utils::get_current_screen_resolution,
};
use bevy::{
  core::FixedTimestep,
  diagnostic::{
    FrameTimeDiagnosticsPlugin,
    LogDiagnosticsPlugin,
  },
  ecs::schedule::SystemSet,
  input::system::exit_on_esc_system,
  prelude::{
    App,
    IntoSystem,
  },
  window::{
    WindowDescriptor,
    WindowMode,
  },
  DefaultPlugins,
};
use constants::{
  CLEAR_COLOR,
  MSAA,
};

fn main() {
  let (height, width) = get_current_screen_resolution();

  App::build()
    .insert_resource(WindowDescriptor {
      cursor_visible: false,
      title: "Alien Cake Party!".to_string(),
      width,
      height,
      vsync: false,
      mode: WindowMode::Fullscreen {
        use_size: true,
      },
      ..Default::default()
    })
    .insert_resource(CLEAR_COLOR)
    .insert_resource(MSAA)
    .init_resource::<Cake>()
    .init_resource::<GameCamera>()
    .init_resource::<Game>()
    .init_resource::<Player>()
    .add_plugins(DefaultPlugins)
    .add_plugin(LogDiagnosticsPlugin::default())
    .add_plugin(FrameTimeDiagnosticsPlugin::default())
    // Game states
    .add_plugin(OnEnterPlayingPlugin)
    .add_plugin(OnUpdatePlayingPlugin)
    .add_plugin(OnGameOverPlugin)
    // Game loop
    .add_system_set(
      SystemSet::new()
      .with_run_criteria(FixedTimestep::step(5.))
      .with_system(spawn_cake.system())
    )
    // Doom random tiles
    .add_system_set(
      SystemSet::new()
        .with_run_criteria(FixedTimestep::step(5.))
        .with_system(doom_random_tile.system())
    )
    // Add general system (always active)
    .add_system(exit_on_esc_system.system())
    // Set initial game state
    .add_state(GameState::Playing)
    // Run
    .run();
}

use crate::{
  constants::{
    BOARD_SIZE_I,
    BOARD_SIZE_J,
  },
  enums::{
    game_state::GameState,
    tile_state::TileState,
  },
  newtypes::sugar::Sugar,
  resources::{
    cake::Cake,
    game::Game,
    player::Player,
  },
};
use bevy::{
  math::Vec3,
  pbr::{
    Light,
    LightBundle,
  },
  prelude::{
    BuildChildren,
    Commands,
    DespawnRecursiveExt,
    GlobalTransform,
    Res,
    ResMut,
    SpawnSceneAsChildCommands,
    State,
    Transform,
  },
};
use rand::{
  thread_rng,
  Rng,
};

const ADDED_HEIGHT_FOR_UFO_EFFECT: f32 = 3.;

// despawn the cake if there is one, then spawn a new one at a random location
pub fn spawn_cake(mut cake: ResMut<Cake>, mut commands: Commands, game: Res<Game>, mut player: ResMut<Player>, mut state: ResMut<State<GameState>>) {
  // lose points for having failed to grab the cake in time
  if let Some(cake_entity) = cake.entity {
    player.blood_sugar -= Sugar(3);

    commands.entity(cake_entity).despawn_recursive();

    cake.entity = None;

    if player.blood_sugar <= Sugar(-5) {
      state.set(GameState::GameOver).ok();

      return;
    }
  }

  // TODO: handle this in a `GameState`-specific system (and not using a wonky boolean comparison like now)
  // NOTE: this happens because `spawn_cake` is actually the main loop of the game and so is run outside of the other systems
  // Don't spawn cakes during the game over screen
  if *state.current() == GameState::GameOver {
    return;
  }

  // Determine cake (re)spawn location
  loop {
    let random_i = thread_rng().gen_range(0..BOARD_SIZE_I);
    let random_j = thread_rng().gen_range(0..BOARD_SIZE_J);

    // Ensure cake doesn't (re)spawn on top of the player
    let is_on_top_of_player = random_i == player.position.i && random_j == player.position.j;

    if is_on_top_of_player {
      continue;
    }

    // Ensure cake doesn't (re)spawn on a fallen tile
    let is_on_top_of_fallen_tile = game.board.0[random_j][random_i].state == TileState::Fallen;

    if is_on_top_of_fallen_tile {
      continue;
    }

    cake.position.i = random_i;
    cake.position.j = random_j;
    break;
  }

  let x = cake.position.i as f32;
  let y = game.board.0[player.position.j][player.position.i].height + ADDED_HEIGHT_FOR_UFO_EFFECT;
  let z = cake.position.j as f32;

  cake.entity = Some(
    commands
      .spawn_bundle((
        Transform {
          translation: Vec3::new(x, y, z),
          ..Default::default()
        },
        GlobalTransform::identity(),
      ))
      .with_children(|parent| {
        parent.spawn_scene(cake.handle.clone());
        parent.spawn_bundle(LightBundle {
          light: Light {
            range: 4.,
            ..Default::default()
          },
          ..Default::default()
        });
      })
      .id(),
  );
}

use crate::{
  components::markers::Falling,
  constants::{
    BOARD_SIZE_I,
    BOARD_SIZE_J,
    FACING_EAST,
    FACING_NORTH,
    FACING_SOUTH,
    FACING_WEST,
  },
  enums::tile_state::TileState,
  resources::{
    game::Game,
    player::Player,
  },
};
use bevy::{
  input::Input,
  math::{
    Quat,
    Vec3,
  },
  prelude::{
    Commands,
    KeyCode,
    Query,
    Res,
    ResMut,
    Transform,
    Without,
  },
};

///////////////////////////////////////////////////////////////////////////////////////////////////////

fn move_down(player: &mut ResMut<Player>) {
  player.rotation = FACING_SOUTH;

  if player.position.i > 0 {
    player.position.i -= 1;
  }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////

fn move_left(player: &mut ResMut<Player>) {
  player.rotation = FACING_WEST;

  if player.position.j > 0 {
    player.position.j -= 1;
  }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////

fn move_right(player: &mut ResMut<Player>) {
  player.rotation = FACING_EAST;

  if player.position.j < BOARD_SIZE_J - 1 {
    player.position.j += 1;
  }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////

fn move_up(player: &mut ResMut<Player>) {
  player.rotation = FACING_NORTH;

  if player.position.i < BOARD_SIZE_I - 1 {
    player.position.i += 1;
  }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////

const TINY_HEIGHT_ADJUSTMENT_FOR_ALIEN_MODEL: f32 = 0.2;

///////////////////////////////////////////////////////////////////////////////////////////////////////

type Handler = fn(&mut ResMut<Player>) -> ();

const KEY_HANDLER_PAIRS: [(KeyCode, Handler); 4] = [
  (KeyCode::Down, move_down as Handler),
  (KeyCode::Left, move_left as Handler),
  (KeyCode::Right, move_right as Handler),
  (KeyCode::Up, move_up as Handler),
];

///////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn handle_player_movement(
  mut commands: Commands,
  game: Res<Game>,
  keyboard_input: Res<Input<KeyCode>>,
  mut player: ResMut<Player>,
  mut query: Query<&mut Transform, Without<Falling>>,
) {
  for (key, handler) in &KEY_HANDLER_PAIRS {
    if keyboard_input.just_pressed(*key) {
      handler(&mut player);
    }
  }

  let tile_player_is_standing_on = &game.board.0[player.position.j][player.position.i];

  // Visually move the player as well
  let x = player.position.i as f32;
  let y = tile_player_is_standing_on.height + TINY_HEIGHT_ADJUSTMENT_FOR_ALIEN_MODEL;
  let z = player.position.j as f32;

  if let Some(player_entity) = player.entity {
    if let Ok(mut player_transform) = query.get_mut(player_entity) {
      *player_transform = Transform {
        translation: Vec3::new(x, y, z),
        rotation: Quat::from_rotation_y(player.rotation),
        ..Default::default()
      };
    }

    if tile_player_is_standing_on.state != TileState::Fallen {
      return;
    }

    // Kill player if standing on fallen tile
    commands.entity(player_entity).insert(Falling);
  }
}

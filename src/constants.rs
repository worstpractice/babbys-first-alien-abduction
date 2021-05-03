use bevy::prelude::{
  ClearColor,
  Color,
  Msaa,
};
use std::f32::consts::{
  FRAC_PI_2,
  PI,
};

// Camera
pub const CAMERA_SPEED: f32 = 2.0;
pub const RESET_FOCUS: [f32; 3] = [BOARD_SIZE_I as f32 / 2.0, 0.0, BOARD_SIZE_J as f32 / 2.0 - 0.5];

// Game
pub const BOARD_SIZE_I: usize = 21;
pub const BOARD_SIZE_J: usize = 21;

// On the game board
pub const FACING_NORTH: f32 = -FRAC_PI_2;
pub const FACING_SOUTH: f32 = FRAC_PI_2;
pub const FACING_EAST: f32 = PI;
pub const FACING_WEST: f32 = 0.0;

// PLayer
pub const PLAYER_STARTING_I: usize = BOARD_SIZE_I / 2;
pub const PLAYER_STARTING_J: usize = BOARD_SIZE_J / 2;

// Gfx
pub const CLEAR_COLOR: ClearColor = ClearColor(Color::rgb(0., 0., 0.));
pub const MSAA: Msaa = Msaa {
  samples: 8,
};

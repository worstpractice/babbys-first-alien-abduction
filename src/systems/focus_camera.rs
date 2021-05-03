use crate::{
  constants::{
    CAMERA_SPEED,
    RESET_FOCUS,
  },
  resources::{
    cake::Cake,
    game_camera::GameCamera,
    player::Player,
  },
};
use bevy::{
  core::Time,
  math::Vec3,
  prelude::{
    Query,
    QuerySet,
    Res,
    ResMut,
    Transform,
  },
  render::{
    camera::Camera,
    render_graph::base::camera::CAMERA_3D,
  },
};

fn target_midpoint_between_cake_and_player(cake_transform: &Transform, game_camera: &mut GameCamera, player_transform: &Transform) {
  game_camera.ideal_offset = player_transform.translation.lerp(cake_transform.translation, 0.5);
}

fn target_player(game_camera: &mut GameCamera, player_transform: &Transform) {
  game_camera.ideal_offset = player_transform.translation;
}

fn target_middle_of_board(game_camera: &mut GameCamera) {
  game_camera.ideal_offset = Vec3::from(RESET_FOCUS);
}

/// Calculate the camera motion based on the difference between where the camera is looking and where it should be looking.
///
/// The greater the distance, the faster the motion.
///
/// Smooth out the camera movement using the frame time.
fn calculate_camera_motion(game_camera: &mut GameCamera, time: &Time) {
  let mut camera_motion = game_camera.ideal_offset - game_camera.look_at;

  if camera_motion.length() > 0.2 {
    camera_motion *= CAMERA_SPEED * time.delta_seconds();

    // set the new camera's actual focus
    game_camera.look_at += camera_motion;
  }
}

fn look_at_target(camera: &Camera, game_camera: &mut GameCamera, transform: &mut Transform) {
  if camera.name != Some(CAMERA_3D.to_string()) {
    return;
  }

  *transform = transform.looking_at(game_camera.look_at, Vec3::Y);
}

#[allow(clippy::type_complexity)]
fn decide_camera_target(
  cake: &Cake,
  mut game_camera: &mut GameCamera,
  player: &Player,
  queries: &mut QuerySet<(Query<(&Camera, &mut Transform)>, Query<&Transform>)>,
) {
  // if there is both a player and a cake, target the mid-point of them
  if let (Some(player_entity), Some(cake_entity)) = (player.entity, cake.entity) {
    if let (Ok(player_transform), Ok(cake_transform)) = (queries.q1().get(player_entity), queries.q1().get(cake_entity)) {
      target_midpoint_between_cake_and_player(cake_transform, &mut game_camera, player_transform);
    }

  // otherwise, if there is only a player, target the player
  } else if let Some(player_entity) = player.entity {
    if let Ok(player_transform) = queries.q1().get(player_entity) {
      target_player(&mut game_camera, player_transform);
    }

  // otherwise, target the middle
  } else {
    target_middle_of_board(&mut game_camera);
  }
}

#[allow(clippy::type_complexity)]
pub fn focus_camera(
  cake: Res<Cake>,
  mut game_camera: ResMut<GameCamera>,
  player: Res<Player>,
  time: Res<Time>,
  mut queries: QuerySet<(Query<(&Camera, &mut Transform)>, Query<&Transform>)>,
) {
  decide_camera_target(&cake, &mut game_camera, &player, &mut queries);

  calculate_camera_motion(&mut game_camera, &time);

  // NOTE: the for_each is necessary since there are atleast two cameras (considering the UI camera).
  queries.q0_mut().for_each_mut(|(camera, mut camera_transform)| {
    // look at that new camera's actual focus
    look_at_target(camera, &mut game_camera, &mut camera_transform);
  });
}

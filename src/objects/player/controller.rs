use std::fmt::Debug;

use super::Player;
use crate::{objects::KinematicObject, util::get_real_cursor_position};
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;

/* DEUBG PLAYER CONTROLLER FOR KinematicObject
if key_in.pressed(KeyCode::T) {
    if let Some(cursor_pos) =
        get_real_cursor_position(window, camera_orth_proj, &camera_trf.translation())
    {
        vel.linvel = cursor_pos - Vec2::new(trf.translation().x, trf.translation().y);
    }
}
if key_in.just_pressed(KeyCode::R) {
    vel.linvel = Vec2::ZERO;
}
*/

/// Controls the player!
pub(crate) fn sys_player_controller(
    key_in: Res<Input<KeyCode>>,
    time: Res<Time>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&OrthographicProjection, &GlobalTransform), With<Camera2d>>,
    mut q_players: Query<(&mut KinematicObject, &mut Velocity, &GlobalTransform), With<Player>>,
) {
    let window = q_window.single();
    let (camera_orth_proj, camera_trf) = q_camera.single();

    for (mut _object, mut vel, trf) in q_players.iter_mut() {
        // vel.linvel.y -= 200. * time.delta_seconds();
        // const TERMINAL_VEL: f32 = 200.;
        // if vel.linvel.y < -TERMINAL_VEL {
        //     vel.linvel.y = -TERMINAL_VEL;
        // }
        if key_in.pressed(KeyCode::T) {
            if let Some(cursor_pos) =
                get_real_cursor_position(window, camera_orth_proj, &camera_trf.translation())
            {
                vel.linvel = cursor_pos - Vec2::new(trf.translation().x, trf.translation().y);
            }
        }
        if key_in.just_pressed(KeyCode::R) {
            vel.linvel = Vec2::ZERO;
        }
    }
}

// TODO: remove
pub(crate) fn sys_print_player_trf<Label>() -> impl Fn(Res<Time>, Query<&Transform, With<Player>>) {
    let sys = |time: Res<Time>, q_player: Query<&Transform, With<Player>>| {
        let trf = q_player.single();
        println!(
            "delta_time at stage {} = {}",
            std::any::type_name::<Label>(),
            time.delta_seconds()
        );
    };
    sys
}

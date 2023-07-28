use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI};

use super::Player;
use crate::{
    objects::{KinematicObject, KinematicVelocity},
    util::get_real_cursor_position,
};
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;

fn effective_meters(value: f32, dt: f32) -> f32 {
    value * 10. * dt
}

/// Controls the player!
pub(crate) fn sys_player_controller(
    key_in: Res<Input<KeyCode>>,
    time: Res<Time>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&OrthographicProjection, &GlobalTransform), With<Camera2d>>,
    mut q_players: Query<
        (
            &mut KinematicObject,
            &mut KinematicVelocity,
            &GlobalTransform,
        ),
        With<Player>,
    >,
) {
    let window = q_window.single();
    let (camera_orth_proj, camera_trf) = q_camera.single();

    for (k_object, mut vel, trf) in q_players.iter_mut() {
        vel.linvel.y -= effective_meters(50., time.delta_seconds());

        let (pressed_left, pressed_right) = (
            key_in.pressed(KeyCode::Left),
            key_in.pressed(KeyCode::Right),
        );
        if pressed_left ^ pressed_right {
            if pressed_right {
                vel.linvel.x += effective_meters(200., time.delta_seconds());
                if vel.linvel.x > 50. {
                    vel.linvel.x = 50.;
                }
            }
            if pressed_left {
                vel.linvel.x -= effective_meters(200., time.delta_seconds());
                if vel.linvel.x < -50. {
                    vel.linvel.x = -50.;
                }
            }
        } else {
            let res = effective_meters(
                if k_object.touching_floor() { 200. } else { 50. },
                time.delta_seconds(),
            );
            if vel.linvel.x > res {
                vel.hard_add_assign_x(-res);
            } else if vel.linvel.x < -res {
                vel.hard_add_assign_x(res);
            } else {
                vel.hard_assign_x(0.);
            }
        }
        if k_object.touching_floor() {
            if key_in.just_pressed(KeyCode::Up) {
                vel.hard_assign_y(120.);
            } else {
                vel.linvel.y = -10.;
            }
        }

        // dbg stuff
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

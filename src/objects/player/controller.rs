use super::Player;
use crate::objects::KinematicObject;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

/// Controls the player!
pub(crate) fn sys_player_controller(
    _key_in: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut q_players: Query<(&mut KinematicObject, &mut Velocity), With<Player>>,
) {
    for (mut object, mut vel) in q_players.iter_mut() {
        // vel.linvel.y -= 200. * time.delta_seconds();
        // const TERMINAL_VEL: f32 = 200.;
        // if vel.linvel.y < -TERMINAL_VEL {
        //     vel.linvel.y = -TERMINAL_VEL;
        // }
    }
}

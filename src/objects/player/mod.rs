pub mod animation;
pub mod controller;

use self::{
    animation::{sys_player_animation, sys_setup_player_animations},
    controller::sys_player_controller,
};
use super::{KinematicObject, KinematicVelocity};
use crate::{animation::*, animations};

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

/// The child of PlayerObjectBundle
#[derive(Bundle)]
struct PlayerDecorationBundle {
    sprite: AnimatedSpriteSheetBundle,
}

#[derive(Bundle)]
struct PlayerObjectBundle {
    /// KinematicVelocityBased
    // rigid_body: RigidBody,
    collider: Collider,
    transform: TransformBundle,
    /// Name::new("Player")
    name: Name,
    visibility: VisibilityBundle,
    velocity: KinematicVelocity,
    object: KinematicObject,
    player: Player,
}

#[derive(Component)]
pub struct Player;

impl Player {
    pub fn spawn(commands: &mut Commands, animations: &Animations) {
        let player_decoration = commands
            .spawn(PlayerDecorationBundle {
                sprite: AnimatedSpriteSheetBundle {
                    sprite: SpriteSheetBundle {
                        // TODO: this is a magic number for part of the grassland level
                        transform: Transform::from_xyz(0., 1., 0.),
                        ..default()
                    },
                    animation: animations.get(&animations::PLAYER_IDLE).unwrap().clone(),
                    ..default()
                },
            })
            .id();
        commands
            .spawn(PlayerObjectBundle {
                // rigid_body: RigidBody::KinematicVelocityBased,
                collider: Collider::cuboid(3.5, 7.),
                transform: TransformBundle::from(Transform::from_xyz(49., -8.99, 0.)),
                name: Name::new("Player"),
                visibility: VisibilityBundle::default(),
                velocity: KinematicVelocity::zero(),
                object: KinematicObject::new(),
                player: Player,
            })
            .push_children(&[player_decoration]);
    }
}

pub struct PlayerPlugin;

fn sys_spawn_player(mut commands: Commands, animations: ResMut<Animations>) {
    Player::spawn(&mut commands, &animations);
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                sys_setup_player_animations.before(sys_spawn_player),
                sys_spawn_player,
            ),
        )
        .add_systems(Update, sys_player_controller)
        .add_systems(Update, sys_player_animation.after(sys_player_controller));
    }
}
